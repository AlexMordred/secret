mod checker;
mod generator;

use checker::Checker;
pub use checker::Strength;
use generator::Generator;

pub struct Password {
    pub checker: Checker,
    pub generator: Generator,
}

impl Password {
    pub fn new() -> Self {
        Self {
            checker: Checker::new(),
            generator: Generator::new(),
        }
    }

    pub fn check(&self, password: &String) -> Strength {
        self.checker.check(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializing_new_password_object_doesnt_require_any_arguments() {
        Password::new();
    }

    #[test]
    fn proxying_call_to_the_checker_check_method() {
        let password = Password::new();

        password.check(&String::from("qwerty"));
    }
}
