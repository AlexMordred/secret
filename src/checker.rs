use regex::Regex;

const COMMON_PASSWORDS: [&str; 3] = ["qwerty", "password", "secret"];

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

    pub fn check(&self, password: &str) -> Strength {
        if self.too_short(password) || self.is_common_word(password) {
            return Strength::Weak;
        }

        let mut char_types: u8 = 0;

        if self.has_small_letters(password) {
            char_types += 1;
        }
        if self.has_capital_letters(password) {
            char_types += 1;
        }
        if self.has_numbers(password) {
            char_types += 1;
        }
        if self.has_special_chars(password) {
            char_types += 1;
        }

        if char_types < 2 {
            return Strength::Weak;
        }

        if char_types < 4 {
            return Strength::Medium;
        }

        Strength::Strong
    }

    fn too_short(&self, password: &str) -> bool {
        password.len() < 8
    }

    fn is_common_word(&self, password: &str) -> bool {
        COMMON_PASSWORDS.contains(&password)
    }

    pub fn has_small_letters(&self, password: &str) -> bool {
        let re = Regex::new(r"[a-z]").unwrap();

        re.is_match(password)
    }

    pub fn has_capital_letters(&self, password: &str) -> bool {
        let re = Regex::new(r"[A-Z]").unwrap();

        re.is_match(password)
    }

    pub fn has_numbers(&self, password: &str) -> bool {
        let re = Regex::new(r"[0-9]").unwrap();

        re.is_match(password)
    }

    pub fn has_special_chars(&self, password: &str) -> bool {
        let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();

        re.is_match(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Weak passwords */
    #[test]
    fn passwords_less_than_8_chars_long_are_always_weak() {
        let checker = Checker::new();

        assert_eq!(Strength::Weak, checker.check("1234567"));
        assert_eq!(Strength::Weak, checker.check("1a3b5c7"));
        assert_eq!(Strength::Weak, checker.check("1aAb5c7"));
        assert_eq!(Strength::Weak, checker.check("1aAb5c@"));
    }

    #[test]
    fn passwords_that_are_common_are_weak() {
        let checker = Checker::new();

        assert_eq!(Strength::Weak, checker.check("qwerty"));
        assert_eq!(Strength::Weak, checker.check("password"));
        assert_eq!(Strength::Weak, checker.check("secret"));
    }

    #[test]
    fn passwords_with_only_small_letters_are_weak() {
        let checker = Checker::new();
        assert_eq!(Strength::Weak, checker.check("qwertyuiop"));
    }

    #[test]
    fn passwords_with_only_capital_letters_are_weak() {
        let checker = Checker::new();
        assert_eq!(Strength::Weak, checker.check("QWERTYUIOP"));
    }

    #[test]
    fn passwords_with_only_numbers_are_weak() {
        let checker = Checker::new();
        assert_eq!(Strength::Weak, checker.check("1234567890"));
    }

    #[test]
    fn passwords_with_only_symbols_are_weak() {
        let checker = Checker::new();
        assert_eq!(Strength::Weak, checker.check("!@#$%^&*()"));
    }

    #[test]
    fn passwords_with_a_single_repetitive_character_are_weak() {
        let checker = Checker::new();

        assert_eq!(Strength::Weak, checker.check("aaaaaaaaaa"));
        assert_eq!(Strength::Weak, checker.check("AAAAAAAAAA"));
        assert_eq!(Strength::Weak, checker.check("1111111111"));
        assert_eq!(Strength::Weak, checker.check("@@@@@@@@@@"));
    }

    /* Medium passwords */
    #[test]
    fn passwords_at_least_8_chars_long_with_2_char_types_in_them_are_medium() {
        let checker = Checker::new();

        assert_eq!(Strength::Medium, checker.check("abcdABCD"));
        assert_eq!(Strength::Medium, checker.check("abcd!@#$"));
        assert_eq!(Strength::Medium, checker.check("abcd1234"));
        assert_eq!(Strength::Medium, checker.check("ABCD1234"));
        assert_eq!(Strength::Medium, checker.check("ABCD!@#$"));
        assert_eq!(Strength::Medium, checker.check("1234!@#$"));
        assert_eq!(Strength::Medium, checker.check("S0meL0ngP4ssw0rd"));
    }

    #[test]
    fn passwords_at_least_8_chars_long_with_3_char_types_in_them_are_medium() {
        let checker = Checker::new();

        assert_eq!(Strength::Medium, checker.check("abcDEF12"));
        assert_eq!(Strength::Medium, checker.check("abcDEF!@"));
        assert_eq!(Strength::Medium, checker.check("ABC123!@"));
        assert_eq!(Strength::Medium, checker.check("abc123!@"));
        assert_eq!(Strength::Medium, checker.check("S0meStr0ngPassw0rd"));
    }

    /* Strong passwords */
    #[test]
    fn passwords_at_least_8_chars_long_with_all_char_types_in_them_are_strong() {
        let checker = Checker::new();

        assert_eq!(Strength::Strong, checker.check("12abCD!@"));
        assert_eq!(Strength::Strong, checker.check("12345aA!"));
        assert_eq!(Strength::Strong, checker.check("abcdeT7*"));
        assert_eq!(Strength::Strong, checker.check("QWERTy6^"));
        assert_eq!(Strength::Strong, checker.check("S0meL0n(P@ssWORD"));
    }
}
