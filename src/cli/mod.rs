use crate::utils::formatted_strings::APP_VERSION;

/// Parse CLI arguments, and exit if `--help`, `--version`, or an
/// unknown argument was supplied
pub fn parse_cli_args() {
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => print_help(),
            "--version" | "-v" => print_version(),
            _ => {
                unknown_argument(&arg);
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }
}

fn print_help() {
    println!(
        "Application to comfortably monitor your Internet traffic\n\
        Usage: sniffnet [OPTIONS]\n\
        Options:\n\
        \t-h, --help      Print help\n\
        \t-v, --version   Print version info\n\
        (Run without options to start the app)"
    );
}

fn print_version() {
    println!("sniffnet {APP_VERSION}");
}

fn unknown_argument(arg: &str) {
    eprintln!(
        "sniffnet: unknown option '{arg}'\n\
        For more information, try 'sniffnet --help'"
    );
}
