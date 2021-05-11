use crate::engine::node::Node;

#[derive(Debug)]
pub struct Parser {
    html: String,
    pos: usize,
}

impl Parser {
    pub fn new(html: String) -> Parser {
        Parser {
            html,
            pos: 0
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {Vec::new()}

    pub fn get_next_node(&mut self) -> Option<String> {
        let read= self.read_to('>').replace('\n' , "");
        if read.is_empty() {
            None
        } else {
            Some(read)
        }
    }

    pub fn read_to(&mut self, to: char) -> String {
        let mut string = String::new();
        while let Some(e) = self.next_char() {
            string.push(e);
            if e == to {
                break;
            }
        }

        return string;
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(e) = self.html[self.pos..].chars().next() {
            self.pos += 1;
            Some(e)
        } else {
            None
        }
    }
}

