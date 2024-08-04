use std::{collections::VecDeque, fmt};

#[derive(Debug)]
enum TagKind {
    Start,
    End
}

struct Attribute {
    name: String,
    value: String
}

impl Attribute {
    fn new() -> Attribute {
        Attribute { name: String::new(), value: String::new() }
    }
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value == "" {
            write!(f, "\"{}\"", self.name)
        } else {
            write!(f, "\"{}\"=\"{}\"", self.name, self.value)
        }
    }
}

struct Tag {
    kind: TagKind,
    name: String,
    self_closing: bool,
    attributes: Vec<Attribute>
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<")?;
        if matches!(self.kind, TagKind::End) {
            write!(f, "/")?;
        }
        write!(f, "{}", self.name)?;
        if self.attributes.len() > 0 {
            let attrs = self.attributes.iter().map(|attr| format!("{:?}", attr)).collect::<Vec<String>>().join(" ");
            write!(f, " {}", attrs)?;
        }
        if self.self_closing {
            write!(f, " />")
        } else {
            write!(f, ">")
        }
    }
}

impl Tag {
    fn new(kind: TagKind) -> Tag {
        Tag {
            kind,
            name: String::new(),
            self_closing: false,
            attributes: vec![]
        }
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        if self.attributes.iter().any(|attr| attr.name == attribute.name) {
            return
        }
        self.attributes.push(attribute);
    }
}

#[derive(Debug)]
enum TokenizerState<'a> {
    AfterAttributeName { input: &'a str, tag: Tag, attribute: Attribute },
    AfterAttributeValueQuoted { input: &'a str, tag: Tag },
    AttributeName { input: &'a str, tag: Tag, attribute: Attribute },
    AttributeValueDoubleQuoted { input: &'a str, tag: Tag ,attribute: Attribute },
    AttributeValueSingleQuoted { input: &'a str, tag: Tag ,attribute: Attribute },
    AttributeValueUnquoted { input: &'a str, tag: Tag ,attribute: Attribute },
    BeforeAttributeName { input: &'a str, tag: Tag },
    BeforeAttributeValue { input: &'a str, tag: Tag ,attribute: Attribute },
    Data { input: &'a str },
    EndTagOpen { input: &'a str },
    EOF,
    SelfClosingStartTag { input: &'a str, tag: Tag },
    TagName { input: &'a str, tag: Tag },
    TagOpen { input: &'a str },
}

enum Token {
    EOF,
    Char(char),
    Tag(Tag)
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::EOF => write!(f, "EOF"),
            Token::Char(ch) => write!(f, "{}", ch),
            Token::Tag(tag) => write!(f, "{:?}", tag),
        }
    }
}

impl<'a> TokenizerState<'a> {
    fn step(self, tokens: &mut VecDeque<Token>) -> TokenizerState<'a> {
        match self {
            TokenizerState::AfterAttributeName { input, mut tag, attribute } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::AfterAttributeName { input: &input[1..], tag, attribute },
                    Some('/') => TokenizerState::SelfClosingStartTag { input: &input[1..], tag } ,
                    Some('=') => TokenizerState::BeforeAttributeValue { input: &input[1..], tag, attribute },
                    Some('>') => {
                        tag.add_attribute(attribute);
                        tokens.push_back(Token::Tag(tag));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    Some(_) => {
                        tag.add_attribute(attribute);
                        let attribute = Attribute::new();
                        TokenizerState::AttributeName { input, tag, attribute }
                    }
                }
            }
            TokenizerState::AfterAttributeValueQuoted { input, tag } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeAttributeName { input: &input[1..], tag },
                    Some('/') => TokenizerState::SelfClosingStartTag { input: &input[1..], tag } ,
                    Some('>') => {
                        tokens.push_back(Token::Tag(tag));
                        TokenizerState::Data { input: &input[1..] }
                    }
                    Some(_) => todo!()
                }
            }
            TokenizerState::AttributeName { input, tag, mut attribute } => {
                match input.chars().nth(0) { 
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ' | '/' | '>') | None => {
                        TokenizerState::AfterAttributeName { input, tag, attribute }
                    },
                    Some('=') => TokenizerState::BeforeAttributeValue { input: &input[1..], tag, attribute },
                    Some('\0') => todo!(),
                    Some('"' | '\'' | '<') => todo!(),
                    Some(ch) => {
                        attribute.name.push(ch);
                        TokenizerState::AttributeName { input: &input[1..], tag, attribute }
                    }
                }
            }
            TokenizerState::AttributeValueDoubleQuoted { input, mut tag, mut attribute } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('"') => {
                        tag.add_attribute(attribute);
                        TokenizerState::AfterAttributeValueQuoted { input: &input[1..], tag }
                    }
                    Some('&') => todo!(),
                    Some('\0') => todo!(),
                    Some(ch) => {
                        attribute.value.push(ch);
                        TokenizerState::AttributeValueDoubleQuoted { input: &input[1..], tag, attribute }
                    }
                }
            }
            TokenizerState::AttributeValueSingleQuoted { input, mut tag, mut attribute } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\'') => {
                        tag.add_attribute(attribute);
                        TokenizerState::AfterAttributeValueQuoted { input: &input[1..], tag }
                    }
                    Some('&') => todo!(),
                    Some('\0') => todo!(),
                    Some(ch) => {
                        attribute.value.push(ch);
                        TokenizerState::AttributeValueSingleQuoted { input: &input[1..], tag, attribute }
                    }
                }
            }
            TokenizerState::AttributeValueUnquoted { input, mut tag, mut attribute } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => {
                        tag.add_attribute(attribute);
                        TokenizerState::BeforeAttributeName { input: &input[1..], tag }
                    },
                    Some('&') => todo!(),
                    Some('>') => {
                        tag.add_attribute(attribute);
                        tokens.push_back(Token::Tag(tag));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    Some('\0') => todo!(),
                    Some('"' | '\'' | '<' | '=' | '`') => todo!(),
                    Some(ch) => {
                        attribute.value.push(ch);
                        TokenizerState::AttributeValueUnquoted { input: &input[1..], tag, attribute }
                    }
                }
            }
            TokenizerState::BeforeAttributeName { input, tag } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeAttributeName { input: &input[1..], tag },
                    Some('/') => TokenizerState::SelfClosingStartTag { input: &input[1..], tag } ,
                    Some('>') => {
                        tokens.push_back(Token::Tag(tag));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    Some('=') => todo!(),
                    Some(_) => {
                        let attribute = Attribute::new();
                        TokenizerState::AttributeName { input, tag, attribute }
                    }
                }
            }
            TokenizerState::BeforeAttributeValue { input, tag, attribute } => {
                match input.chars().nth(0) {
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeAttributeValue { input: &input[1..], tag, attribute },
                    Some('"') => TokenizerState::AttributeValueDoubleQuoted { input: &input[1..], tag, attribute },
                    Some('\'') => TokenizerState::AttributeValueSingleQuoted { input: &input[1..], tag, attribute },
                    Some('>') => todo!(),
                    _ => TokenizerState::AttributeValueUnquoted{ input, tag, attribute },
                }
            }
            TokenizerState::Data { input } => {
                match input.chars().nth(0) {
                    None => {
                        tokens.push_back(Token::EOF);
                        TokenizerState::EOF
                    },
                    Some('&') => todo!(),
                    Some('<') => TokenizerState::TagOpen { input: &input[1..] },
                    Some('\0') => todo!(),
                    Some(ch) => {
                        tokens.push_back(Token::Char(ch));
                        TokenizerState::Data { input: &input[1..] }
                    }
                }
            }
            TokenizerState::EndTagOpen { input } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some(ch) if matches!(ch, 'a'..'z' | 'A'..'Z') => TokenizerState::TagName { input, tag: Tag::new(TagKind::End) },
                    Some(_) => todo!()
                }
            }
            TokenizerState::EOF => {
                panic!("Cannot call TokenizerState.step with EOF state");
            }
            TokenizerState::SelfClosingStartTag { input, mut tag } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('>') => {
                        tag.self_closing = true;
                        tokens.push_back(Token::Tag(tag));
                        TokenizerState::Data { input: &input[1..] }
                    }
                    Some(_) => todo!()
                }
            }
            TokenizerState::TagName { input, mut tag } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeAttributeName { input: &input[1..], tag },
                    Some('/') => TokenizerState::SelfClosingStartTag { input: &input[1..], tag } ,
                    Some('>') => {
                        tokens.push_back(Token::Tag(tag));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    Some('\0') => todo!(),
                    Some(ch) => {
                        tag.name.push(ch.to_ascii_lowercase());
                        TokenizerState::TagName { input: &input[1..], tag }
                    }
                }
            }
            TokenizerState::TagOpen { input } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('!') => todo!(),
                    Some('/') => TokenizerState::EndTagOpen { input: &input[1..] },
                    Some('?') => todo!(),
                    Some(ch) if matches!(ch, 'a'..'z' | 'A'..'Z') => TokenizerState::TagName { input, tag: Tag::new(TagKind::Start) },
                    Some(_) => todo!()
                }
            }
        }
    }
}

fn main() {
    println!("start");

    let mut tokens: VecDeque<Token> = VecDeque::new();
    let mut state = TokenizerState::Data {
        input: "<div><input id=check disabled type=\"checkbox\" name='valid'/></div>"
    };

    let mut step = 1;
    while !matches!(state, TokenizerState::EOF) {
        state = state.step(&mut tokens);
        println!("{}: {:?}", step, state);
        step += 1;
    }

    println!("{:?}", tokens);
}
