use clap::{App, Arg, ArgMatches};
use password::{Password, Strength};

const ARG_COMMAND: &str = "command";
const ARG_PASSWORD: &str = "password";

const COMMAND_CHECK: &str = "check";
const COMMAND_GENERATE: &str = "generate";

const COLOR_RED: &str = "\x1b[0;31m";
const COLOR_GREEN: &str = "\x1b[0;32m";
const COLOR_YELLOW: &str = "\x1b[0;33m";
const COLOR_RESET: &str = "\x1b[0m";

fn make_app() -> App<'static, 'static> {
    App::new("Secret")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Random password generator and password strength checker")
        .arg(
            Arg::with_name(ARG_COMMAND)
                .required(true)
                .possible_values(&[COMMAND_CHECK, COMMAND_GENERATE])
        )
        .arg(
            Arg::with_name(ARG_PASSWORD)
                .required_if(ARG_COMMAND, COMMAND_CHECK)
        )
}

fn die(message: &str) -> ! {
    println!("{}", message);

    std::process::exit(1);
}

fn check_password(args: ArgMatches) {
    let password = Password::new();
    let pwd = args.value_of(ARG_PASSWORD).unwrap().to_string();

    let result: Strength = password.check(&pwd);

    let strength = match result {
        Strength::Weak => "WEAK",
        Strength::Medium => "MEDIUM",
        Strength::Strong => "STRONG",
    };
    let color = match result {
        Strength::Weak => COLOR_RED,
        Strength::Medium => COLOR_YELLOW,
        Strength::Strong => COLOR_GREEN,
    };

    println!("Your password is: {}{}{}", color, strength, COLOR_RESET);
}

fn main() {
    let args = make_app().get_matches();

    match args.value_of(ARG_COMMAND) {
        Some(COMMAND_CHECK) => check_password(args),
        Some(_) => die("Not enough arguments"),
        None => die("Not enough arguments")
    };
}
