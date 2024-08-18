/*
The following specification lists all of the token types that are used by
Wab:

Reserved Keywords:
    VAR     : 'var'
    PRINT   : 'print'
    IF      : 'if'
    ELSE    : 'else'
    WHILE   : 'while'
    FUNC    : 'func'
    RETURN  : 'return'

Identifiers/Names:
    NAME    : Text starting with a letter followed by any number
              number of letters or digits.
              Examples:  'abc' 'ABC' 'abc123'

Literals:
    INTEGER :  123

Symbols and Operators:
    PLUS     : '+'
    TIMES    : '*'
    LT       : '<'
    EQ       : '=='
    ASSIGN   : '='
    SEMI     : ';'
    LPAREN   : '('
    RPAREN   : ')'
    LBRACE   : '{'
    RBRACE   : '}'
    COMMA    : ','

Comments:  To be ignored
    //             Skips the rest of the line
*/

/*
Approach:

The token types are defined in an enum and then further classified in separate hash maps for symbols and reserved words.

The tokens are defined using a Token struct which has the token type and the actual string token.

The tokenize function accepts a generic which should implement BufRead. This is so that we don't have to read the entire file into memory and we can use a buffered reader
to iterate and tokenize. The reader is then converted into a Peekable to be able to peek ahead for certain tokenization. It then just goes char by char
and puts the tokens it finds into a Vec<Token> which is then returned.

*/

use std::{collections::HashMap, io::BufRead};

use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq, Debug)]
enum TokenType {
    // Reserved Keywords
    Var,
    Print,
    If,
    Else,
    While,
    Func,
    Return,

    // Identifiers/Names
    Name,

    // Literals
    Integer,

    // Symbols and Operators
    Plus,
    Times,
    Lt,
    Eq,
    Assign,
    Semi,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
}

lazy_static! {
    static ref SYMBOL_TO_TOKEN_MAP: HashMap<char, TokenType> = {
        let mut m = HashMap::new();
        m.insert('+', TokenType::Plus);
        m.insert('*', TokenType::Times);
        m.insert('<', TokenType::Lt);
        m.insert(';', TokenType::Semi);
        m.insert('(', TokenType::Lparen);
        m.insert(')', TokenType::Rparen);
        m.insert('{', TokenType::Lbrace);
        m.insert('}', TokenType::Rbrace);
        m.insert(',', TokenType::Comma);
        m
    };
    static ref KEYWORD_TO_TOKEN_MAP: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("var", TokenType::Var);
        m.insert("print", TokenType::Print);
        m.insert("if", TokenType::If);
        m.insert("else", TokenType::Else);
        m.insert("while", TokenType::While);
        m.insert("func", TokenType::Func);
        m.insert("return", TokenType::Return);
        m
    };
}

#[derive(PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    token_str: String,
}

impl Token {
    fn new(token_type: TokenType, token_str: String) -> Self {
        Token {
            token_type,
            token_str,
        }
    }
}

fn tokenize<R: BufRead>(reader: R) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut char_iter = reader.bytes().map(|r| r.unwrap() as char).peekable();

    while let Some(c) = char_iter.next() {
        match c {
            c if SYMBOL_TO_TOKEN_MAP.contains_key(&c) => handle_symbol(&mut tokens, c),
            c if c.is_whitespace() => continue,
            c if c.is_ascii_digit() => handle_digit(&mut tokens, c, &mut char_iter),
            c if c.is_ascii_alphanumeric() => handle_alphanumeric(&mut tokens, c, &mut char_iter),
            '=' => handle_equals(&mut tokens, &mut char_iter),
            '/' if char_iter.next_if_eq(&'/').is_some() => {
                char_iter.find(|&s| s == '\n');
            }
            _ => panic!("Unsupported char: {}", c),
        }
    }

    tokens
}

fn handle_symbol(tokens: &mut Vec<Token>, c: char) {
    if let Some(&token_type) = SYMBOL_TO_TOKEN_MAP.get(&c) {
        tokens.push(Token::new(token_type, c.to_string()));
    }
}

fn handle_equals(
    tokens: &mut Vec<Token>,
    char_iter: &mut std::iter::Peekable<impl Iterator<Item = char>>,
) {
    if char_iter.next_if_eq(&'=').is_some() {
        tokens.push(Token::new(TokenType::Eq, "==".to_string()));
    } else {
        tokens.push(Token::new(TokenType::Assign, "=".to_string()));
    }
}

fn handle_digit(
    tokens: &mut Vec<Token>,
    c: char,
    char_iter: &mut std::iter::Peekable<impl Iterator<Item = char>>,
) {
    let mut num_str = String::from(c);

    while let Some(s) = char_iter.next_if(|&s| s.is_ascii_digit()) {
        num_str.push(s);
    }
    tokens.push(Token::new(TokenType::Integer, num_str));
}

fn handle_alphanumeric(
    tokens: &mut Vec<Token>,
    c: char,
    char_iter: &mut std::iter::Peekable<impl Iterator<Item = char>>,
) {
    let mut name = String::from(c);

    while let Some(s) = char_iter.next_if(|&s| s.is_ascii_alphanumeric()) {
        name.push(s);
    }
    let token_type = KEYWORD_TO_TOKEN_MAP
        .get(name.as_str())
        .copied()
        .unwrap_or(TokenType::Name);
    tokens.push(Token::new(token_type, name));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let input = "print 123 + xy;";
        let tokens = tokenize(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Print, "print".to_string()),
                Token::new(TokenType::Integer, "123".to_string()),
                Token::new(TokenType::Plus, "+".to_string()),
                Token::new(TokenType::Name, "xy".to_string()),
                Token::new(TokenType::Semi, ";".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_keywords() {
        let input = "var if else while func return";
        let tokens = tokenize(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Var, "var".to_string()),
                Token::new(TokenType::If, "if".to_string()),
                Token::new(TokenType::Else, "else".to_string()),
                Token::new(TokenType::While, "while".to_string()),
                Token::new(TokenType::Func, "func".to_string()),
                Token::new(TokenType::Return, "return".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_symbols() {
        let input = "+*<==();{}";
        let tokens = tokenize(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Plus, "+".to_string()),
                Token::new(TokenType::Times, "*".to_string()),
                Token::new(TokenType::Lt, "<".to_string()),
                Token::new(TokenType::Eq, "==".to_string()),
                Token::new(TokenType::Lparen, "(".to_string()),
                Token::new(TokenType::Rparen, ")".to_string()),
                Token::new(TokenType::Semi, ";".to_string()),
                Token::new(TokenType::Lbrace, "{".to_string()),
                Token::new(TokenType::Rbrace, "}".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_comments() {
        let input = "print 123; // This is a comment\nvar x = 10;";
        let tokens = tokenize(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Print, "print".to_string()),
                Token::new(TokenType::Integer, "123".to_string()),
                Token::new(TokenType::Semi, ";".to_string()),
                Token::new(TokenType::Var, "var".to_string()),
                Token::new(TokenType::Name, "x".to_string()),
                Token::new(TokenType::Assign, "=".to_string()),
                Token::new(TokenType::Integer, "10".to_string()),
                Token::new(TokenType::Semi, ";".to_string()),
            ]
        );
    }
}
