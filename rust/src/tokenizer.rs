// derive implements <Token> == <Token> comparisons
#[derive(PartialEq)]
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
    fn new(input: &str) -> Self {
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
                    self.pos += 1;
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

    #[test]
    fn eos() {
        let mut t = Tokenizer::new("");
        assert!(matches!(t.token, Token::EOS));
        t.nextToken();
        assert!(matches!(t.token, Token::EOS));
    }

    #[test]
    fn skip_unknown() {
        let mut t = Tokenizer::new("13+");
        assert!(matches!(t.token, Token::ONE));
        t.nextToken();
        assert!(matches!(t.token, Token::PLUS));
        t.nextToken();
        assert!(matches!(t.token, Token::EOS));
        t.nextToken();
    }

    #[test]
    fn tokenize() {
        let mut t = Tokenizer::new("012()+*");
        assert!(matches!(t.token, Token::ZERO));
        t.nextToken();
        assert!(matches!(t.token, Token::ONE));
        t.nextToken();
        assert!(matches!(t.token, Token::TWO));
        t.nextToken();
        assert!(matches!(t.token, Token::OPEN));
        t.nextToken();
        assert!(matches!(t.token, Token::CLOSE));
        t.nextToken();
        assert!(matches!(t.token, Token::PLUS));
        t.nextToken();
        assert!(matches!(t.token, Token::MULT));
        t.nextToken();
        assert!(matches!(t.token, Token::EOS));
    }
}
