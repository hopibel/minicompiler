use crate::tokenizer::{Tokenizer, Token};
use crate::ast;

pub struct Parser {
    t: Tokenizer,
}

impl Parser {
    pub fn new(s: &str) -> Self {
        Parser {
            t: Tokenizer::new(s),
        }
    }

    // E ::= T E'
    pub fn parse(&mut self) -> Result<Box<dyn ast::Exp>, String> {
        self.parse_e()
    }

    // NOTE: Errors mit ? propagieren ist viel übersichtlicher
    fn parse_e(&mut self) -> Result<Box<dyn ast::Exp>, String> {
        let tok = self.parse_t()?;
        self.parse_e2(tok)
    }

    // E' ::= + T E' |
    fn parse_e2(&mut self, left: Box<dyn ast::Exp>) -> Result<Box<dyn ast::Exp>, String> {
        // NOTE: if let is shorthand for match one, ignore rest
        if let Token::PLUS = self.t.token {
            self.t.next_token();
            let right = self.parse_t()?;
            self.parse_e2(Box::new(ast::PlusExp::new(left, right)))
        } else {
            Ok(left)
        }
    }

    // T ::= F T'
    fn parse_t(&mut self) -> Result<Box<dyn ast::Exp>, String> {
        let tok = self.parse_f()?;
        self.parse_t2(tok)
    }

    // T' ::= * F T' |
    fn parse_t2(&mut self, left: Box<dyn ast::Exp>) -> Result<Box<dyn ast::Exp>, String> {
        if let Token::MULT = self.t.token {
            self.t.next_token();
            let right = self.parse_f()?;
            self.parse_t2(Box::new(ast::MultExp::new(left, right)))
        } else {
            Ok(left)
        }
    }

    // F ::= N | (E)
    fn parse_f(&mut self) -> Result<Box<dyn ast::Exp>, String> {
        let pos = self.t.position();
        match self.t.token {
            Token::ZERO => {
                self.t.next_token();
                Ok(Box::new(ast::IntExp::new(0)))
            },
            Token::ONE => {
                self.t.next_token();
                Ok(Box::new(ast::IntExp::new(1)))
            },
            Token::TWO => {
                self.t.next_token();
                Ok(Box::new(ast::IntExp::new(2)))
            },
            Token::OPEN => {
                self.t.next_token();
                let e = self.parse_e()?;
                if self.t.token != Token::CLOSE {
                    Err(format!("Unclosed parenthesis at column {}", pos))
                } else {
                    self.t.next_token();
                    Ok(e)
                }
            },
            _ => Err(format!("Unexpected token '{}' at column {}", self.t.token.show(), pos)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() {
        assert_eq!(Parser::new("1").parse().unwrap().eval(), 1);
    }
}
