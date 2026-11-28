use std::f64::consts::{E, PI};

enum Expr {

    Constant(f64),
    Variable,
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Tanh(Box<Expr>),
    Exp(Box<Expr>),
}

#[derive(PartialEq)]
enum Token {
    Constant(f64),
    Variable,
    Plus, Minus, Star, Slash, Caret, 

    Sin, Cos, Tan, Tanh, Exp,
    OpenParen, ClosedParen,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Result<&Token, String> {
        if self.pos < self.tokens.len() {
            Ok(&self.tokens[self.pos])
        } else {
            Err("Past length".to_string())

        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}

fn eval(e: &Expr, x: f64) -> f64 {
    match e {
        Expr::Constant(n) => *n,
        Expr::Variable => x,
        Expr::Add(a, b) => eval(a, x) + eval(b, x),
        Expr::Mul(a, b) => eval(a, x) * eval(b, x),
        Expr::Pow(a, b) => eval(a, x).powf(eval(b, x)),
        Expr::Sin(n) => eval(n, x).sin(),
        Expr::Cos(n) => eval(n, x).cos(),
        Expr::Tan(n) => eval(n, x).tan(),
        Expr::Tanh(n) => eval(n, x).tanh(),
        Expr::Exp(n) => eval(n, x).exp(),
        Expr::Sub(a, b) => eval(a, x) - eval(b, x),
        Expr::Div(a, b) => eval(a, x) / eval(b, x),
        Expr::Neg(n) => eval(n, x) * -1.0,
    }
}

fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = s.chars().collect();
    let mut tokens: Vec<Token> = vec![];

    let mut i = 0;
    while i < s.chars().count() {
        let c = s.chars().nth(i).unwrap();
        match c {
            '+' => { tokens.push(Token::Plus); i += 1},
            '-' => {tokens.push(Token::Minus); i += 1},
            '*' => {tokens.push(Token::Star); i += 1},
            '/' => {tokens.push(Token::Slash); i += 1},
            '^' => {tokens.push(Token::Caret); i += 1},
            '(' => {tokens.push(Token::OpenParen); i += 1},
            ')' => {tokens.push(Token::ClosedParen); i += 1},
            ' ' | '\t' | '\n' => i += 1,
            '0'..='9' | '.' =>  { 
                            let mut end = i;
                            let mut decimal = 0.0;
                            let mut start: f64 = 0.0;
                            let mut dec = false;
                            while end + 1 < chars.len() && (chars[end + 1].is_ascii_digit() || chars[end + 1] == '.') {
                                if chars[end + 1] == '.' {
                                    dec = true;
                                } else if dec {
                                    decimal /= 10 as f64;
                                    decimal += 0.1 * chars[end + 1].to_digit(10).unwrap() as f64;
                                } else {
                                    start *= 10.0;
                                    start += chars[end + 1].to_digit(10).unwrap() as f64;
                                }
                                end += 1;
                            }
                            tokens.push(Token::Constant(start + decimal));
                            i = end + 2;
                        }

            c if c.is_ascii_alphabetic() => {
                        let mut end = i;
                        while end + 1 < s.chars().count() && chars[end + 1].is_ascii_alphabetic() {
                            end += 1;
                        }
                        let substr = &s[i..=end];
                        match substr {
                            "x" => tokens.push(Token::Variable),
                            "sin" => tokens.push(Token::Sin),
                            "cos" => tokens.push(Token::Cos),
                            "tan" => tokens.push(Token::Tan),
                            "tanh" => tokens.push(Token::Tanh),
                            "e" => tokens.push(Token::Constant(E)),
                            "pi" => tokens.push(Token::Constant(PI)),
                            "exp" => tokens.push(Token::Exp),
                            _ => return Err("Invalid string input".to_string()),
                        }
                        i = end + 1;
                    }
            _ => return Err("Invalid symbol".to_string())
                }
    }

    Ok(tokens)
}


fn parse(p: &mut Parser) -> Result<Expr, String> {
    if p.pow >= p.tokens.len() {
        Err()
    }
}

//addition
fn parse_expr(p: &mut Parser) -> Result<Expr, String> {
    let left = parse_term(p)?;

    let mut plus = false;
    while *p.peek()? == Token::Plus {
        plus = true;
        p.advance();
    }

    while *p.peek()? == Token::Minus {
        plus = false;
        p.advance();
    }
    let right = parse_term(p)?;
    
    if plus {
        Ok(Expr::Add(Box::new(left), Box::new(right)))
    } else {
        Ok(Expr::Sub(Box::new(left), Box::new(right)))
    }
}

//multi, div
fn parse_term(p: &mut Parser) -> Result<Expr, String> {
    let 
}

//exponent
fn parse_power(p: &mut Parser) -> Result<Expr, String> {

}

//negative, sin, cos, etc
fn parse_unary(p: &mut Parser) -> Result<Expr, String> {
    
}

//numbers, vars, parens
fn parse_primary(p: &mut Parser) -> Result<Expr, String> {
    match p.tokens[p.pos] {
        Token::Constant(f) => Ok(Expr::Constant(f)),
        Token::Variable => Ok(Expr::Variable),
        Token::OpenParen => { p.pos += 1; let r = parse_expr(p); p += 1; r}
    }
}