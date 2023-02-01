use dom::{AttributesMap, ElementData, Node, NodeType};

use std::iter::Peekable;
use std::str::chars;

pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>,
    node_q: Vec<String>,
}

impl <'a> HtmlParser<'a> {
    pub fn new(full_html: &str) -> HtmlParser {
        HtmlParser {
            chars: full_html.chars().peekable(),
            node_q: Vec::new()
        }
    }

    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.chars.peek().is_some() {
            self.consume_while(char::is_whitespace);
            // New Element
            if(self.chars.peek().map_or(false, |c| *c == '<')) {
                self.chars.next();
                // Ending character
                if(self.chars.peek().map_or(false, |c| *c == '/')) {
                }

            }
        }
    }

    fn consume_while<F>(&mut self, condition: F) -> String 
        where F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        return result
    }
}

fn is_valid_tag_name(ch: char) -> bool {
    ch.is_digit(36)
}

fn is_valid_attr_name(c: char) -> bool {
    !is_excluded_name(c) && !is_control(c)
}

fn is_control(ch: char) -> bool {
    match ch {
        '\u{007F}' => true,
        c if c >= '\u{0000}' && c <= '\u{001F}' => true,
        c if c >= '\u{0080}' && c <= '\u{009F}' => true,
        _ => false,
    }
}

fn is_excluded_name(c: char) -> bool {
    match c {
        ' ' | '"' | '\'' | '>' | '/' | '=' => true,
        _ => false,
    }
}

fn is_valid_attr_value(c: char) -> bool {
    match c {
        ' ' | '"' | '\'' | '=' | '<' | '>' | '`' => false,
        _ => true,
    }
}