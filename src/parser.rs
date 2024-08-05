use crate::tokenizer::{TagKind, Token, Tokenizer};

pub fn parse(input: &str) {
    let tokenizer = Tokenizer::new(input);

    let mut doc = Document::new();

    let mut state = ParserState::Initial;
    let mut step = 1;
    for token in tokenizer {
        state = state.process_token(&mut doc, token);
        println!("{}: {:?}", step, state);
        step += 1;
    }
}

#[derive(Debug)]
pub struct DocumentType {
    pub name: String,
    pub public_id: String,
    pub system_id: String,
}

#[derive(Debug)]
pub struct Document {
    pub doctype: Option<DocumentType>,
}

impl Document {
    fn new() -> Document {
        Document { doctype: None }
    }
}

#[derive(Debug)]
enum ParserState {
    Initial,
    BeforeHtml,
}

impl ParserState {
    fn process_token(self, document: &mut Document, token: Token) -> ParserState {
        match self {
            ParserState::BeforeHtml => {
                match token {
                    Token::Char('\t' | '\u{0a}' | '\u{0c}' | '\u{0d}' | ' ') => {
                        ParserState::BeforeHtml
                    }
                    Token::Tag(tag) if matches!(tag.kind, TagKind::Start) && tag.name == "html" => {
                        todo!()
                    }
                    _ => todo!(),
                }
            }
            ParserState::Initial => {
                match token {
                    Token::Char('\t' | '\u{0a}' | '\u{0c}' | '\u{0d}' | ' ') => {
                        ParserState::Initial
                    }
                    Token::Doctype(doctype) => {
                        // TODO: handle more doctype besides just <!DOCTYPE html>
                        document.doctype = Some(DocumentType {
                            name: doctype.name,
                            public_id: doctype.public_identifier.unwrap_or_else(|| String::new()),
                            system_id: doctype.system_identifier.unwrap_or_else(|| String::new()),
                        });
                        ParserState::BeforeHtml
                    }
                    _ => todo!(),
                }
            }
        }
    }
}
