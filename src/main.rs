use clap::{App, Arg, ArgMatches, SubCommand};
use secret::{Options, Password, Strength};

const ARG_PASSWORD: &str = "password";
const ARG_LENGTH: &str = "length";
const ARG_FORMAT: &str = "format";

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
        .subcommand(
            SubCommand::with_name(COMMAND_CHECK)
                .about("Check password strength")
                .arg(
                    Arg::with_name(ARG_PASSWORD)
                        .required(true)
                        .help("Password to be checked")
                )
        )
        .subcommand(
            SubCommand::with_name(COMMAND_GENERATE)
                .about("Generate a random password")
                .arg(
                    Arg::with_name(ARG_LENGTH)
                        .required(true)
                        .help("Length of the password to be generated")
                        .default_value("16")
                )
                .arg(
                    Arg::with_name(ARG_FORMAT)
                        .required(true)
                        .help("Characters to be included in the generated password (in any order). a - small letters; A - capital letters; 1 - numbers; @ - special characters.")
                        .default_value("aA1@")
                )
        )
}

fn die(message: &str) -> ! {
    println!("{}", message);

    std::process::exit(1);
}

fn check_password(args: &ArgMatches) {
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

fn generate_password(args: &ArgMatches) {
    let password = Password::new();

    let length = args.value_of(ARG_LENGTH).unwrap().parse::<u8>();
    if length.is_err() {
        die("Password can only be between 4 and 255 characters long")
    }

    let length = length.unwrap();
    if length < 4 {
        die("Password can only be between 4 and 255 characters long")
    }

    let format = args.value_of(ARG_FORMAT).unwrap();

    let options = Options {
        length,
        small_letters: format.contains('a'),
        capital_letters: format.contains('A'),
        numbers: format.contains('1'),
        special_chars: format.contains('@'),
    };

    println!("{}", password.generate(options));
}

fn main() {
    let args = make_app().get_matches();

    match args.subcommand_name() {
        Some(COMMAND_CHECK) => check_password(args.subcommand_matches(COMMAND_CHECK).unwrap()),
        Some(COMMAND_GENERATE) => {
            generate_password(args.subcommand_matches(COMMAND_GENERATE).unwrap())
        }
        Some(_) => die("Invalid argument"),
        None => die(format!(
            "Not enough arguments. For more information try {}--help{}",
            COLOR_GREEN, COLOR_RESET
        )
        .as_str()),
    };
}
