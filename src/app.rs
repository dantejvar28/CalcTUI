use crate::expresion_parser::{evaluate, format_number};

pub struct App {
    pub input: String,
    pub result: String,
    pub error: bool,
    just_evaluated: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            result: String::from("0"),
            error: false,
            just_evaluated: false,
        }
    }

    fn is_valid_input(c: char) -> bool {
        matches!(
            c,
            '0'..='9' | '+' | '-' | '*' | '/' | '.' | '^' | '%' | '√' | '(' | ')'
        )
    }

    fn is_binary_operator(c: char) -> bool {
        matches!(c, '+' | '-' | '*' | '/' | '^')
    }

    /// Numero que se esta escribiendo en este momento (sin signo ni operador).
    fn current_number(&self) -> Option<String> {
        let mut number = String::new();
        for c in self.input.chars().rev() {
            if c.is_ascii_digit() || c == '.' {
                number.insert(0, c);
            } else {
                break;
            }
        }
        if number.is_empty() {
            None
        } else {
            Some(number)
        }
    }

    fn open_parens(&self) -> i32 {
        let mut open = 0;
        for c in self.input.chars() {
            match c {
                '(' => open += 1,
                ')' => open -= 1,
                _ => {}
            }
        }
        open
    }

    pub fn add_input(&mut self, c: char) {
        if !Self::is_valid_input(c) {
            return;
        }

        if self.just_evaluated {
            self.just_evaluated = false;
            if c.is_ascii_digit() || c == '.' || c == '(' || c == '√' {
                self.input.clear();
            }
        }

        if Self::is_binary_operator(c) {
            // se reemplaza el operador anterior, salvo que el nuevo sea un menos
            // (asi se puede escribir 2*-3, pero no 5--3)
            if let Some(last) = self.input.chars().last()
                && Self::is_binary_operator(last)
                && !(c == '-' && last != '-')
            {
                self.input.pop();
            }
        }

        if c == '.' && self.current_number().is_some_and(|n| n.contains('.')) {
            return;
        }

        if c == ')' && self.open_parens() <= 0 {
            return;
        }

        self.input.push(c);
    }

    pub fn backspace(&mut self) {
        self.input.pop();
        self.just_evaluated = false;
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.result = String::from("0");
        self.error = false;
        self.just_evaluated = false;
    }

    /// Invierte el signo del ultimo numero escrito.
    pub fn toggle_sign(&mut self) {
        if self.input.is_empty() {
            if let Ok(current) = self.result.parse::<f64>() {
                let negated = format_number(-current);
                self.result = negated.clone();
                self.input = negated;
                self.just_evaluated = true;
            }
            return;
        }

        let chars: Vec<char> = self.input.chars().collect();
        let mut start = chars.len();
        while start > 0 && (chars[start - 1].is_ascii_digit() || chars[start - 1] == '.') {
            start -= 1;
        }

        // Todavia no hay ningun numero: se escribe (o se saca) el menos del proximo.
        if start == chars.len() {
            if self.input.ends_with('-') {
                self.input.pop();
            } else {
                self.input.push('-');
            }
            self.just_evaluated = false;
            return;
        }

        // el menos anterior al numero es signo si estaba al principio o detras de un operador
        let has_sign = start > 0
            && chars[start - 1] == '-'
            && (start == 1 || Self::is_binary_operator(chars[start - 2]));

        let tail: String = chars[start..].iter().collect();
        self.input = if has_sign {
            let without_sign: String = chars[..start - 1].iter().collect();
            format!("{without_sign}{tail}")
        } else {
            let head: String = chars[..start].iter().collect();
            format!("{head}-{tail}")
        };

        self.just_evaluated = false;
    }

    pub fn evaluate(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }

        match evaluate(&self.input) {
            Ok(value) => {
                let text = format_number(value);
                self.result = text.clone();
                self.input = text;
                self.error = false;
                self.just_evaluated = true;
            }
            Err(err) => {
                self.result = err.to_string();
                self.error = true;
                self.just_evaluated = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app_with(text: &str) -> App {
        let mut app = App::new();
        for c in text.chars() {
            app.add_input(c);
        }
        app
    }

    #[test]
    fn ignores_invalid_chars() {
        let mut app = App::new();
        app.add_input('a');
        app.add_input('7');
        assert_eq!(app.input, "7");
    }

    #[test]
    fn replaces_repeated_operators() {
        assert_eq!(app_with("5++3").input, "5+3");
        assert_eq!(app_with("5**").input, "5*");
        assert_eq!(app_with("5-").input, "5-");
    }

    #[test]
    fn allows_minus_after_an_operator() {
        assert_eq!(app_with("2*-3").input, "2*-3");
        assert_eq!(app_with("2^-3").input, "2^-3");
    }

    #[test]
    fn allows_only_one_dot_per_number() {
        assert_eq!(app_with("1.2.3").input, "1.23");
        assert_eq!(app_with("1.2+3.4").input, "1.2+3.4");
    }

    #[test]
    fn ignores_unmatched_closing_paren() {
        assert_eq!(app_with("2)").input, "2");
        assert_eq!(app_with("(2)").input, "(2)");
    }

    #[test]
    fn evaluate_fills_result_and_input() {
        let mut app = app_with("(2+3)^2");
        app.evaluate();
        assert_eq!(app.result, "25");
        assert_eq!(app.input, "25");
        assert!(!app.error);
    }

    #[test]
    fn chains_from_result_with_an_operator() {
        let mut app = app_with("10+5");
        app.evaluate();
        app.add_input('/');
        app.add_input('2');
        app.evaluate();
        assert_eq!(app.result, "7.5");
    }

    #[test]
    fn starts_fresh_when_typing_after_equals() {
        let mut app = app_with("3*3");
        app.evaluate();
        app.add_input('4');
        assert_eq!(app.input, "4");
    }

    #[test]
    fn shows_errors_without_losing_the_expression() {
        let mut app = app_with("5/0");
        app.evaluate();
        assert!(app.error);
        assert_eq!(app.input, "5/0");
    }

    #[test]
    fn backspace_and_clear() {
        let mut app = app_with("123");
        app.backspace();
        assert_eq!(app.input, "12");
        app.clear();
        assert_eq!(app.input, "");
        assert_eq!(app.result, "0");
    }

    #[test]
    fn toggle_sign_on_last_number() {
        fn toggled(text: &str) -> String {
            let mut app = app_with(text);
            app.toggle_sign();
            app.input
        }

        assert_eq!(toggled("5"), "-5");
        assert_eq!(toggled("-5"), "5");
        assert_eq!(toggled("2*3"), "2*-3");
        assert_eq!(toggled("2*-3"), "2*3");
        assert_eq!(toggled("5-3"), "5--3");
    }
}
