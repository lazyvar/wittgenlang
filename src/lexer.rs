use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    #[token("by")]
    By, // Function declaration
    #[token("is")]
    Is, // Variable assignment and equality
    #[token("forNow")]
    ForNow, // Mutable variable declaration
    #[token("change")]
    Change, // Variable reassignment
    #[token("to")]
    To, // Used with change
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("unless")]
    Unless,
    #[token("of")]
    Of, // Switch statement
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("produce")]
    Produce, // Early return
    #[token("import")]
    Import,
    #[token("module")]
    Module,
    #[token("from")]
    From,
    #[token("as")]
    As,
    #[token("see")]
    See, // Type definition
    #[token("record")]
    Record,
    #[token("variant")]
    Variant,
    #[token("write")]
    Write, // Print
    #[token("otherwise")]
    Otherwise, // Default case in switch

    // Types
    #[token("#")]
    TypePrefix,
    #[token("#Number")]
    NumberType,
    #[token("#Integer")]
    IntegerType,
    #[token("#Text")]
    TextType,
    #[token("#Decision")]
    DecisionType,
    #[token("#Nothing")]
    NothingType,
    #[token("#Bliss")]
    BlissType, // Void
    #[token("#Any")]
    AnyType,
    #[token("#List")]
    ListType,
    #[token("#Map")]
    MapType,
    #[token("#Tuple")]
    TupleType,
    #[token("#Result")]
    ResultType,
    #[token("#Shape")]
    ShapeType,

    // Boolean values
    #[token("yes")]
    Yes,
    #[token("no")]
    No,
    #[token("nothing")]
    Nothing,

    // Comment tokens
    #[token("!", priority = 2)]
    ExclamationMark, // Single line comment
    #[token("!doc")]
    DocComment,

    // Symbols
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("//")]
    IntegerDivide,
    #[token("%")]
    Modulo,
    #[token("^")]
    Power,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("..")]
    Range,
    #[token("&")]
    Ampersand, // String concatenation
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token("'")]
    Apostrophe, // For type/module function access
    #[token("|")]
    Pipe, // For union types
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token(">=")]
    GreaterEqual,
    #[token("<=")]
    LessEqual,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("=>")]
    FatArrow, // For lambdas

    // Literals
    #[regex(r"-?[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().map_err(|_| ()))]
    Number(f64),
    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),
    #[regex(r#""""[\s\S]*""""#, |lex| {
        let content = lex.slice();
        let trimmed = &content[3..content.len()-3];
        Ok::<String, ()>(trimmed.to_string())
    })]
    MultilineString(String),
    #[regex(r"[a-zA-Z][a-zA-Z0-9_\-]*", |lex| lex.slice().to_string())]
    Identifier(String), // Support for kebab-case identifiers

    // Whitespace and comments
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r"!([^\n]*)", logos::skip, priority = 3)]
    Comment,

    // End of file
    #[end]
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::MultilineString(s) => write!(f, "\"\"\"{}\"\"\"", s),
            Token::Identifier(name) => write!(f, "{}", name),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|result| result.unwrap_or(Token::EOF))
    }
} 