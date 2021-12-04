use std::io::{Error, ErrorKind, Read};

pub struct FakeStdin {
    buf: Vec<u8>,
    pos: usize,
}

impl FakeStdin {
    pub fn new(buf: &[u8]) -> Self {
        Self {
            buf: buf.to_vec(),
            pos: 0,
        }
    }
}

impl Read for FakeStdin {
    fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!()
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        if self.pos >= self.buf.len() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Unexpected end if input",
            ));
        }

        // read until new line or eof
        let mut size = 0;

        while self.pos < self.buf.len() {
            let ch = char::from(self.buf[self.pos]);
            self.pos += 1;
            if ch == '\n' {
                if buf.len() == 0 {
                    continue;
                }
                break;
            }

            buf.push(ch);
            size += 1;
        }

        Ok(size)
    }
}
