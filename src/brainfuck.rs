pub struct Interpreter {
    buf: std::vec::Vec<u8>,
    pos: usize,
}

#[derive(Debug)]
pub enum InterpretationError {
    NonAsciiCode,
    UnmatchedRightBracket,
    UnmatchedLeftBracket,
}

fn find_closing(interpretable: &[u8], from: usize) -> std::result::Result<usize, InterpretationError> {
    let mut counter = 0;
    for i in from..interpretable.len() {
        let ch = unsafe { interpretable.get_unchecked(i) }.to_owned() as char;
        counter += match ch {
            '[' => 1,
            ']' => -1,
            _ => 0
        };
        if counter == 0 {
            return Ok(i);
        }
    }
    return Err(InterpretationError::UnmatchedLeftBracket);
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut buf: std::vec::Vec<u8> = Vec::with_capacity(30000);
        buf.resize(buf.capacity(), 0);
        return Interpreter { buf, pos: 0 };
    }

    pub fn interpret(&mut self, mut source_code: String)
                     -> std::result::Result<String, InterpretationError> {
        source_code.retain(|c| ['+', '-', '<', '>', '.', '[', ']'].contains(&c));
        if !source_code.is_ascii() {
            return Err(InterpretationError::NonAsciiCode);
        }
        let interpretable = source_code.as_bytes();
        let mut brackets = std::collections::LinkedList::new();
        let mut result = String::new();
        let mut i = 0;
        while i < interpretable.len() {
            let ch = unsafe { interpretable.get_unchecked(i) }.to_owned() as char;
            i = match ch {
                '+' => {
                    self.inc();
                    i + 1
                }
                '-' => {
                    self.dec();
                    i + 1
                }
                '>' => {
                    self.move_right();
                    i + 1
                }
                '<' => {
                    self.move_left();
                    i + 1
                }
                '.' => {
                    result.push(self.buf[self.pos] as char);
                    i + 1
                }
                '[' => {
                    let closing = find_closing(interpretable, i)?;
                    if self.buf[self.pos] == 0 {
                        closing + 1
                    } else {
                        brackets.push_back(i);
                        i + 1
                    }
                }
                ']' => {
                    match brackets.pop_back() {
                        Some(position) => position,
                        None => return Err(InterpretationError::UnmatchedRightBracket)
                    }
                }
                _ => i + 1
            }
        }
        Ok(result)
    }

    fn inc(&mut self) {
        self.buf[self.pos] += 1;
    }

    fn dec(&mut self) {
        self.buf[self.pos] -= 1;
    }

    fn move_left(&mut self) {
        self.pos -= 1;
    }

    fn move_right(&mut self) {
        self.pos += 1;
    }
}
