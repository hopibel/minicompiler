enum Token {
    EOS, // End of string
    ZERO,
    ONE,
    TWO,
    OPEN,
    CLOSE,
    PLUS,
    MULT,
}

struct Tokenizer {
    s: Vec<char>,  // NOTE: can't use Chars iterator because input string isn't owned by the struct
    pos: usize,
    token: Token,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        // NOTE: struct members private and immutable by default
        let mut tokenizer = Tokenizer {
            s: input.chars().collect(),
            pos: 0,
            token: Token::EOS,
        };
        tokenizer.nextToken();
        return tokenizer;
    }

    fn nextToken(&mut self) {
        loop {
            if self.pos >= self.s.len() {
                self.token = Token::EOS;
                break;
            }
            // match symbol to token
            let token = match self.s[self.pos] {
                '0' => Some(Token::ZERO),
                '1' => Some(Token::ONE),
                '2' => Some(Token::TWO),
                '(' => Some(Token::OPEN),
                ')' => Some(Token::CLOSE),
                '+' => Some(Token::PLUS),
                '*' => Some(Token::MULT),
                _ => None,
            };
            match token {
                Some(token) => { // set current token
                    self.token = token;
                    break;
                },
                None => (),  // skip all other symbols
            }
            self.pos += 1;
        }
    }

    fn scan() {
        unimplemented!()
    }

    fn show() {
        unimplemented!()
    }

    fn showTok() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_running() {
        assert_eq!(2, 2);
    }
}
