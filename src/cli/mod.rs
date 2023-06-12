use crate::utils::formatted_strings::APP_VERSION;

/// Parse CLI arguments, and exit if `--help`, `--version`, or an
/// unknown argument was supplied
pub fn parse_cli_args() {
    let args = std::env::args().skip(1);
    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => print_help(),
            "--version" | "-v" | "-V" => print_version(),
            _ => {
                unknown_argument(&arg);
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }
}

fn print_help() {
    print_version();
    eprintln!(
        "Application to comfortably monitor your Internet traffic

Usage: sniffnet [OPTIONS]

Options:
    -h, --help      Print help
    -v, --version   Print version info"
    );
}

fn print_version() {
    eprintln!("sniffnet {APP_VERSION}");
}

fn unknown_argument(arg: &str) {
    eprintln!(
        "error: unknown argument '{arg}'

For more information, try '--help'"
    );
}
