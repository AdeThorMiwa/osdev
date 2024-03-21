use crate::scanner::Scanner;

#[derive(Debug)]
pub struct Token {
    pub value: String,
}

pub struct Tokenizer<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(buf: &'a str) -> Self {
        let scanner = Scanner::new(buf);
        Self { scanner }
    }

    pub fn tokenize(&mut self) -> Option<Token> {
        let mut token = String::new();

        while let Some(char) = self.scanner.next() {
            match char {
                ' ' => break,
                '\t' => break,
                '\n' => {
                    if token.len() > 0 {
                        self.scanner.reverse_peek();
                    } else {
                        token.push(char);
                    }
                    break;
                }
                _ => token.push(char),
            }
        }

        if token.len() == 0 {
            return None;
        }

        Some(Token { value: token })
    }

    pub fn skip_whitespaces(&mut self) {
        self.scanner.skip_whitespaces()
    }
}
