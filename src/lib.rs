mod checker;

use checker::Checker;

pub struct Password {
    pub checker: Checker,
    pub generator: bool,
}

impl Password {
    fn new() -> Self {
        Self {
            checker: Checker::new(),
            generator: true,
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
