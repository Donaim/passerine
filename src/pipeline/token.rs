use crate::utils::annotation::Ann;
use crate::vm::data::Data;
use crate::vm::local::Local;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    // Delimiterss
    OpenBracket,
    CloseBracket,
    Sep,

    // Lambda
    Assign,

    // Datatypes
    Symbol(Local),
    Boolean(Data),
}

type Consume = Option<(Token, usize)>;

impl Token {
    pub fn from(source: &str) -> Consume {
        // check all functions
        // but are closures really the way to go?
        // also, maybe use array rather than vec?
        // also, there's no gaurantee I remember to closure-wrap all the functions
        // probably a more idiomatic way tbh
        let rules: Vec<Box<dyn Fn(&str) -> Consume>> = vec![
            // higher up in order = higher precedence
            // think 'or' as symbol or 'or' as operator
            // static
            Box::new(|s| Token::open_bracket(s) ),
            Box::new(|s| Token::close_bracket(s)),
            Box::new(|s| Token::assign(s)       ),

            // option
            Box::new(|s| Token::sep(s)    ),
            Box::new(|s| Token::boolean(s)),

            // keep this @ the bottom, lmao
            Box::new(|s| Token::symbol(s) ),
        ];

        // maybe some sort of map reduce?
        let mut best = None;

        // check longest
        for rule in &rules {
            if let Some((k, c)) = rule(source) {
                match best {
                    None                  => best = Some((k, c)),
                    Some((_, o)) if c > o => best = Some((k, c)),
                    Some(_)               => (),
                }
            }
        }

        return best;
    }

    // helpers

    fn literal(source: &str, literal: &str, kind: Token) -> Consume {
        if literal.len() > source.len() { return None }

        if &source[..literal.len()] == literal {
            return Some((kind, literal.len()));
        }

        return None;
    }

    // token classifiers

    fn symbol(source: &str) -> Consume {
        // for now, a symbol is one or more ascii alphanumerics
        let mut len = 0;

        for char in source.chars() {
            if !char.is_ascii_alphanumeric() {
                break;
            }
            len += 1;
        }

        return match len {
            0 => None,
            // TODO: make sure that symbol name is correct
            l => Some((Token::Symbol(Local::new(source[..l].to_string())), l)),
        };
    }

    fn open_bracket(source: &str) -> Consume {
        return Token::literal(source, "{", Token::OpenBracket);
    }

    fn close_bracket(source: &str) -> Consume {
        return Token::literal(source, "}", Token::CloseBracket);

    }

    // NEXT: parse

    fn assign(source: &str) -> Consume {
        return Token::literal(source, "=", Token::Assign);
    }

    // the below pattern is pretty common...
    // but I'm not going to abstract it out, yet

    fn boolean(source: &str) -> Consume {
        if let Some(x) = Token::literal(source, "true", Token::Boolean(Data::Boolean(true))) {
            return Some(x);
        }

        if let Some(x) = Token::literal(source, "false", Token::Boolean(Data::Boolean(false))) {
            return Some(x);
        }

        return None;
    }

    fn sep(source: &str) -> Consume {
        match source.chars().next()? {
            '\n' | ';' => Some((Token::Sep, 1)),
            _          => None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnnToken {
    pub kind: Token,
    pub ann:  Ann,
}

impl AnnToken {
    pub fn new(kind: Token, ann: Ann) -> AnnToken {
        AnnToken { kind, ann }
    }
}

// cfg ain't working
#[cfg(test)]
mod test {
    use super::*;

    // each case tests the detection of a specific token type

    #[test]
    fn boolean() {
        assert_eq!(
            Token::from("true"),
            Some((Token::Boolean(Data::Boolean(true)), 4)),
        );

        assert_eq!(
            Token::from("false"),
            Some((Token::Boolean(Data::Boolean(false)), 5)),
        );
    }

    #[test]
    fn assign() {
        assert_eq!(
            Token::from("="),
            Some((Token::Assign, 1)),
        );
    }

    #[test]
    fn symbol() {
        assert_eq!(
            Token::from(""),
            None,
        );

        assert_eq!(
            Token::from("heck"),
            Some((Token::Symbol(Local::new("heck".to_string())), 4))
        );
    }

    #[test]
    fn sep() {
        assert_eq!(
            Token::from("\nheck"),
            Some((Token::Sep, 1)),
        );

        assert_eq!(
            Token::from("; heck"),
            Some((Token::Sep, 1)),
        );
    }
}
