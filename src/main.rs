#[cfg(target_os = "linux")]
mod v4l2;

use lunsaran_taytay::{
    Artifact, ArtifactState, SourceId, TaytayError, adapter::SourceAdapter,
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

const USAGE: &str = "Usage: usb-camera-taytay <check-config|fixture|run-filesystem|run-v4l2|status> CONFIG | <pause|resume> ARTIFACT_ID CONFIG | --help | --version";
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
    v4l2_device: Option<PathBuf>,
    v4l2_width: Option<u32>,
    v4l2_height: Option<u32>,
    v4l2_pixel_format: Option<String>,
    v4l2_segment_frames: Option<usize>,
    v4l2_segment_max_bytes: Option<u64>,
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
        Some(command @ ("check-config" | "fixture" | "run-filesystem" | "run-v4l2" | "status")) => {
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
        Some(command @ ("pause" | "resume")) => {
            let (Some(id), Some(path)) = (arguments.next(), arguments.next()) else {
                eprintln!("artifact ID and configuration path are required\n{USAGE}");
                return ExitCode::from(2);
            };
            if arguments.next().is_some() {
                eprintln!("unexpected argument\n{USAGE}");
                return ExitCode::from(2);
            }
            match manage_job(command, &id, Path::new(&path)) {
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
        "run-v4l2" => capture_v4l2_segment(&config),
        "status" => show_status(&config),
        _ => unreachable!(),
    }
}

#[cfg(target_os = "linux")]
fn capture_v4l2_segment(config: &Config) -> Result<String, String> {
    let (Some(device), Some(width), Some(height), Some(pixel_format)) = (
        config.v4l2_device.as_deref(),
        config.v4l2_width,
        config.v4l2_height,
        config.v4l2_pixel_format.as_deref(),
    ) else {
        return Err(
            "V4L2 device, width, height, and four-character pixel format are required".into(),
        );
    };
    let bytes = pixel_format.as_bytes();
    if bytes.len() != 4 || !bytes.is_ascii() {
        return Err("V4L2 pixel format must contain four ASCII characters".into());
    }
    if pixel_format != "MJPG" {
        return Err("run-v4l2 currently writes complete MJPEG frames only".into());
    }
    let frame_count = config.v4l2_segment_frames.unwrap_or(30);
    let max_bytes = config.v4l2_segment_max_bytes.unwrap_or(MAX_ARTIFACT_BYTES);
    if frame_count == 0 || frame_count > 300 || max_bytes == 0 || max_bytes > MAX_ARTIFACT_BYTES {
        return Err("V4L2 segment limits must be 1-300 frames and at most 64 MiB".into());
    }
    let fourcc = u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let mut capture = v4l2::Capture::open(device, width, height, fourcc)
        .map_err(|_| "could not initialize V4L2 capture; verify device and mode".to_owned())?;
    write_v4l2_frames(&config.input_dir, frame_count, max_bytes, || {
        capture
            .next_frame()
            .map_err(|_| "could not read a complete V4L2 frame".to_owned())
    })
}

fn write_v4l2_frames(
    directory: &Path,
    frame_count: usize,
    max_bytes: u64,
    mut next_frame: impl FnMut() -> Result<Vec<u8>, String>,
) -> Result<String, String> {
    let mut total_bytes = 0_u64;
    let mut written = 0;
    for _ in 0..frame_count {
        let frame = next_frame()?;
        let frame_size = u64::try_from(frame.len()).map_err(|_| "invalid V4L2 frame size")?;
        total_bytes = total_bytes
            .checked_add(frame_size)
            .filter(|size| *size <= max_bytes)
            .ok_or_else(|| "V4L2 segment reached its configured size limit".to_owned())?;
        let captured_at = utc_timestamp(std::time::SystemTime::now())?;
        write_jpeg_frame(directory, &frame, &captured_at)?;
        written += 1;
    }
    Ok(format!(
        "captured {written} timestamped V4L2 JPEG frames ({total_bytes} bytes)"
    ))
}

#[cfg(not(target_os = "linux"))]
fn capture_v4l2_segment(_config: &Config) -> Result<String, String> {
    Err("V4L2 capture is supported only on Linux".into())
}

fn write_jpeg_frame(directory: &Path, frame: &[u8], captured_at: &str) -> Result<(), String> {
    if frame.len() < 4 || !frame.starts_with(&[0xff, 0xd8]) || !frame.ends_with(&[0xff, 0xd9]) {
        return Err("V4L2 returned an incomplete JPEG frame".into());
    }
    if !valid_utc_timestamp(captured_at) {
        return Err("invalid V4L2 capture timestamp".into());
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| "system clock is before the Unix epoch".to_owned())?
        .as_nanos();
    let stem = format!("v4l2_{}_{}", captured_at.replace(['-', ':'], ""), nonce);
    let image_path = directory.join(format!("{stem}.jpg"));
    let image_partial = directory.join(format!("{stem}.jpg.part"));
    let sidecar_path = directory.join(format!("{stem}.json"));
    let sidecar_partial = directory.join(format!("{stem}.json.part"));

    write_synced_file(
        &sidecar_partial,
        format!("{{\"captured_at\":\"{captured_at}\"}}\n").as_bytes(),
    )?;
    fs::rename(&sidecar_partial, &sidecar_path)
        .map_err(|_| "could not publish V4L2 capture metadata".to_owned())?;
    write_synced_file(&image_partial, frame)?;
    fs::rename(&image_partial, &image_path)
        .map_err(|_| "could not publish V4L2 JPEG frame".to_owned())?;
    Ok(())
}

fn write_synced_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| "could not create V4L2 capture file".to_owned())?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|_| "could not write V4L2 capture file".to_owned())
}

fn utc_timestamp(time: std::time::SystemTime) -> Result<String, String> {
    let seconds = time
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| "system clock is before the Unix epoch".to_owned())?
        .as_secs();
    let seconds = libc::time_t::try_from(seconds)
        .map_err(|_| "system clock timestamp is out of range".to_owned())?;
    let mut broken_down = std::mem::MaybeUninit::<libc::tm>::uninit();
    // SAFETY: gmtime_r initializes the supplied tm and does not retain either pointer.
    if unsafe { libc::gmtime_r(&seconds, broken_down.as_mut_ptr()) }.is_null() {
        return Err("could not format V4L2 capture timestamp".into());
    }
    // SAFETY: gmtime_r returned a non-null pointer to the initialized value.
    let tm = unsafe { broken_down.assume_init() };
    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        tm.tm_year + 1900,
        tm.tm_mon + 1,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec
    ))
}

fn open_spool(config: &Config) -> Result<Spool, String> {
    Spool::open(&config.spool_dir, config.spool_quota_bytes)
        .map_err(|_| "could not open durable spool".to_owned())
}

fn show_status(config: &Config) -> Result<String, String> {
    let spool = open_spool(config)?;
    let ledger = spool.ledger();
    let mut jobs = ledger.pending();
    jobs.extend(ledger.paused());
    jobs.extend(ledger.completed());
    jobs.sort_by(|left, right| left.artifact.id.0.cmp(&right.artifact.id.0));
    if jobs.is_empty() {
        return Ok("no artifacts".into());
    }
    Ok(jobs
        .iter()
        .map(|job| format!("{} {:?}", job.artifact.id.0, job.state))
        .collect::<Vec<_>>()
        .join("\n"))
}

fn manage_job(command: &str, id: &str, config_path: &Path) -> Result<String, String> {
    let config = Config::load(config_path)?;
    let spool = open_spool(&config)?;
    let artifact_id = lunsaran_taytay::ArtifactId::new(id);
    let job = match command {
        "pause" => spool.pause(&artifact_id),
        "resume" => spool.resume(&artifact_id),
        _ => unreachable!(),
    }
    .map_err(|_| format!("could not {command} artifact; check its ID and current state"))?;
    Ok(format!("artifact {} is {:?}", job.artifact.id.0, job.state))
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
    const MP4: &[u8] = &[
        0x00, 0x00, 0x00, 0x18, b'f', b't', b'y', b'p', b'i', b's', b'o', b'm', 0x00, 0x00, 0x02,
        0x00, b'i', b's', b'o', b'm', b'm', b'p', b'4', b'2', 0x00, 0x00, 0x00, 0x08, b'm', b'd',
        b'a', b't',
    ];
    for name in ["fixture.jpg", "fixture.mp4"] {
        if config.input_dir.join(name).exists() {
            return Err(format!("{name} already exists"));
        }
    }
    for (name, bytes) in [("fixture.jpg", JPEG), ("fixture.mp4", MP4)] {
        let target = config.input_dir.join(name);
        let temporary = config.input_dir.join(format!("{name}.part"));
        fs::write(&temporary, bytes).map_err(|_| "could not write fixture".to_owned())?;
        fs::rename(temporary, target).map_err(|_| "could not publish fixture".to_owned())?;
    }
    Ok("published fixture.jpg and fixture.mp4".into())
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
    let mut archived = 0;
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
            duplicates += 1;
            if matches!(
                existing.state,
                ArtifactState::Completed | ArtifactState::Retained
            ) {
                archive_completed_source(&source_path, &config.archive_dir, &bytes)?;
                let sidecar = source_path.with_extension("json");
                if sidecar.exists() {
                    archive_copy(
                        &sidecar,
                        &config.archive_dir.join(
                            sidecar
                                .file_name()
                                .ok_or_else(|| "invalid sidecar filename".to_owned())?,
                        ),
                    )?;
                }
                archived += 1;
            } else {
                spool
                    .verify(&existing)
                    .map_err(|_| "existing duplicate checksum verification failed".to_owned())?;
            }
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
        "published {published} stable artifact(s); skipped {duplicates} verified duplicate(s); archived {archived} completed source(s)"
    ))
}

fn archive_completed_source(source: &Path, archive_dir: &Path, bytes: &[u8]) -> Result<(), String> {
    let filename = source
        .file_name()
        .ok_or_else(|| "invalid source filename".to_owned())?;
    let destination = archive_dir.join(filename);
    let expected = format!("{:x}", Sha256::digest(bytes));
    archive_copy_with_checksum(source, &destination, Some(&expected))
}

fn archive_copy(source: &Path, destination: &Path) -> Result<(), String> {
    archive_copy_with_checksum(source, destination, None)
}

fn archive_copy_with_checksum(
    source: &Path,
    destination: &Path,
    expected_checksum: Option<&str>,
) -> Result<(), String> {
    let source_checksum = checksum_file(source)?;
    if expected_checksum.is_some_and(|expected| expected != source_checksum) {
        return Err("source changed before archival".into());
    }
    if destination.exists() {
        if checksum_file(destination)? != source_checksum {
            return Err("archive destination conflicts with source".into());
        }
        fs::remove_file(source).map_err(|_| "could not remove archived source".to_owned())?;
        return Ok(());
    }
    let filename = destination
        .file_name()
        .ok_or_else(|| "invalid archive filename".to_owned())?;
    let temporary =
        destination.with_file_name(format!(".{}.archive.part", filename.to_string_lossy()));
    let mut input =
        fs::File::open(source).map_err(|_| "could not read source for archival".to_owned())?;
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|_| "could not create archive temporary file".to_owned())?;
    std::io::copy(&mut input, &mut output)
        .map_err(|_| "could not copy source to archive".to_owned())?;
    output
        .sync_all()
        .map_err(|_| "could not sync archived source".to_owned())?;
    drop(output);
    if checksum_file(&temporary)? != source_checksum {
        let _ = fs::remove_file(&temporary);
        return Err("archived source checksum verification failed".into());
    }
    fs::rename(&temporary, destination)
        .map_err(|_| "could not publish archived source".to_owned())?;
    fs::remove_file(source).map_err(|_| "could not remove archived source".to_owned())?;
    Ok(())
}

fn checksum_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(path).map_err(|_| "could not read file checksum".to_owned())?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 16 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|_| "could not read file checksum".to_owned())?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
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
        CaptureSidecar, Config, ExitCode, USAGE, capture_v4l2_segment, enrich_capture_metadata,
        execute, filename_capture_time, manage_job, poll_completed_files, run, show_status,
        supported_media_type, utc_timestamp, valid_utc_timestamp, write_jpeg_frame,
        write_v4l2_frames,
    };
    use lunsaran_taytay::{
        Artifact, ArtifactId, ArtifactState, SourceId, adapter::filesystem::FilesystemAdapter,
        spool::Spool,
    };
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
    fn v4l2_requires_a_complete_device_configuration() {
        let config = Config {
            input_dir: "input".into(),
            spool_dir: "spool".into(),
            archive_dir: "archive".into(),
            source_id: "camera-1".into(),
            spool_quota_bytes: 1024,
            v4l2_device: None,
            v4l2_width: None,
            v4l2_height: None,
            v4l2_pixel_format: None,
            v4l2_segment_frames: None,
            v4l2_segment_max_bytes: None,
        };
        assert_eq!(
            capture_v4l2_segment(&config).unwrap_err(),
            "V4L2 device, width, height, and four-character pixel format are required"
        );
    }

    #[test]
    fn v4l2_rejects_unsupported_formats_and_unbounded_segment_settings() {
        let mut config = Config {
            input_dir: "input".into(),
            spool_dir: "spool".into(),
            archive_dir: "archive".into(),
            source_id: "camera-1".into(),
            spool_quota_bytes: 1024,
            v4l2_device: Some("/not/a/camera".into()),
            v4l2_width: Some(640),
            v4l2_height: Some(480),
            v4l2_pixel_format: Some("YUYV".into()),
            v4l2_segment_frames: None,
            v4l2_segment_max_bytes: None,
        };
        assert!(capture_v4l2_segment(&config).unwrap_err().contains("MJPEG"));
        config.v4l2_pixel_format = Some("MJPG".into());
        config.v4l2_segment_frames = Some(301);
        assert!(
            capture_v4l2_segment(&config)
                .unwrap_err()
                .contains("segment limits")
        );
        config.v4l2_segment_frames = Some(1);
        config.v4l2_segment_max_bytes = Some(64 * 1024 * 1024 + 1);
        assert!(
            capture_v4l2_segment(&config)
                .unwrap_err()
                .contains("segment limits")
        );
        config.v4l2_segment_max_bytes = None;
        assert!(
            capture_v4l2_segment(&config)
                .unwrap_err()
                .contains("initialize V4L2")
        );
    }

    #[test]
    fn bounds_and_reports_v4l2_frame_sequences() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-v4l2-sequence-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let mut count = 0;
        let result = write_v4l2_frames(&root, 2, 10, || {
            count += 1;
            Ok(vec![0xff, 0xd8, 0x01, 0xff, 0xd9])
        })
        .unwrap();
        assert!(result.contains("captured 2 timestamped V4L2 JPEG frames (10 bytes)"));
        assert_eq!(count, 2);
        assert_eq!(fs::read_dir(&root).unwrap().count(), 4);

        let result = write_v4l2_frames(&root, 2, 9, || Ok(vec![0xff, 0xd8, 0x01, 0xff, 0xd9]));
        assert!(result.unwrap_err().contains("size limit"));
        let result = write_v4l2_frames(&root, 1, 10, || Err("frame failure".into()));
        assert_eq!(result.unwrap_err(), "frame failure");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn writes_only_complete_timestamped_jpegs_atomically() {
        let root = std::env::temp_dir().join(format!(
            "usb-camera-taytay-v4l2-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let captured_at = "2026-10-21T00:00:00Z";
        assert!(write_jpeg_frame(&root, &[0, 1, 2, 3], captured_at).is_err());
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);

        write_jpeg_frame(&root, &[0xff, 0xd8, 0x01, 0xff, 0xd9], captured_at).unwrap();
        let entries = fs::read_dir(&root)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(entries.len(), 2);
        assert!(
            entries
                .iter()
                .all(|entry| !entry.file_name().to_string_lossy().ends_with(".part"))
        );
        let image = entries
            .iter()
            .find(|entry| entry.path().extension().is_some_and(|ext| ext == "jpg"))
            .unwrap();
        let sidecar = entries
            .iter()
            .find(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
            .unwrap();
        assert_eq!(
            fs::read(image.path()).unwrap(),
            [0xff, 0xd8, 0x01, 0xff, 0xd9]
        );
        assert_eq!(
            serde_json::from_slice::<CaptureSidecar>(&fs::read(sidecar.path()).unwrap())
                .unwrap()
                .captured_at
                .as_deref(),
            Some(captured_at)
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn formats_system_capture_time_as_utc() {
        assert_eq!(utc_timestamp(UNIX_EPOCH).unwrap(), "1970-01-01T00:00:00Z");
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
        assert_eq!(run(["pause".to_owned()].into_iter()), ExitCode::from(2));
        assert_eq!(
            run([
                "status".to_owned(),
                "config.toml".to_owned(),
                "extra".to_owned()
            ]
            .into_iter()),
            ExitCode::from(2)
        );
        assert_eq!(
            run([
                "resume".to_owned(),
                "id".to_owned(),
                "config.toml".to_owned(),
                "extra".to_owned()
            ]
            .into_iter()),
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
        assert_eq!(execute("status", &config_path).unwrap(), "no artifacts");
        assert_eq!(
            execute("fixture", &config_path).unwrap(),
            "published fixture.jpg and fixture.mp4"
        );
        fs::write(
            root.join("input/fixture.json"),
            r#"{"captured_at":"2024-10-21T12:34:56Z"}"#,
        )
        .unwrap();
        let source_bytes = fs::read(root.join("input/fixture.jpg")).unwrap();
        assert!(
            execute("run-filesystem", &config_path)
                .unwrap()
                .contains("published 2")
        );
        assert!(root.join("spool/ledger.jsonl").is_file());
        assert_eq!(fs::read_dir(root.join("spool")).unwrap().count(), 3);
        assert!(root.join("input/fixture.jpg").is_file());
        assert!(root.join("input/fixture.mp4").is_file());
        assert!(root.join("input/pending.jpg.part").is_file());
        assert!(root.join("archive").is_dir());
        assert_eq!(
            execute("run-filesystem", &config_path).unwrap(),
            "published 0 stable artifact(s); skipped 2 verified duplicate(s); archived 0 completed source(s)"
        );
        assert_eq!(fs::read_dir(root.join("spool")).unwrap().count(), 3);

        let spool = Spool::open(root.join("spool"), 1048576).unwrap();
        let job_id = spool.ledger().pending()[0].artifact.id.0.clone();
        let config = Config::load(&config_path).unwrap();
        assert_eq!(
            run(["status".to_owned(), config_path.display().to_string()].into_iter()),
            ExitCode::SUCCESS
        );
        let status = show_status(&config).unwrap();
        assert!(status.contains(&job_id));
        assert!(status.contains("Published"));
        assert!(!status.contains("fixture.jpg"));
        assert_eq!(
            run([
                "pause".to_owned(),
                job_id.clone(),
                config_path.display().to_string()
            ]
            .into_iter()),
            ExitCode::SUCCESS
        );
        assert_eq!(
            run([
                "pause".to_owned(),
                job_id.clone(),
                config_path.display().to_string()
            ]
            .into_iter()),
            ExitCode::from(1)
        );
        assert!(show_status(&config).unwrap().contains("Paused"));
        assert_eq!(
            Spool::open(root.join("spool"), 1048576)
                .unwrap()
                .ledger()
                .paused()
                .len(),
            1
        );
        assert_eq!(
            run([
                "resume".to_owned(),
                job_id.clone(),
                config_path.display().to_string()
            ]
            .into_iter()),
            ExitCode::SUCCESS
        );
        assert!(show_status(&config).unwrap().contains("Published"));
        assert_eq!(
            manage_job("pause", "missing-artifact", &config_path).unwrap_err(),
            "could not pause artifact; check its ID and current state"
        );
        let mut job = spool.ledger().pending().remove(0);
        job.transition(ArtifactState::Uploading).unwrap();
        job.transition(ArtifactState::Completed).unwrap();
        spool.ledger().update(job).unwrap();

        assert_eq!(
            execute("run-filesystem", &config_path).unwrap(),
            "published 0 stable artifact(s); skipped 2 verified duplicate(s); archived 1 completed source(s)"
        );
        assert!(!root.join("input/fixture.jpg").exists());
        assert!(!root.join("input/fixture.json").exists());
        assert_eq!(
            fs::read(root.join("archive/fixture.jpg")).unwrap(),
            source_bytes
        );
        assert!(root.join("archive/fixture.json").is_file());

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
