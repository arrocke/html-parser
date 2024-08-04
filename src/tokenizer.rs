use std::{collections::VecDeque, fmt, mem};

pub struct Tokenizer<'a> {
    state: TokenizerState<'a>,
    pub tokens: VecDeque<Token>
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            state: TokenizerState::Data { input },
            tokens: VecDeque::new()
        }
    }

    fn step(&mut self) {
        let state = mem::replace(&mut self.state, TokenizerState::EOF);
        self.state = state.step(&mut self.tokens);
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.tokens.is_empty() && !matches!(self.state, TokenizerState::EOF) {
            self.step();
        }
        self.tokens.pop_front()
    }
}

#[derive(Debug)]
pub enum TagKind {
    Start,
    End
}

pub struct Attribute {
    pub name: String,
    pub value: String
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

pub struct Tag {
    pub kind: TagKind,
    pub name: String,
    pub self_closing: bool,
    pub attributes: Vec<Attribute>
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

impl Tag  {
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

pub struct Doctype {
    pub name: String,
    pub public_identifier: Option<String>,
    pub system_identifier: Option<String>,
    pub force_quirks: bool
}

impl fmt::Debug for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<!DOCTYPE {}", self.name)?;

        match &self.public_identifier {
            Some(identifier) => {
                write!(f, "PUBLIC '{}'", identifier)?;
                match &self.system_identifier {
                    Some(identifier) => write!(f, " '{}'>", identifier)?,
                    None => write!(f, ">")?
                }
            }
            None => {
                match &self.system_identifier {
                    Some(identifier) => write!(f, "SYSTEM '{}'>", identifier)?,
                    None => write!(f, ">")?
                }
            }
        }
        Ok(())
    }
}

impl Doctype {
    fn new() ->Doctype {
        Doctype {
            name: String::new(),
            public_identifier: None,
            system_identifier: None,
            force_quirks: false
        }
    }
}

#[derive(Debug)]
enum TokenizerState<'a> {
    AfterAttributeName { input: &'a str, tag: Tag, attribute: Attribute },
    AfterAttributeValueQuoted { input: &'a str, tag: Tag },
    AfterDoctypeName { input: &'a str, doctype: Doctype },
    AfterDoctypePublicKeyword { input:&'a str, doctype: Doctype },
    AfterDoctypeSystemKeyword { input:&'a str, doctype: Doctype },
    AfterDoctypePublicIdentifier { input:&'a str, doctype: Doctype },
    AfterDoctypeSystemIdentifier { input:&'a str, doctype: Doctype },
    AttributeName { input: &'a str, tag: Tag, attribute: Attribute },
    AttributeValueDoubleQuoted { input: &'a str, tag: Tag ,attribute: Attribute },
    AttributeValueSingleQuoted { input: &'a str, tag: Tag ,attribute: Attribute },
    AttributeValueUnquoted { input: &'a str, tag: Tag ,attribute: Attribute },
    BeforeAttributeName { input: &'a str, tag: Tag },
    BeforeAttributeValue { input: &'a str, tag: Tag ,attribute: Attribute },
    BeforeDoctypeName { input: &'a str },
    BeforeDoctypePublicIdentifier { input: &'a str, doctype: Doctype },
    BeforeDoctypeSystemIdentifier { input: &'a str, doctype: Doctype },
    BetweenDoctypePublicAndSystemIdentifiers { input: &'a str, doctype: Doctype },
    Data { input: &'a str },
    Doctype { input: &'a str },
    DoctypeName { input: &'a str, doctype: Doctype },
    DoctypePublicIdentifierDoubleQuoted { input: &'a str, doctype: Doctype },
    DoctypePublicIdentifierSingleQuoted { input: &'a str, doctype: Doctype },
    DoctypeSystemIdentifierDoubleQuoted { input: &'a str, doctype: Doctype },
    DoctypeSystemIdentifierSingleQuoted { input: &'a str, doctype: Doctype },
    EndTagOpen { input: &'a str },
    EOF,
    MarkupDeclarationOpen { input: &'a str },
    SelfClosingStartTag { input: &'a str, tag: Tag },
    TagName { input: &'a str, tag: Tag },
    TagOpen { input: &'a str },
}

pub enum Token {
    EOF,
    Char(char),
    Tag(Tag),
    Doctype(Doctype)
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::EOF => write!(f, "EOF"),
            Token::Char(ch) => write!(f, "{}", ch),
            Token::Tag(tag) => write!(f, "{:?}", tag),
            Token::Doctype(doctype) => write!(f, "{:?}", doctype),
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
            TokenizerState::AfterDoctypeName { input, doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::AfterDoctypeName { input: &input[1..], doctype },
                    Some('>') => { 
                        tokens.push_back(Token::Doctype(doctype));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    _ => {
                        if "PUBLIC" == &input[0..6].to_ascii_uppercase() {
                            TokenizerState::AfterDoctypePublicKeyword { input:&input[6..], doctype }
                        } else if "SYSTEM" == &input[0..6].to_ascii_uppercase() {
                            TokenizerState::AfterDoctypeSystemKeyword { input:&input[6..], doctype }
                        } else {
                            todo!()
                        }
                    }
                }
            }
            TokenizerState::AfterDoctypePublicKeyword { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeDoctypePublicIdentifier { input: &input[1..], doctype },
                    Some('"') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypePublicIdentifierDoubleQuoted { input: &input[1..], doctype }
                    }
                    Some('\'') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypePublicIdentifierSingleQuoted { input: &input[1..], doctype }
                    }
                    Some('>') => todo!(),
                    _ => todo!()
                }
            }
            TokenizerState::AfterDoctypePublicIdentifier { input, doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BetweenDoctypePublicAndSystemIdentifiers { input: &input[1..], doctype },
                    Some('>') => {
                        tokens.push_back(Token::Doctype(doctype));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    Some('"' | '\'') => todo!(),
                    _ => todo!()
                }
            }
            TokenizerState::AfterDoctypeSystemKeyword { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeDoctypeSystemIdentifier { input: &input[1..], doctype },
                    Some('"') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypeSystemIdentifierDoubleQuoted { input: &input[1..], doctype }
                    }
                    Some('\'') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypeSystemIdentifierSingleQuoted { input: &input[1..], doctype }
                    }
                    Some('>') => todo!(),
                    _ => todo!()
                }
            }
            TokenizerState::AfterDoctypeSystemIdentifier { input, doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::AfterDoctypeSystemIdentifier { input: &input[1..], doctype },
                    Some('>') => {
                        tokens.push_back(Token::Doctype(doctype));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    _ => todo!()
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
            TokenizerState::BeforeDoctypeName { input } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeDoctypeName { input: &input[1..] },
                    Some('>') => todo!(),
                    Some('\0') => todo!(),
                    Some(ch) => {
                        let mut doctype = Doctype::new();
                        doctype.name.push(ch.to_ascii_lowercase());
                        TokenizerState::DoctypeName { input: &input[1..], doctype }
                    }
                }
            }
            TokenizerState::BeforeDoctypePublicIdentifier { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeDoctypePublicIdentifier { input: &input[1..], doctype },
                    Some('"') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypePublicIdentifierDoubleQuoted { input: &input[1..], doctype }
                    }
                    Some('\'') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypePublicIdentifierSingleQuoted { input: &input[1..], doctype }
                    }
                    Some('>') => todo!(),
                    _ => todo!()
                }
            }
            TokenizerState::BeforeDoctypeSystemIdentifier { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeDoctypeSystemIdentifier { input: &input[1..], doctype },
                    Some('"') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypeSystemIdentifierDoubleQuoted { input: &input[1..], doctype }
                    }
                    Some('\'') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypeSystemIdentifierSingleQuoted { input: &input[1..], doctype }
                    }
                    Some('>') => todo!(),
                    _ => todo!()
                }
            }
            TokenizerState::BetweenDoctypePublicAndSystemIdentifiers { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BetweenDoctypePublicAndSystemIdentifiers { input: &input[1..], doctype },
                    Some('>') => {
                        tokens.push_back(Token::Doctype(doctype));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    Some('"') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypeSystemIdentifierDoubleQuoted { input: &input[1..], doctype }
                    }
                    Some('\'') => {
                        doctype.system_identifier = Some(String::new());
                        TokenizerState::DoctypeSystemIdentifierSingleQuoted { input: &input[1..], doctype }
                    }
                    _ => todo!()
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
            TokenizerState::Doctype { input } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::BeforeDoctypeName { input: &input[1..] },
                    Some('>') => TokenizerState::BeforeDoctypeName { input },
                    _ => todo!()
                }
            }
            TokenizerState::DoctypeName { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\t' | '\u{0a}' | '\u{0c}' | ' ') => TokenizerState::AfterDoctypeName { input: &input[1..], doctype },
                    Some('>') => { 
                        tokens.push_back(Token::Doctype(doctype));
                        TokenizerState::Data { input: &input[1..] }
                    },
                    Some('\0') => todo!(),
                    Some(ch) => {
                        doctype.name.push(ch.to_ascii_lowercase());
                        TokenizerState::DoctypeName { input: &input[1..], doctype }
                    }
                }
            }
            TokenizerState::DoctypePublicIdentifierDoubleQuoted { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('"') => TokenizerState::AfterDoctypePublicIdentifier { input: &input[1..], doctype },
                    Some('>') => todo!(),
                    Some('\0') => todo!(),
                    Some(ch) => {
                        match doctype.public_identifier {
                            Some(ref mut identifier) => identifier.push(ch),
                            None => panic!()
                        }
                        TokenizerState::DoctypePublicIdentifierDoubleQuoted { input: &input[1..], doctype }
                    }
                }
            }
            TokenizerState::DoctypePublicIdentifierSingleQuoted { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\'') => TokenizerState::AfterDoctypePublicIdentifier { input: &input[1..], doctype },
                    Some('>') => todo!(),
                    Some('\0') => todo!(),
                    Some(ch) => {
                        match doctype.public_identifier {
                            Some(ref mut identifier) => identifier.push(ch),
                            None => panic!()
                        }
                        TokenizerState::DoctypePublicIdentifierSingleQuoted { input: &input[1..], doctype }
                    }
                }
            }
            TokenizerState::DoctypeSystemIdentifierDoubleQuoted { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('"') => TokenizerState::AfterDoctypeSystemIdentifier { input: &input[1..], doctype },
                    Some('>') => todo!(),
                    Some('\0') => todo!(),
                    Some(ch) => {
                        match doctype.system_identifier {
                            Some(ref mut identifier) => identifier.push(ch),
                            None => panic!()
                        }
                        TokenizerState::DoctypeSystemIdentifierDoubleQuoted { input: &input[1..], doctype }
                    }
                }
            }
            TokenizerState::DoctypeSystemIdentifierSingleQuoted { input, mut doctype } => {
                match input.chars().nth(0) {
                    None => todo!(),
                    Some('\'') => TokenizerState::AfterDoctypeSystemIdentifier { input: &input[1..], doctype },
                    Some('>') => todo!(),
                    Some('\0') => todo!(),
                    Some(ch) => {
                        match doctype.system_identifier {
                            Some(ref mut identifier) => identifier.push(ch),
                            None => panic!()
                        }
                        TokenizerState::DoctypeSystemIdentifierSingleQuoted { input: &input[1..], doctype }
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
            TokenizerState::MarkupDeclarationOpen { input } => {
                if "--" == &input[0..2] {
                    todo!()
                } else if "DOCTYPE" == &input[0..7].to_uppercase() {
                    TokenizerState::Doctype { input: &input[7..] }
                } else if "[CDATA[" == &input[0..7] {
                    todo!()
                } else {
                    todo!()
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
                    Some('!') => TokenizerState::MarkupDeclarationOpen { input: &input[1..] } ,
                    Some('/') => TokenizerState::EndTagOpen { input: &input[1..] },
                    Some('?') => todo!(),
                    Some(ch) if matches!(ch, 'a'..'z' | 'A'..'Z') => TokenizerState::TagName { input, tag: Tag::new(TagKind::Start) },
                    Some(_) => todo!()
                }
            }
        }
    }
}
