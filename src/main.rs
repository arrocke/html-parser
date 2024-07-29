use std::{ascii, collections::VecDeque};

#[derive(Debug)]
enum TagKind {
    Start,
    End
}

#[derive(Debug)]
struct Tag {
    kind: TagKind,
    name: String
}

impl Tag {
    fn new(kind: TagKind) -> Tag {
        Tag {
            kind,
            name: String::new()
        }
    }
}

#[derive(Debug)]
enum TokenizerState<'a> {
    Data { input: &'a str },
    TagOpen { input: &'a str },
    TagName { input: &'a str, tag: Tag },
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
                    Some('/') => todo!(),
                    Some('?') => todo!(),
                    Some(ch) if matches!(ch, 'a'..'z' | 'A'..'Z') => TokenizerState::TagName { input, tag: Tag::new(TagKind::Start) },
                    Some(_) => todo!()
                }
            }
            TokenizerState::TagName { input, mut tag } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => todo!(),
                    Some('/') => todo!(),
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
        input: "t<div>"
    };

    let mut step = 1;
    while !matches!(state, TokenizerState::EOF) {
        state = state.step(&mut tokens);
        println!("{}: {:?}", step, state);
        step += 1;
    }

    println!("{:?}", tokens);
}
