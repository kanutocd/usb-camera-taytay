use lunsaran_entregar_mock::{MockBehavior, MockServer};
use lunsaran_taytay::{ArtifactId, ArtifactState, SourceId, config::Config, runtime, spool::Spool};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

fn test_root(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "usb-camera-taytay-{name}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn runtime_config(root: &std::path::Path, base_url: &str) -> Config {
    let token_file = root.join("device-token");
    fs::write(&token_file, "test-token").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&token_file, fs::Permissions::from_mode(0o600)).unwrap();
    }
    Config {
        spool_dir: root.join("spool").to_string_lossy().into_owned(),
        quota_bytes: 1024,
        upload_workers: 1,
        lunsaran_base_url: base_url.to_owned(),
        organization_id: "00000000-0000-0000-0000-000000000000".into(),
        project_id: "00000000-0000-0000-0000-000000000000".into(),
        token_file: token_file.to_string_lossy().into_owned(),
    }
}

#[tokio::test]
async fn published_client_uploads_fixture_and_recovers_remote_offset_after_retry() {
    let server = MockServer::start_with_behavior(MockBehavior {
        advance_offset_then_fail_once: true,
        ..MockBehavior::default()
    })
    .await
    .unwrap();
    let root = test_root("published-client");
    fs::create_dir_all(&root).unwrap();
    let config = runtime_config(&root, server.base_url());
    let bytes = b"camera fixture accepted by the published Entregar client";
    let spool = Spool::open(&config.spool_dir, config.quota_bytes).unwrap();
    let job = spool
        .publish(
            ArtifactId::new("acceptance-fixture"),
            SourceId::new("acceptance-camera"),
            "image/jpeg".into(),
            bytes,
            None,
            serde_json::json!({"fixture": true}),
        )
        .unwrap();
    drop(spool);

    let report = runtime::run(&config).await.unwrap();
    assert_eq!(report.completed, 1);
    assert_eq!(report.failed, 0);
    assert_eq!(server.offset().await, bytes.len() as u64);
    assert!(server.session_requests().await >= 1);
    assert_eq!(
        server
            .observed_project_id()
            .await
            .map(|id| id.to_string())
            .as_deref(),
        Some("00000000-0000-0000-0000-000000000000")
    );
    assert_eq!(
        server.checksum().await,
        job.artifact.checksum_sha256.clone()
    );

    let recovered = Spool::open(&config.spool_dir, config.quota_bytes)
        .unwrap()
        .ledger()
        .get(&job.artifact.id)
        .unwrap();
    assert_eq!(recovered.state, ArtifactState::Completed);
    assert_eq!(recovered.offset, bytes.len() as u64);
    assert_eq!(recovered.attempts, 1);

    server.shutdown();
    fs::remove_dir_all(root).unwrap();
}
