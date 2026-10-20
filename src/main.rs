use lunsaran_taytay::{
    SourceId, adapter::SourceAdapter, adapter::filesystem::FilesystemAdapter, spool::Spool,
};
use serde::Deserialize;
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
    adapter
        .poll()
        .map_err(|_| "filesystem scan failed".to_owned())?;
    thread::sleep(Duration::from_millis(1100));
    let artifacts = adapter
        .poll()
        .map_err(|_| "filesystem scan failed".to_owned())?;
    let spool = Spool::open(&config.spool_dir, config.spool_quota_bytes)
        .map_err(|_| "could not open durable spool".to_owned())?;
    let mut published = 0;
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
        "published {published} stable artifact(s); source files retained"
    ))
}

#[cfg(test)]
mod tests {
    use super::{ExitCode, USAGE, execute, run};
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
        assert!(root.join("archive").is_dir());

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
