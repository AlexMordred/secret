use super::checker::Checker;
use rand::Rng;

const SMALL_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const CAPITAL_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SPECIAL_CHARS: &str = "`~!@#$%^&*()-_=+\\/;:'\",.?<>[]{}|";

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub length: u8,
    pub small_letters: bool,
    pub capital_letters: bool,
    pub numbers: bool,
    pub special_chars: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            length: 16,
            small_letters: true,
            capital_letters: true,
            numbers: true,
            special_chars: true,
        }
    }
}

pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Self {}
    }

    fn charset_for_options(&self, options: Options) -> String {
        let mut charset = String::new();

        if options.small_letters {
            charset.push_str(SMALL_LETTERS)
        };
        if options.capital_letters {
            charset.push_str(CAPITAL_LETTERS)
        };
        if options.numbers {
            charset.push_str(NUMBERS)
        };
        if options.special_chars {
            charset.push_str(SPECIAL_CHARS)
        };

        charset
    }

    fn validate_password(&self, password: &str, options: &Options) -> bool {
        let checker = Checker::new();

        if (options.small_letters && !checker.has_small_letters(password))
            || (options.capital_letters && !checker.has_capital_letters(password))
            || (options.numbers && !checker.has_numbers(password))
            || (options.special_chars && !checker.has_special_chars(password))
        {
            return false;
        }

        true
    }

    pub fn generate(&self, options: Options) -> String {
        let mut password = String::with_capacity(options.length as usize);
        let mut rng = rand::thread_rng();
        let charset: String = self.charset_for_options(options);

        for _ in 0..options.length {
            let random: usize = rng.gen_range(0..charset.len());
            let random_char: char = charset.chars().nth(random).unwrap();

            password.push(random_char);
        }

        if !self.validate_password(&password, &options) {
            return self.generate(options);
        }

        password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_a_password_of_provided_length() {
        let generator = Generator::new();

        let mut options = Options::default();

        options.length = 8;
        assert_eq!(8, generator.generate(options).len());

        options.length = 12;
        assert_eq!(12, generator.generate(options).len());

        options.length = 100;
        assert_eq!(100, generator.generate(options).len());
    }

    #[test]
    fn all_generated_passwords_are_not_the_same() {
        let generator = Generator::new();

        let mut passwords: Vec<String> = vec![generator.generate(Options::default())];

        for _ in 0..99 {
            let pwd = generator.generate(Options::default());

            assert!(passwords.binary_search(&pwd).is_err());

            passwords.push(pwd);
        }
    }

    #[test]
    fn generated_password_have_at_least_one_char_of_each_of_the_4_types() {
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(Options::default());

        assert!(checker.has_small_letters(&pwd));
        assert!(checker.has_capital_letters(&pwd));
        assert!(checker.has_numbers(&pwd));
        assert!(checker.has_special_chars(&pwd));
    }

    #[test]
    fn generate_password_with_only_small_letters() {
        let options = Options {
            length: 16,
            small_letters: true,
            capital_letters: false,
            numbers: false,
            special_chars: false,
        };
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(options);

        assert!(checker.has_small_letters(&pwd));
        assert!(!checker.has_capital_letters(&pwd));
        assert!(!checker.has_numbers(&pwd));
        assert!(!checker.has_special_chars(&pwd));
    }

    #[test]
    fn generate_password_with_only_capital_letters() {
        let options = Options {
            length: 16,
            small_letters: false,
            capital_letters: true,
            numbers: false,
            special_chars: false,
        };
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(options);

        assert!(!checker.has_small_letters(&pwd));
        assert!(checker.has_capital_letters(&pwd));
        assert!(!checker.has_numbers(&pwd));
        assert!(!checker.has_special_chars(&pwd));
    }

    #[test]
    fn generate_password_with_only_numbers() {
        let options = Options {
            length: 16,
            small_letters: false,
            capital_letters: false,
            numbers: true,
            special_chars: false,
        };
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(options);

        assert!(!checker.has_small_letters(&pwd));
        assert!(!checker.has_capital_letters(&pwd));
        assert!(checker.has_numbers(&pwd));
        assert!(!checker.has_special_chars(&pwd));
    }

    #[test]
    fn generate_password_with_only_special_characters() {
        let options = Options {
            length: 16,
            small_letters: false,
            capital_letters: false,
            numbers: false,
            special_chars: true,
        };
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(options);

        assert!(!checker.has_small_letters(&pwd));
        assert!(!checker.has_capital_letters(&pwd));
        assert!(!checker.has_numbers(&pwd));
        assert!(checker.has_special_chars(&pwd));
    }

    #[test]
    fn generate_password_with_only_small_and_capital_letters() {
        let options = Options {
            length: 16,
            small_letters: true,
            capital_letters: true,
            numbers: false,
            special_chars: false,
        };
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(options);

        assert!(checker.has_small_letters(&pwd));
        assert!(checker.has_capital_letters(&pwd));
        assert!(!checker.has_numbers(&pwd));
        assert!(!checker.has_special_chars(&pwd));
    }

    #[test]
    fn generate_password_with_only_letters_and_numbers() {
        let options = Options {
            length: 16,
            small_letters: true,
            capital_letters: true,
            numbers: true,
            special_chars: false,
        };
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(options);

        assert!(checker.has_small_letters(&pwd));
        assert!(checker.has_capital_letters(&pwd));
        assert!(checker.has_numbers(&pwd));
        assert!(!checker.has_special_chars(&pwd));
    }

    #[test]
    fn generate_password_with_only_numbers_and_special_characters() {
        let options = Options {
            length: 16,
            small_letters: false,
            capital_letters: false,
            numbers: true,
            special_chars: true,
        };
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(options);

        assert!(!checker.has_small_letters(&pwd));
        assert!(!checker.has_capital_letters(&pwd));
        assert!(checker.has_numbers(&pwd));
        assert!(checker.has_special_chars(&pwd));
    }
}
