use regex::Regex;

const COMMON_PASSWORDS: [&str; 3] = [
    "qwerty",
    "password",
    "secret",
];

#[derive(Debug, PartialEq)]
pub enum Strength {
    Weak,
    Medium,
    Strong,
}

pub struct Checker {}

impl Checker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check(&self, password: &String) -> Strength {
        if self.too_short(password) || self.is_common_word(password) {
            return Strength::Weak;
        }

        let mut char_types: u8 = 0;

        if self.has_small_letters(password) { char_types += 1; }
        if self.has_capital_letters(password) { char_types += 1; }
        if self.has_numbers(password) { char_types += 1; }
        if self.has_special_chars(password) { char_types += 1; }

        if char_types < 2 {
            return Strength::Weak;
        }

        if char_types < 4 {
            return Strength::Medium;
        }

        Strength::Strong
    }

    fn too_short(&self, password: &String) -> bool {
        password.len() < 8
    }

    fn is_common_word(&self, password: &String) -> bool {
        COMMON_PASSWORDS.contains(&password.as_str())
    }

    fn has_small_letters(&self, password: &String) -> bool {
        let re = Regex::new(r"[a-z]").unwrap();

        re.is_match(&password)
    }

    fn has_capital_letters(&self, password: &String) -> bool {
        let re = Regex::new(r"[A-Z]").unwrap();

        re.is_match(&password)
    }

    fn has_numbers(&self, password: &String) -> bool {
        let re = Regex::new(r"[0-9]").unwrap();

        re.is_match(&password)
    }

    fn has_special_chars(&self, password: &String) -> bool {
        let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();

        re.is_match(&password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Weak passwords */
    #[test]
    fn passwords_less_than_8_chars_long_are_always_weak() {
        let checker = Checker::new();

        assert_eq!(Strength::Weak, checker.check(&String::from("1234567")));
        assert_eq!(Strength::Weak, checker.check(&String::from("1a3b5c7")));
        assert_eq!(Strength::Weak, checker.check(&String::from("1aAb5c7")));
        assert_eq!(Strength::Weak, checker.check(&String::from("1aAb5c@")));
    }

    #[test]
    fn passwords_that_are_common_are_weak() {
        let checker = Checker::new();

        assert_eq!(Strength::Weak, checker.check(&String::from("qwerty")));
        assert_eq!(Strength::Weak, checker.check(&String::from("password")));
        assert_eq!(Strength::Weak, checker.check(&String::from("secret")));
    }

    #[test]
    fn passwords_with_only_small_letters_are_weak() {
        let checker = Checker::new();
        assert_eq!(Strength::Weak, checker.check(&String::from("qwertyuiop")));
    }

    #[test]
    fn passwords_with_only_capital_letters_are_weak() {
        let checker = Checker::new();
        assert_eq!(Strength::Weak, checker.check(&String::from("QWERTYUIOP")));
    }

    #[test]
    fn passwords_with_only_numbers_are_weak() {
        let checker = Checker::new();
        assert_eq!(Strength::Weak, checker.check(&String::from("1234567890")));
    }

    #[test]
    fn passwords_with_only_symbols_are_weak() {
        let checker = Checker::new();
        assert_eq!(Strength::Weak, checker.check(&String::from("!@#$%^&*()")));
    }

    #[test]
    fn passwords_with_a_single_repetitive_character_are_weak() {
        let checker = Checker::new();

        assert_eq!(Strength::Weak, checker.check(&String::from("aaaaaaaaaa")));
        assert_eq!(Strength::Weak, checker.check(&String::from("AAAAAAAAAA")));
        assert_eq!(Strength::Weak, checker.check(&String::from("1111111111")));
        assert_eq!(Strength::Weak, checker.check(&String::from("@@@@@@@@@@")));
    }

    /* Medium passwords */
    #[test]
    fn passwords_at_least_8_chars_long_with_2_char_types_in_them_are_medium() {
        let checker = Checker::new();

        assert_eq!(Strength::Medium, checker.check(&String::from("abcdABCD")));
        assert_eq!(Strength::Medium, checker.check(&String::from("abcd!@#$")));
        assert_eq!(Strength::Medium, checker.check(&String::from("abcd1234")));
        assert_eq!(Strength::Medium, checker.check(&String::from("ABCD1234")));
        assert_eq!(Strength::Medium, checker.check(&String::from("ABCD!@#$")));
        assert_eq!(Strength::Medium, checker.check(&String::from("1234!@#$")));
        assert_eq!(
            Strength::Medium,
            checker.check(&String::from("S0meL0ngP4ssw0rd"))
        );
    }

    #[test]
    fn passwords_at_least_8_chars_long_with_3_char_types_in_them_are_medium() {
        let checker = Checker::new();

        assert_eq!(Strength::Medium, checker.check(&String::from("abcDEF12")));
        assert_eq!(Strength::Medium, checker.check(&String::from("abcDEF!@")));
        assert_eq!(Strength::Medium, checker.check(&String::from("ABC123!@")));
        assert_eq!(Strength::Medium, checker.check(&String::from("abc123!@")));
        assert_eq!(
            Strength::Medium,
            checker.check(&String::from("S0meStr0ngPassw0rd"))
        );
    }

    /* Strong passwords */
    #[test]
    fn passwords_at_least_8_chars_long_with_all_char_types_in_them_are_strong() {
        let checker = Checker::new();

        assert_eq!(Strength::Strong, checker.check(&String::from("12abCD!@")));
        assert_eq!(Strength::Strong, checker.check(&String::from("12345aA!")));
        assert_eq!(Strength::Strong, checker.check(&String::from("abcdeT7*")));
        assert_eq!(Strength::Strong, checker.check(&String::from("QWERTy6^")));
        assert_eq!(
            Strength::Strong,
            checker.check(&String::from("S0meL0n(P@ssWORD"))
        );
    }
}
