use std::collections::HashMap;
use std::fs;
use std::io;
use std::iter::Peekable;
use std::str::Chars;

mod monoisotopic;
pub use monoisotopic::Monoisotopic;

pub struct Fasta<'s> {
    input: Peekable<Chars<'s>>,
}

impl<'s> Fasta<'s> {
    pub fn new(s: &'s str) -> Self {
        Fasta {
            input: s.chars().peekable(),
        }
    }

    pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> io::Result<HashMap<String, String>> {
        let s = fs::read_to_string(path)?;
        let p = Fasta::new(&s);
        Ok(p.read())
    }

    fn read_ident(input: &mut Peekable<Chars<'s>>) -> String {
        input
            .take_while(|&ch| ch != '\n')
            .filter(|&ch| !char::is_whitespace(ch))
            .collect()
    }

    fn read_sequence(input: &mut Peekable<Chars<'s>>) -> String {
        input
            .take_while(|&ch| ch != '>')
            .filter(|&ch| !char::is_whitespace(ch))
            .collect()
    }

    pub fn read(mut self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        while let Some(ch) = self.input.peek() {
            if ch == &'>' {
                self.input.next().unwrap();
            }
            let id = Self::read_ident(&mut self.input);
            let seq = Self::read_sequence(&mut self.input);
            map.insert(id, seq);
        }
        map
    }
}