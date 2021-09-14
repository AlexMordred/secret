mod checker;
mod generator;

use checker::Checker;
use generator::Generator;

pub struct Password {
    pub checker: Checker,
    pub generator: Generator,
}

impl Password {
    fn new() -> Self {
        Self {
            checker: Checker::new(),
            generator: Generator::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializing_new_password_object_doesnt_require_any_arguments() {
        Password::new();
    }
}
