const SINGLE: char = '\'';
const DOUBLE: char = '\"';
const ESCAPE: char = '\\';

fn parse_quotes(param: String) -> String {
    let mut quotes_stack: Vec<char> = Vec::new();
    let mut escape: bool = false;
    let mut buf: String = String::new();
    for c in param.chars() {
        match c {
            SINGLE => {}
            DOUBLE => {}
            _ => {
                buf.push(c);
            }
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use crate::quotations::parse_quotes;

    #[test]
    fn double_quotes() {
        let a = "\'this is a      test \\n a\'";
        let b = parse_quotes(a.to_string());
        assert_eq!(r"this is a      test \n a", b);
    }
}
