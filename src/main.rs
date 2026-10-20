use lunsaran_taytay::{
    Artifact, SourceId, TaytayError, adapter::SourceAdapter,
    adapter::filesystem::FilesystemAdapter, spool::Spool,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
    thread,
    time::Duration,
};

const USAGE: &str =
    "Usage: usb-camera-taytay <check-config|fixture|run-filesystem> CONFIG | --help | --version";
const MAX_ARTIFACT_BYTES: u64 = 64 * 1024 * 1024;
const PARTIAL_FILE_SUFFIX: &str = ".part";
const OBSERVATION_POLL_COUNT: usize = 3;
const OBSERVATION_POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug, Deserialize)]
struct CaptureSidecar {
    captured_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    input_dir: PathBuf,
    spool_dir: PathBuf,
    archive_dir: PathBuf,
    source_id: String,
    spool_quota_bytes: u64,
}

impl Config {
    fn load(path: &Path) -> Result<Self, String> {
        let text =
            fs::read_to_string(path).map_err(|_| "could not read configuration".to_owned())?;
        let config: Self = toml::from_str(&text).map_err(|_| "invalid configuration".to_owned())?;
        if config.source_id.trim().is_empty() || config.spool_quota_bytes == 0 {
            return Err("source_id and spool_quota_bytes must be non-empty and positive".into());
        }
        for directory in [&config.input_dir, &config.spool_dir, &config.archive_dir] {
            fs::create_dir_all(directory)
                .map_err(|_| "could not create configured directory".to_owned())?;
            if !directory.is_dir() {
                return Err("configured path is not a directory".into());
            }
        }
        let directories = [
            config.input_dir.canonicalize(),
            config.spool_dir.canonicalize(),
            config.archive_dir.canonicalize(),
        ];
        let directories: Vec<_> = directories
            .into_iter()
            .map(|path| path.map_err(|_| "could not resolve configured directory".to_owned()))
            .collect::<Result<_, _>>()?;
        for (index, first) in directories.iter().enumerate() {
            for second in &directories[index + 1..] {
                if first.starts_with(second) || second.starts_with(first) {
                    return Err("configured directories must not overlap".into());
                }
            }
        }
        Ok(config)
    }
}

#[cfg(not(coverage))]
fn main() -> ExitCode {
    run(env::args().skip(1))
}

fn run(mut arguments: impl Iterator<Item = String>) -> ExitCode {
    match arguments.next().as_deref() {
        Some("--help" | "-h") if arguments.next().is_none() => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some("--version" | "-V") if arguments.next().is_none() => {
            println!("usb-camera-taytay {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some(command @ ("check-config" | "fixture" | "run-filesystem")) => {
            let Some(path) = arguments.next() else {
                eprintln!("configuration path is required\n{USAGE}");
                return ExitCode::from(2);
            };
            if arguments.next().is_some() {
                eprintln!("unexpected argument\n{USAGE}");
                return ExitCode::from(2);
            }
            match execute(command, Path::new(&path)) {
                Ok(message) => {
                    println!("{message}");
                    ExitCode::SUCCESS
                }
                Err(message) => {
                    eprintln!("{message}");
                    ExitCode::FAILURE
                }
            }
        }
        None => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
        Some(_) => {
            eprintln!("unknown command or invalid arguments\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn execute(command: &str, config_path: &Path) -> Result<String, String> {
    let config = Config::load(config_path)?;
    match command {
        "check-config" => Ok("configuration is valid".into()),
        "fixture" => write_fixture(&config),
        "run-filesystem" => ingest_stable_files(&config),
        _ => unreachable!(),
    }
}

fn write_fixture(config: &Config) -> Result<String, String> {
    const JPEG: &[u8] = &[
        0xff, 0xd8, 0xff, 0xdb, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07,
        0x07, 0x07, 0x09, 0x09, 0x08, 0x0a, 0x0c, 0x14, 0x0d, 0x0c, 0x0b, 0x0b, 0x0c, 0x19, 0x12,
        0x13, 0x0f, 0x14, 0x1d, 0x1a, 0x1f, 0x1e, 0x1d, 0x1a, 0x1c, 0x1c, 0x20, 0x24, 0x2e, 0x27,
        0x20, 0x22, 0x2c, 0x23, 0x1c, 0x1c, 0x28, 0x37, 0x29, 0x2c, 0x30, 0x31, 0x34, 0x34, 0x34,
        0x1f, 0x27, 0x39, 0x3d, 0x38, 0x32, 0x3c, 0x2e, 0x33, 0x34, 0x32, 0xff, 0xc0, 0x00, 0x0b,
        0x08, 0x00, 0x01, 0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0xff, 0xc4, 0x00, 0x14, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xda, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3f, 0x00, 0x00, 0xff, 0xd9,
    ];
    let target = config.input_dir.join("fixture.jpg");
    if target.exists() {
        return Err("fixture.jpg already exists".into());
    }
    let temporary = config.input_dir.join("fixture.jpg.part");
    fs::write(&temporary, JPEG).map_err(|_| "could not write fixture".to_owned())?;
    fs::rename(temporary, target).map_err(|_| "could not publish fixture".to_owned())?;
    Ok("published fixture.jpg".into())
}

fn ingest_stable_files(config: &Config) -> Result<String, String> {
    let mut adapter =
        FilesystemAdapter::new(SourceId::new(config.source_id.clone()), &config.input_dir);
    let mut artifacts = poll_completed_files(&mut adapter, || {
        thread::sleep(OBSERVATION_POLL_INTERVAL);
    })
    .map_err(|_| "filesystem scan failed".to_owned())?;
    artifacts.retain(|artifact| supported_media_type(&artifact.media_type));
    for artifact in &mut artifacts {
        enrich_capture_metadata(artifact)?;
    }
    let spool = Spool::open(&config.spool_dir, config.spool_quota_bytes)
        .map_err(|_| "could not open durable spool".to_owned())?;
    let mut published = 0;
    let mut duplicates = 0;
    for artifact in artifacts {
        let source_path = PathBuf::from(&artifact.path);
        let file =
            fs::File::open(&source_path).map_err(|_| "stable input is unreadable".to_owned())?;
        let mut bytes = Vec::new();
        file.take(MAX_ARTIFACT_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| "stable input is unreadable".to_owned())?;
        if bytes.len() as u64 > MAX_ARTIFACT_BYTES {
            return Err("input exceeds the 64 MiB MVP1 artifact limit".into());
        }
        if !matches_observed_checksum(artifact.checksum_sha256.as_deref(), &bytes) {
            return Err("stable input changed after checksum observation".into());
        }
        if let Some(existing) = spool.ledger().get(&artifact.id) {
            if existing.artifact.checksum_sha256 != artifact.checksum_sha256 {
                return Err("artifact ID conflicts with a different durable checksum".into());
            }
            spool
                .verify(&existing)
                .map_err(|_| "existing duplicate checksum verification failed".to_owned())?;
            duplicates += 1;
            continue;
        }
        let job = spool
            .publish(
                artifact.id,
                artifact.source,
                artifact.media_type,
                &bytes,
                artifact.captured_at,
                artifact.metadata,
            )
            .map_err(|_| "durable publication failed".to_owned())?;
        spool
            .verify(&job)
            .map_err(|_| "published checksum verification failed".to_owned())?;
        published += 1;
    }
    Ok(format!(
        "published {published} stable artifact(s); skipped {duplicates} verified duplicate(s); source files retained"
    ))
}

fn matches_observed_checksum(expected: Option<&str>, bytes: &[u8]) -> bool {
    let Some(expected) = expected else {
        return false;
    };
    let actual = Sha256::digest(bytes);
    expected.eq_ignore_ascii_case(&format!("{actual:x}"))
}

fn poll_completed_files(
    adapter: &mut FilesystemAdapter,
    mut wait_between_polls: impl FnMut(),
) -> Result<Vec<Artifact>, TaytayError> {
    let mut artifacts = Vec::new();
    for poll in 0..OBSERVATION_POLL_COUNT {
        artifacts.extend(
            adapter
                .poll()?
                .into_iter()
                .filter(|artifact| !is_partial_file(&artifact.path)),
        );
        if poll + 1 < OBSERVATION_POLL_COUNT {
            wait_between_polls();
        }
    }
    Ok(artifacts)
}

fn is_partial_file(path: &str) -> bool {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.to_ascii_lowercase().ends_with(PARTIAL_FILE_SUFFIX))
}

fn supported_media_type(media_type: &str) -> bool {
    matches!(media_type, "image/jpeg" | "video/mp4")
}

fn enrich_capture_metadata(artifact: &mut Artifact) -> Result<(), String> {
    let path = Path::new(&artifact.path);
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "input filename is not valid UTF-8".to_owned())?;
    let sidecar_path = path.with_extension("json");
    let captured_at = if sidecar_path.exists() {
        let bytes =
            fs::read(&sidecar_path).map_err(|_| "could not read capture sidecar".to_owned())?;
        let sidecar: CaptureSidecar = serde_json::from_slice(&bytes)
            .map_err(|_| "capture sidecar must be valid JSON".to_owned())?;
        match sidecar.captured_at {
            Some(value) if valid_utc_timestamp(&value) => Some(value),
            Some(_) => {
                return Err("capture sidecar captured_at must use YYYY-MM-DDTHH:MM:SSZ".into());
            }
            None => filename_capture_time(filename),
        }
    } else {
        filename_capture_time(filename)
    };
    artifact.captured_at = captured_at;
    artifact.metadata = serde_json::json!({"filename": filename});
    Ok(())
}

fn filename_capture_time(filename: &str) -> Option<String> {
    let stem = Path::new(filename).file_stem()?.to_str()?;
    let suffix = stem.rsplit_once('_')?.1;
    let bytes = suffix.as_bytes();
    if bytes.len() != 16
        || !bytes[..8].iter().all(u8::is_ascii_digit)
        || bytes[8] != b'T'
        || !bytes[9..15].iter().all(u8::is_ascii_digit)
        || bytes[15] != b'Z'
    {
        return None;
    }
    let timestamp = format!(
        "{}-{}-{}T{}:{}:{}Z",
        &suffix[0..4],
        &suffix[4..6],
        &suffix[6..8],
        &suffix[9..11],
        &suffix[11..13],
        &suffix[13..15]
    );
    valid_utc_timestamp(&timestamp).then_some(timestamp)
}

fn valid_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return false;
    }
    let number = |start: usize, end: usize| value[start..end].parse::<u32>().ok();
    let (Some(year), Some(month), Some(day), Some(hour), Some(minute), Some(second)) = (
        number(0, 4),
        number(5, 7),
        number(8, 10),
        number(11, 13),
        number(14, 16),
        number(17, 19),
    ) else {
        return false;
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day) && hour < 24 && minute < 60 && second < 60
}

#[cfg(test)]
mod tests {
    use super::{
        ExitCode, USAGE, enrich_capture_metadata, execute, filename_capture_time,
        poll_completed_files, run, supported_media_type, valid_utc_timestamp,
    };
    use lunsaran_taytay::{Artifact, ArtifactId, SourceId, adapter::filesystem::FilesystemAdapter};
    use sha2::{Digest, Sha256};
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn usage_names_the_binary_and_supported_options() {
        assert!(USAGE.starts_with("Usage: usb-camera-taytay"));
        assert!(USAGE.contains("--help"));
        assert!(USAGE.contains("--version"));
        assert!(USAGE.contains("run-filesystem"));
    }

    #[test]
    fn recognizes_the_supported_reference_media_types() {
        assert!(supported_media_type("image/jpeg"));
        assert!(supported_media_type("video/mp4"));
        assert!(!supported_media_type("application/octet-stream"));
        assert!(!supported_media_type("image/png"));
    }

    #[test]
    fn extracts_only_valid_utc_capture_times_from_filename_suffixes() {
        assert_eq!(
            filename_capture_time("front-door_20241021T123456Z.mp4").as_deref(),
            Some("2024-10-21T12:34:56Z")
        );
        assert_eq!(filename_capture_time("snapshot.jpg"), None);
        assert_eq!(filename_capture_time("camera_20240230T123456Z.jpg"), None);
    }

    #[test]
    fn validates_utc_capture_time_calendar_and_clock_fields() {
        assert!(valid_utc_timestamp("2024-02-29T23:59:59Z"));
        assert!(!valid_utc_timestamp("2023-02-29T23:59:59Z"));
        assert!(!valid_utc_timestamp("2024-01-01T24:00:00Z"));
        assert!(!valid_utc_timestamp("2024-01-01T00:00:00+00:00"));
    }

    #[test]
    fn sidecar_capture_time_overrides_filename_and_invalid_values_are_rejected() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-sidecar-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let path = root.join("front-door_20241021T123456Z.jpg");
        fs::write(&path, b"image bytes").unwrap();
        let mut artifact = Artifact {
            id: ArtifactId::new("id"),
            source: SourceId::new("camera-1"),
            path: path.to_string_lossy().into_owned(),
            media_type: "image/jpeg".into(),
            captured_at: None,
            metadata: serde_json::json!({}),
            size: 11,
            checksum_sha256: None,
        };

        fs::write(
            root.join("front-door_20241021T123456Z.json"),
            r#"{"captured_at":"2024-10-22T01:02:03Z"}"#,
        )
        .unwrap();
        enrich_capture_metadata(&mut artifact).unwrap();
        assert_eq!(
            artifact.captured_at.as_deref(),
            Some("2024-10-22T01:02:03Z")
        );
        assert_eq!(
            artifact.metadata["filename"],
            "front-door_20241021T123456Z.jpg"
        );

        fs::write(
            root.join("front-door_20241021T123456Z.json"),
            r#"{"captured_at":"not-a-timestamp"}"#,
        )
        .unwrap();
        assert!(enrich_capture_metadata(&mut artifact).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn help_and_version_options_succeed() {
        assert_eq!(run(["--help".to_owned()].into_iter()), ExitCode::SUCCESS);
        assert_eq!(run(["--version".to_owned()].into_iter()), ExitCode::SUCCESS);
        assert_eq!(run(["-h".to_owned()].into_iter()), ExitCode::SUCCESS);
        assert_eq!(run(["-V".to_owned()].into_iter()), ExitCode::SUCCESS);
    }

    #[test]
    fn missing_or_unknown_arguments_fail() {
        assert_eq!(run(std::iter::empty()), ExitCode::from(2));
        assert_eq!(run(["capture".to_owned()].into_iter()), ExitCode::from(2));
        assert_eq!(
            run(["--help".to_owned(), "unexpected".to_owned()].into_iter()),
            ExitCode::from(2)
        );
        assert_eq!(
            run([
                "check-config".to_owned(),
                "config.toml".to_owned(),
                "unexpected".to_owned()
            ]
            .into_iter()),
            ExitCode::from(2)
        );
        assert_eq!(
            run(["check-config".to_owned()].into_iter()),
            ExitCode::from(2)
        );
    }

    #[test]
    fn fixture_and_filesystem_ingestion_publish_a_verified_durable_job() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(root.join("input")).unwrap();
        fs::write(root.join("input/pending.jpg.part"), b"incomplete").unwrap();
        let config_path = root.join("config.toml");
        fs::write(
            &config_path,
            format!(
                "input_dir = {:?}\nspool_dir = {:?}\narchive_dir = {:?}\nsource_id = \"camera-1\"\nspool_quota_bytes = 1048576\n",
                root.join("input").display().to_string(),
                root.join("spool").display().to_string(),
                root.join("archive").display().to_string(),
            ),
        )
        .unwrap();

        assert_eq!(
            execute("check-config", &config_path).unwrap(),
            "configuration is valid"
        );
        assert_eq!(
            execute("fixture", &config_path).unwrap(),
            "published fixture.jpg"
        );
        assert!(
            execute("run-filesystem", &config_path)
                .unwrap()
                .contains("published 1")
        );
        assert!(root.join("spool/ledger.jsonl").is_file());
        assert_eq!(fs::read_dir(root.join("spool")).unwrap().count(), 2);
        assert!(root.join("input/fixture.jpg").is_file());
        assert!(root.join("input/pending.jpg.part").is_file());
        assert!(root.join("archive").is_dir());
        assert_eq!(
            execute("run-filesystem", &config_path).unwrap(),
            "published 0 stable artifact(s); skipped 1 verified duplicate(s); source files retained"
        );
        assert_eq!(fs::read_dir(root.join("spool")).unwrap().count(), 2);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_input_bytes_that_differ_from_the_observed_checksum() {
        let observed = Sha256::digest(b"observed bytes");
        let expected = format!("{observed:x}");

        assert!(super::matches_observed_checksum(
            Some(&expected),
            b"observed bytes"
        ));
        assert!(!super::matches_observed_checksum(
            Some(&expected),
            b"changed bytes"
        ));
        assert!(!super::matches_observed_checksum(None, b"observed bytes"));
    }

    #[test]
    fn atomic_rename_and_repeated_stability_emit_only_the_completed_file() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-rename-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let partial = root.join("capture.jpg.part");
        let complete = root.join("capture.jpg");
        fs::write(&partial, b"complete image bytes").unwrap();
        let mut adapter = FilesystemAdapter::new(SourceId::new("camera-1"), &root);
        let mut waits = 0;

        let artifacts = poll_completed_files(&mut adapter, || {
            waits += 1;
            if waits == 1 {
                fs::rename(&partial, &complete).unwrap();
            }
        })
        .unwrap();

        assert_eq!(waits, 2);
        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].path, complete.to_string_lossy());
        assert!(complete.is_file());
        assert!(!partial.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn a_file_changed_between_observations_is_emitted_after_it_stabilizes() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-stable-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let input = root.join("capture.jpg");
        fs::write(&input, b"partial").unwrap();
        let mut adapter = FilesystemAdapter::new(SourceId::new("camera-1"), &root);
        let mut waits = 0;

        let artifacts = poll_completed_files(&mut adapter, || {
            waits += 1;
            if waits == 1 {
                fs::write(&input, b"completed image bytes").unwrap();
            }
        })
        .unwrap();

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].size, b"completed image bytes".len() as u64);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_missing_malformed_and_overlapping_directory_configurations() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-invalid-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let config_path = root.join("config.toml");
        assert!(execute("check-config", &config_path).is_err());

        fs::write(&config_path, "input_dir = [\n").unwrap();
        assert!(execute("check-config", &config_path).is_err());

        fs::write(
            &config_path,
            format!(
                "input_dir = {:?}\nspool_dir = {:?}\narchive_dir = {:?}\nsource_id = \"camera-1\"\nspool_quota_bytes = 1\n",
                root.display().to_string(),
                root.join("spool").display().to_string(),
                root.join("archive").display().to_string(),
            ),
        )
        .unwrap();
        assert!(execute("check-config", &config_path).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_empty_source_and_zero_spool_quota() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-fields-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let config_path = root.join("config.toml");
        for (source, quota) in [("", 1), ("camera-1", 0)] {
            fs::write(
                &config_path,
                format!(
                    "input_dir = {:?}\nspool_dir = {:?}\narchive_dir = {:?}\nsource_id = {:?}\nspool_quota_bytes = {quota}\n",
                    root.join("input").display().to_string(),
                    root.join("spool").display().to_string(),
                    root.join("archive").display().to_string(),
                    source,
                ),
            )
            .unwrap();
            assert!(execute("check-config", &config_path).is_err());
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reports_duplicate_fixture_and_spool_quota_errors() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-quota-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let config_path = root.join("config.toml");
        fs::write(
            &config_path,
            format!(
                "input_dir = {:?}\nspool_dir = {:?}\narchive_dir = {:?}\nsource_id = \"camera-1\"\nspool_quota_bytes = 1\n",
                root.join("input").display().to_string(),
                root.join("spool").display().to_string(),
                root.join("archive").display().to_string(),
            ),
        )
        .unwrap();
        execute("fixture", &config_path).unwrap();
        assert!(execute("fixture", &config_path).is_err());
        assert!(
            execute("run-filesystem", &config_path)
                .unwrap_err()
                .contains("durable publication failed")
        );
        fs::remove_dir_all(root).unwrap();
    }
}
