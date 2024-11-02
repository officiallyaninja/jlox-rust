use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl Literal {
    pub fn truthy(&self) -> bool {
        match self {
            Literal::False | Literal::Nil => false,
            _ => true,
        }
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Number(value.into())
    }
}
impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::String(value.into())
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        if value {
            Literal::True
        } else {
            Literal::False
        }
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::String(value) => format!("{value}"),
                Literal::Number(num) => format!("{num}"),
                Literal::True => "true".to_string(),
                Literal::False => "false".to_string(),
                Literal::Nil => "nil".to_string(),
            }
        )
    }
}
