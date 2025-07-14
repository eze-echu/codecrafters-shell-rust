const SINGLE: char = '\'';
const DOUBLE: char = '\"';
const ESCAPE: char = '\\';

pub fn parse_quotes(param: &str) -> String {
    let mut quotes_stack: Vec<char> = Vec::new();
    let mut escape: bool = false;
    let mut literal: bool = false;
    let mut double_quotes: bool = false;
    let mut buf: Vec<char> = vec![];
    for c in param.chars() {
        match c {
            SINGLE => {
                if literal {
                    literal = false;
                } else if double_quotes {
                    buf.push(c);
                } else {
                    literal = true;
                }
            }
            DOUBLE => {
                if literal {
                    buf.push(c);
                } else if escape && !double_quotes {
                    buf.push(c);
                    escape = false;
                } else {
                    double_quotes = !double_quotes;
                }
            }
            ESCAPE => {
                if literal {
                    buf.push(c);
                } else if !escape {
                    escape = true;
                } else {
                    buf.push(c);
                    escape = false;
                }
            }
            _ => {
                buf.push(c);
                if escape {
                    escape = false;
                }
            }
        }
    }
    buf.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::quotations::parse_quotes;

    #[test]
    fn single_quotes() {
        let a = "\'this is a      test \\n a\'";
        let b = parse_quotes(a);
        assert_eq!(r"this is a      test \n a", b);
    }
    #[test]
    fn double_quotes() {
        let a = r#""before\   after""#;
        let b = parse_quotes(a);
        assert_eq!(r#""before   after""#, b);
    }
}
