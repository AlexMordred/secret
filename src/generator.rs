use rand::Rng;
use super::checker::Checker;

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789`~!@#$%^&*()-_=+\\/;:'\",.?<>[]{}|";

pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, length: &u8) -> String {
        let checker = Checker::new();
        let mut password = String::with_capacity(*length as usize);
        let mut rng = rand::thread_rng();

        for _ in 0..*length {
            let random: usize = rng.gen_range(0..CHARS.len());
            let random_char: char = CHARS.chars().nth(random).unwrap();

            password.push(random_char);
        }

        if
            !checker.has_small_letters(&password) ||
            !checker.has_capital_letters(&password) ||
            !checker.has_numbers(&password) ||
            !checker.has_special_chars(&password)
        {
            return self.generate(length);
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

        assert_eq!(8, generator.generate(&8).len());
        assert_eq!(12, generator.generate(&12).len());
        assert_eq!(100, generator.generate(&100).len());
    }

    #[test]
    fn all_generated_passwords_are_not_the_same() {
        let generator = Generator::new();

        let mut passwords: Vec<String> = vec![generator.generate(&16)];

        for _ in 0..99 {
            let pwd = generator.generate(&16);

            assert!(passwords.binary_search(&pwd).is_err());

            passwords.push(pwd);
        }
    }

    #[test]
    fn generated_password_have_at_least_one_char_of_each_of_the_4_types() {
        let generator = Generator::new();
        let checker = Checker::new();
        let pwd = generator.generate(&16);

        assert!(checker.has_small_letters(&pwd));
        assert!(checker.has_capital_letters(&pwd));
        assert!(checker.has_numbers(&pwd));
        assert!(checker.has_special_chars(&pwd));
    }
}