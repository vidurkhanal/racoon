use super::{Node, Statement};

#[derive(Default, Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = &self.statements;

        for (_, statement) in statements.iter().enumerate() {
            writeln!(f, "{}", statement.to_string())?;
        }
        Ok(())
    }
}
