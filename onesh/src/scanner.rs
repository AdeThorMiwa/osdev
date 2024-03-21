#[derive(Clone)]
enum CursorPosition {
    Negative,
    Positive(usize),
    Eof,
}

pub struct Scanner<'a> {
    buf: &'a str,
    bufsize: usize,
    cursor: CursorPosition,
}

impl<'a> Scanner<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self {
            buf,
            bufsize: buf.len(),
            cursor: CursorPosition::Positive(0),
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if let CursorPosition::Positive(cursor) = self.cursor {
            let next_char = self.buf.chars().nth(cursor);

            if cursor + 1 == self.bufsize {
                self.cursor = CursorPosition::Eof;
            } else {
                self.cursor = CursorPosition::Positive(cursor + 1);
            }

            next_char
        } else {
            None
        }
    }

    pub fn reverse_peek(&mut self) {
        if let CursorPosition::Positive(pos) = self.cursor {
            if pos < 1 {
                self.cursor = CursorPosition::Negative;
            } else {
                self.cursor = CursorPosition::Positive(pos - 1)
            }
        }
    }

    pub fn peek(&self) -> Option<char> {
        let mut peekable_cur = self.cursor.clone();
        if let CursorPosition::Negative = peekable_cur {
            peekable_cur = CursorPosition::Positive(0);
        }

        let index = match peekable_cur {
            CursorPosition::Positive(pos) => pos,
            CursorPosition::Negative => 1,
            CursorPosition::Eof => return None,
        };

        self.buf.chars().nth(index)
    }

    pub fn skip_whitespaces(&mut self) {
        while let Some(char) = self.peek() {
            if char == ' ' || char == '\t' {
                self.next();
                continue;
            }

            break;
        }
    }
}
