use std::env;
use std::process::ExitCode;

const USAGE: &str = "Usage: usb-camera-taytay [--help] [--version]";

#[cfg(not(coverage))]
fn main() -> ExitCode {
    run(env::args().skip(1))
}

fn run(mut arguments: impl Iterator<Item = String>) -> ExitCode {
    match arguments.next().as_deref() {
        Some("--help" | "-h") => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some("--version" | "-V") => {
            println!("usb-camera-taytay {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
        Some(argument) => {
            eprintln!("unknown argument: {argument}\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ExitCode, USAGE, run};

    #[test]
    fn usage_names_the_binary_and_supported_options() {
        assert!(USAGE.starts_with("Usage: usb-camera-taytay"));
        assert!(USAGE.contains("--help"));
        assert!(USAGE.contains("--version"));
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
    }
}
