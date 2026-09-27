// Parser descendente recursivo para las expresiones de la calculadora.
//
//   expr    := term (('+' | '-') term)*
//   term    := unary (('*' | '/') unary)*
//   unary   := ('+' | '-' | '√') unary | power
//   power   := postfix ('^' unary)?          // asociativo por la derecha
//   postfix := primary ('%' | '√')*
//   primary := number | '(' expr ')'
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    EmptyExpression,
    UnexpectedEnd,
    UnexpectedChar(char),
    MissingClosingParen,
    InvalidNumber,
    DivisionByZero,
    UndefinedResult,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyExpression => write!(f, "Expresion vacia"),
            ParseError::UnexpectedEnd => write!(f, "Expresion incompleta"),
            ParseError::UnexpectedChar(c) => write!(f, "Caracter invalido: '{c}'"),
            ParseError::MissingClosingParen => write!(f, "Falta cerrar parentesis"),
            ParseError::InvalidNumber => write!(f, "Numero invalido"),
            ParseError::DivisionByZero => write!(f, "Division por cero"),
            ParseError::UndefinedResult => write!(f, "Resultado no definido"),
        }
    }
}

impl std::error::Error for ParseError {}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(expression: &str) -> Self {
        Self {
            chars: expression.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let current = self.peek();
        if current.is_some() {
            self.pos += 1;
        }
        current
    }

    fn skip_spaces(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.pos += 1;
        }
    }

    fn expect_end(&mut self) -> Result<(), ParseError> {
        self.skip_spaces();
        match self.peek() {
            Some(c) => Err(ParseError::UnexpectedChar(c)),
            None => Ok(()),
        }
    }

    fn check(value: f64) -> Result<f64, ParseError> {
        if value.is_nan() {
            return Err(ParseError::UndefinedResult);
        }
        if value.is_infinite() {
            return Err(ParseError::UndefinedResult);
        }
        Ok(value)
    }

    fn parse_expr(&mut self) -> Result<f64, ParseError> {
        let mut value = self.parse_term()?;

        loop {
            self.skip_spaces();
            match self.peek() {
                Some('+') => {
                    self.pos += 1;
                    let rhs = self.parse_term()?;
                    value = Self::check(value + rhs)?;
                }
                Some('-') => {
                    self.pos += 1;
                    let rhs = self.parse_term()?;
                    value = Self::check(value - rhs)?;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    fn parse_term(&mut self) -> Result<f64, ParseError> {
        let mut value = self.parse_unary()?;

        loop {
            self.skip_spaces();
            match self.peek() {
                Some('*') => {
                    self.pos += 1;
                    let rhs = self.parse_unary()?;
                    value = Self::check(value * rhs)?;
                }
                Some('/') => {
                    self.pos += 1;
                    let rhs = self.parse_unary()?;
                    if rhs == 0.0 {
                        return Err(ParseError::DivisionByZero);
                    }
                    value = Self::check(value / rhs)?;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    fn parse_unary(&mut self) -> Result<f64, ParseError> {
        self.skip_spaces();
        match self.peek() {
            Some('-') => {
                self.pos += 1;
                Self::check(-self.parse_unary()?)
            }
            Some('+') => {
                self.pos += 1;
                self.parse_unary()
            }
            Some('√') => {
                self.pos += 1;
                Self::check(self.parse_unary()?.sqrt())
            }
            _ => self.parse_power(),
        }
    }

    fn parse_power(&mut self) -> Result<f64, ParseError> {
        let base = self.parse_postfix()?;
        self.skip_spaces();

        if self.peek() == Some('^') {
            self.pos += 1;
            let exponent = self.parse_unary()?;
            return Self::check(base.powf(exponent));
        }

        Ok(base)
    }

    fn parse_postfix(&mut self) -> Result<f64, ParseError> {
        let mut value = self.parse_primary()?;

        loop {
            self.skip_spaces();
            match self.peek() {
                Some('%') => {
                    self.pos += 1;
                    value /= 100.0;
                }
                Some('√') => {
                    self.pos += 1;
                    value = value.sqrt();
                }
                _ => break,
            }
        }

        Self::check(value)
    }

    fn parse_primary(&mut self) -> Result<f64, ParseError> {
        self.skip_spaces();

        match self.peek() {
            Some(c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some('(') => {
                self.pos += 1;
                let value = self.parse_expr()?;
                self.skip_spaces();
                if self.bump() == Some(')') {
                    Ok(value)
                } else {
                    Err(ParseError::MissingClosingParen)
                }
            }
            Some(c) => Err(ParseError::UnexpectedChar(c)),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn parse_number(&mut self) -> Result<f64, ParseError> {
        let start = self.pos;
        let mut digits = 0;
        let mut seen_dot = false;

        while let Some(current) = self.peek() {
            match current {
                '0'..='9' => {
                    digits += 1;
                    self.pos += 1;
                }
                '.' if !seen_dot => {
                    seen_dot = true;
                    self.pos += 1;
                }
                _ => break,
            }
        }

        if digits == 0 {
            return Err(ParseError::InvalidNumber);
        }

        let text: String = self.chars[start..self.pos].iter().collect();
        text.parse::<f64>().map_err(|_| ParseError::InvalidNumber)
    }
}

/// Evalua una expresion completa. Devuelve el resultado o el error encontrado.
pub fn evaluate(expression: &str) -> Result<f64, ParseError> {
    if expression.trim().is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    let mut parser = Parser::new(expression);
    let value = parser.parse_expr()?;
    parser.expect_end()?;

    Ok(value)
}

/// Da formato al resultado evitando ceros de relleno y ruido de coma flotante.
pub fn format_number(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    let absolute = value.abs();

    if absolute < 1e-10 {
        return format!("{value:.6e}");
    }

    if absolute >= 1e15 {
        return format!("{value}");
    }

    let mut text = format!("{value:.10}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(expression: &str) -> f64 {
        evaluate(expression).expect("la expresion deberia ser valida")
    }

    #[test]
    fn basic_operations() {
        assert_eq!(value("2+3"), 5.0);
        assert_eq!(value("10-4"), 6.0);
        assert_eq!(value("6*7"), 42.0);
        assert_eq!(value("9/2"), 4.5);
    }

    #[test]
    fn operator_precedence() {
        assert_eq!(value("2+3*4"), 14.0);
        assert_eq!(value("(2+3)*4"), 20.0);
        assert_eq!(value("10-2-3"), 5.0);
        assert_eq!(value("100/5/2"), 10.0);
    }

    #[test]
    fn unary_minus() {
        assert_eq!(value("-5"), -5.0);
        assert_eq!(value("3*-2"), -6.0);
        assert_eq!(value("3--2"), 5.0);
    }

    #[test]
    fn decimals() {
        assert_eq!(value("0.5+0.25"), 0.75);
        assert_eq!(value("1.5*2"), 3.0);
    }

    #[test]
    fn power_is_right_associative() {
        assert_eq!(value("2^3^2"), 512.0);
        assert_eq!(value("2^10"), 1024.0);
        assert_eq!(value("2^-2"), 0.25);
        assert_eq!(value("-2^2"), -4.0);
    }

    #[test]
    fn percentage_is_postfix() {
        assert_eq!(value("50%"), 0.5);
        assert_eq!(value("200*10%"), 20.0);
        assert_eq!(value("10%+5"), 5.1);
    }

    #[test]
    fn sqrt_prefix_and_postfix() {
        assert_eq!(value("√9"), 3.0);
        assert_eq!(value("9√"), 3.0);
        assert_eq!(value("√(9+16)"), 5.0);
        // el postfijo se une antes que la potencia: 2^(sqrt(16))
        assert_eq!(value("2^16√"), 16.0);
        assert_eq!(value("√(2^10)"), 32.0);
    }

    #[test]
    fn combined_expression() {
        assert_eq!(value("(2+3)^2*10%"), 2.5);
        assert_eq!(value("√144/12"), 1.0);
    }

    #[test]
    fn errors() {
        assert_eq!(evaluate("1/0"), Err(ParseError::DivisionByZero));
        assert_eq!(evaluate("√-4"), Err(ParseError::UndefinedResult));
        assert_eq!(evaluate("(2+3"), Err(ParseError::MissingClosingParen));
        assert_eq!(evaluate("(2+3))"), Err(ParseError::UnexpectedChar(')')));
        assert_eq!(evaluate(""), Err(ParseError::EmptyExpression));
        assert_eq!(evaluate("2 3"), Err(ParseError::UnexpectedChar('3')));
        assert_eq!(evaluate("5+"), Err(ParseError::UnexpectedEnd));
    }

    #[test]
    fn formatting() {
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(5.0), "5");
        assert_eq!(format_number(0.1 + 0.2), "0.3");
        assert_eq!(format_number(1024.0), "1024");
        assert_eq!(format_number(-3.5), "-3.5");
    }
}
