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
    pub fn parse(&mut self) -> Option<Box<dyn ast::Exp>> {
        self.parse_e()
    }
    fn parse_e(&mut self) -> Option<Box<dyn ast::Exp>> {
        let tok = self.parse_t();
        if let Some(tok) = tok {
            self.parse_e2(tok)
        } else {
            None
        }
    }

    // E' ::= + T E' |
    fn parse_e2(&mut self, left: Box<dyn ast::Exp>) -> Option<Box<dyn ast::Exp>> {
        // NOTE: if let is shorthand for match one, ignore rest
        if let Token::PLUS = self.t.token {
            self.t.next_token();
            let right = self.parse_t();
            if let Some(right) = right {
                self.parse_e2(Box::new(ast::PlusExp::new(left, right)))
            } else {
                right
            }
        } else {
            Some(left)
        }
    }

    // T ::= F T'
    fn parse_t(&mut self) -> Option<Box<dyn ast::Exp>> {
        let tok = self.parse_f();
        if let Some(tok) = tok {
            self.parse_t2(tok)
        } else {
            None
        }
    }

    // T' ::= * F T' |
    fn parse_t2(&mut self, left: Box<dyn ast::Exp>) -> Option<Box<dyn ast::Exp>> {
        if let Token::MULT = self.t.token {
            self.t.next_token();
            let right = self.parse_f();
            if let Some(right) = right {
                self.parse_t2(Box::new(ast::MultExp::new(left, right)))
            } else {
                None
            }
        } else {
            Some(left)
        }
    }

    // F ::= N | (E)
    fn parse_f(&mut self) -> Option<Box<dyn ast::Exp>> {
        match self.t.token {
            Token::ZERO => {
                self.t.next_token();
                Some(Box::new(ast::IntExp::new(0)))
            },
            Token::ONE => {
                self.t.next_token();
                Some(Box::new(ast::IntExp::new(1)))
            },
            Token::TWO => {
                self.t.next_token();
                Some(Box::new(ast::IntExp::new(2)))
            },
            Token::OPEN => {
                self.t.next_token();
                let e = self.parse_e();
                if let None = e {
                    e
                } else if self.t.token != Token::CLOSE {
                    None
                } else {
                    self.t.next_token();
                    e
                }
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_tests() {
        todo!("port parser integration tests!")
    }

    #[test]
    fn parse_int() {
        assert_eq!(Parser::new("1").parse().unwrap().eval(), 1);
    }
}
