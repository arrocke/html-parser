use std::{ascii, collections::VecDeque};

#[derive(Debug)]
enum TagKind {
    Start,
    End
}

#[derive(Debug)]
struct Attribute {
    name: String,
    value: String
}

impl Attribute {
    fn new() -> Attribute {
        Attribute { name: String::new(), value: String::new() }
    }
}

#[derive(Debug)]
struct Tag {
    kind: TagKind,
    name: String,
    self_closing: bool,
    attributes: Vec<Attribute>
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
    Data { input: &'a str },
    TagOpen { input: &'a str },
    EndTagOpen { input: &'a str },
    TagName { input: &'a str, tag: Tag },
    SelfClosingStartTag { input: &'a str, tag: Tag },
    BeforeAttributeName { input: &'a str, tag: Tag },
    AttributeName { input: &'a str, tag: Tag, attribute: Attribute },
    AfterAttributeName { input: &'a str, tag: Tag, attribute: Attribute },
    BeforeAttributeValue { input: &'a str, tag: Tag ,attribute: Attribute },
    AttributeValueDoubleQuoted { input: &'a str, tag: Tag ,attribute: Attribute },
    AttributeValueSingleQuoted { input: &'a str, tag: Tag ,attribute: Attribute },
    AttributeValueUnquoted { input: &'a str, tag: Tag ,attribute: Attribute },
    AfterAttributeValueQuoted { input: &'a str, tag: Tag },
    EOF
}

#[derive(Debug)]
enum Token {
    EOF,
    Char(char),
    Tag(Tag)
}

impl<'a> TokenizerState<'a> {
    fn step(self, tokens: &mut VecDeque<Token>) -> TokenizerState<'a> {
        match self {
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
            TokenizerState::EndTagOpen { input } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some(ch) if matches!(ch, 'a'..'z' | 'A'..'Z') => TokenizerState::TagName { input, tag: Tag::new(TagKind::End) },
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
            },
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
            TokenizerState::BeforeAttributeValue { input, tag, attribute } => {
                match input.chars().nth(0) {
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeAttributeValue { input: &input[1..], tag, attribute },
                    Some('"') => TokenizerState::AttributeValueDoubleQuoted { input: &input[1..], tag, attribute },
                    Some('\'') => TokenizerState::AttributeValueSingleQuoted { input: &input[1..], tag, attribute },
                    Some('>') => todo!(),
                    _ => TokenizerState::AttributeValueUnquoted{ input, tag, attribute },
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
            },
            TokenizerState::EOF => {
                panic!("Cannot call TokenizerState.step with EOF state");
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
