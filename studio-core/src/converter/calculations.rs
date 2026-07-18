// src/converter/calculations.rs

use serde_json::Value;

// ==========================================
// 1. AST (Abstract Syntax Tree) Definitions
// ==========================================

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    StringLit(String),
    Variable(String),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

// ==========================================
// 2. Lexer (Tokenization)
// ==========================================

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    StringLit(String),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    LParen,
    RParen,
    Comma,
    And,
    Or,
    Not,
    Eof,
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        match self.peek() {
            None => Ok(Token::Eof),
            Some(c) => match c {
                '+' => {
                    self.advance();
                    Ok(Token::Plus)
                }
                '*' => {
                    self.advance();
                    Ok(Token::Star)
                }
                '/' => {
                    self.advance();
                    Ok(Token::Slash)
                }
                '(' => {
                    self.advance();
                    Ok(Token::LParen)
                }
                ')' => {
                    self.advance();
                    Ok(Token::RParen)
                }
                ',' => {
                    self.advance();
                    Ok(Token::Comma)
                }
                '-' => {
                    self.advance();
                    Ok(Token::Minus)
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Ok(Token::Eq)
                    } else {
                        Err("Expected '=' after '='".to_string())
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Ok(Token::Le)
                    } else if self.peek() == Some('>') {
                        self.advance();
                        Ok(Token::Ne)
                    } else {
                        Ok(Token::Lt)
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Ok(Token::Ge)
                    } else {
                        Ok(Token::Gt)
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Ok(Token::Ne)
                    } else {
                        Err("Expected '=' after '!'".to_string())
                    }
                }
                '"' | '\'' => {
                    let quote = self.advance().unwrap();
                    let mut s = String::new();
                    while let Some(nc) = self.peek() {
                        if nc == quote {
                            self.advance();
                            break;
                        }
                        s.push(nc);
                        self.advance();
                    }
                    Ok(Token::StringLit(s))
                }

                //  Allow commas in numbers, but strip them before parsing
                _ if c.is_ascii_digit() || c == '.' || c == ',' => {
                    let mut num_str = String::new();
                    while let Some(nc) = self.peek() {
                        if nc.is_ascii_digit() || nc == '.' || nc == ',' {
                            if nc != ',' {
                                num_str.push(nc);
                            } // Ignore commas
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    num_str
                        .parse::<f64>()
                        .map(Token::Number)
                        .map_err(|_| "Invalid number".to_string())
                }

                _ if c.is_ascii_digit() || c == '.' => {
                    let mut num_str = String::new();
                    while let Some(nc) = self.peek() {
                        if nc.is_ascii_digit() || nc == '.' {
                            num_str.push(nc);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    num_str
                        .parse::<f64>()
                        .map(Token::Number)
                        .map_err(|_| "Invalid number".to_string())
                }
                _ if c.is_ascii_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(nc) = self.peek() {
                        if nc.is_ascii_alphanumeric() || nc == '_' {
                            ident.push(nc);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    match ident.to_uppercase().as_str() {
                        "AND" => Ok(Token::And),
                        "OR" => Ok(Token::Or),
                        "NOT" => Ok(Token::Not),
                        "TRUE" => Ok(Token::Number(1.0)),
                        "FALSE" => Ok(Token::Number(0.0)),
                        _ => Ok(Token::Ident(ident)),
                    }
                }
                _ => Err(format!("Unexpected character: {}", c)),
            },
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let t = self.next_token()?;
            if t == Token::Eof {
                tokens.push(t);
                break;
            }
            tokens.push(t);
        }
        Ok(tokens)
    }
}

// ==========================================
// 3. Parser (Recursive Descent)
// ==========================================

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let t = self.peek().clone();
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.peek() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.peek()))
        }
    }

    // Entry point: handles OR (lowest precedence)
    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while *self.peek() == Token::Or {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinaryOp(Box::new(left), BinOp::Or, Box::new(right));
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_not()?;
        while *self.peek() == Token::And {
            self.advance();
            let right = self.parse_not()?;
            left = Expr::BinaryOp(Box::new(left), BinOp::And, Box::new(right));
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expr, String> {
        if *self.peek() == Token::Not {
            self.advance();
            let expr = self.parse_not()?;
            return Ok(Expr::UnaryOp(UnaryOp::Not, Box::new(expr)));
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_add_sub()?;
        loop {
            let op = match self.peek() {
                Token::Eq => BinOp::Eq,
                Token::Ne => BinOp::Ne,
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                Token::Le => BinOp::Le,
                Token::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_add_sub()?;
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul_div()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_mul_div()?;
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if *self.peek() == Token::Minus {
            self.advance();
            let expr = self.parse_primary()?;
            return Ok(Expr::UnaryOp(UnaryOp::Neg, Box::new(expr)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s))
            }
            Token::Ident(name) => {
                self.advance();
                if *self.peek() == Token::LParen {
                    // Function call
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    if *self.peek() != Token::RParen {
                        args.push(self.parse_expr()?);
                        while *self.peek() == Token::Comma {
                            self.advance();
                            args.push(self.parse_expr()?);
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::FunctionCall(name, args))
                } else {
                    // Variable
                    Ok(Expr::Variable(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }
}

// ==========================================
// 4. Typst Transpiler (AST -> Typst Code)
// ==========================================

pub struct TranspileContext {
    pub is_loop: bool,
    pub loop_path: String, // e.g., "items" or "details.collaterals"
}

pub fn transpile_to_typst(formula: &str, ctx: &TranspileContext) -> Result<String, String> {
    if !formula.starts_with('=') {
        return Ok(formula.to_string());
    }
    let expr_str = &formula[1..];
    let mut lexer = Lexer::new(expr_str);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr()?;

    let typst_code = emit_typst(&ast, ctx);
    Ok(format!("#{{ {} }}", typst_code))
}

fn emit_typst(expr: &Expr, ctx: &TranspileContext) -> String {
    match expr {
        Expr::Number(n) => format!("{}", n),
        Expr::StringLit(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Expr::Variable(name) => {
            if ctx.is_loop {
                format!("num(safe-get(item, \"{}\"))", name)
            } else {
                format!("num(safe-get(data, \"{}\"))", name)
            }
        }
        Expr::BinaryOp(left, op, right) => {
            let l = emit_typst(left, ctx);
            let r = emit_typst(right, ctx);
            let op_str = match op {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/",
                BinOp::Eq => "==",
                BinOp::Ne => "!=",
                BinOp::Lt => "<",
                BinOp::Gt => ">",
                BinOp::Le => "<=",
                BinOp::Ge => ">=",
                BinOp::And => "and",
                BinOp::Or => "or",
            };
            format!("({} {} {})", l, op_str, r)
        }
        Expr::UnaryOp(op, expr) => {
            let e = emit_typst(expr, ctx);
            match op {
                UnaryOp::Neg => format!("(-{})", e),
                UnaryOp::Not => format!("(not {})", e),
            }
        }
        Expr::FunctionCall(name, args) => {
            let name_upper = name.to_uppercase();
            match name_upper.as_str() {
                "SUM" | "AVG" | "MIN" | "MAX" | "COUNT" => {
                    if let Some(Expr::Variable(field)) = args.get(0) {
                        let map_expr = format!(
                            "data.{}.map(i => num(safe-get(i, \"{}\")))",
                            ctx.loop_path.replace('.', "."),
                            field
                        );
                        match name_upper.as_str() {
                            "SUM" => format!("{}.sum()", map_expr),
                            "AVG" => format!("{{ let arr = {}; arr.sum() / arr.len() }}", map_expr),
                            "MIN" => format!("{}.min()", map_expr),
                            "MAX" => format!("{}.max()", map_expr),
                            "COUNT" => format!("{}.len()", map_expr),
                            _ => unreachable!(),
                        }
                    } else {
                        "0".to_string() // Fallback
                    }
                }
                "IF" => {
                    if args.len() == 3 {
                        let cond = emit_typst(&args[0], ctx);
                        let t = emit_typst(&args[1], ctx);
                        let f = emit_typst(&args[2], ctx);
                        format!("if {} {{ {} }} else {{ {} }}", cond, t, f)
                    } else {
                        "0".to_string()
                    }
                }
                "CONCAT" => {
                    let parts: Vec<String> = args
                        .iter()
                        .map(|a| format!("str({})", emit_typst(a, ctx)))
                        .collect();
                    parts.join(" + ")
                }
                _ => "0".to_string(), // Unknown function
            }
        }
    }
}

// ==========================================
// 5. Rust Evaluator (For HTML Engine)
// ==========================================

pub fn evaluate_formula(
    formula: &str,
    current_row: Option<&Value>,
    all_rows: &[Value],
    loop_path: &str,
) -> String {
    if !formula.starts_with('=') {
        return formula.to_string();
    }
    let expr_str = &formula[1..];
    let mut lexer = Lexer::new(expr_str);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => return formula.to_string(),
    };
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse_expr() {
        Ok(a) => a,
        Err(_) => return formula.to_string(),
    };

    let val = eval_ast(&ast, current_row, all_rows, loop_path);
    match val {
        EvalResult::Number(n) => format!("{:.2}", n),
        EvalResult::String(s) => s,
        EvalResult::Bool(b) => if b { "TRUE" } else { "FALSE" }.to_string(),
        EvalResult::Error(e) => e,
    }
}

#[derive(Debug)]
enum EvalResult {
    Number(f64),
    String(String),
    Bool(bool),
    Error(String),
}

fn eval_ast(expr: &Expr, row: Option<&Value>, rows: &[Value], path: &str) -> EvalResult {
    match expr {
        Expr::Number(n) => EvalResult::Number(*n),
        Expr::StringLit(s) => EvalResult::String(s.clone()),
        Expr::Variable(name) => {
            let val = if let Some(r) = row {
                get_num(r, name)
            } else {
                None
            };
            val.map(EvalResult::Number)
                .unwrap_or(EvalResult::Number(0.0))
        }
        Expr::BinaryOp(left, op, right) => {
            let l = eval_ast(left, row, rows, path);
            let r = eval_ast(right, row, rows, path);

            // Handle string concatenation or comparison
            if let (EvalResult::String(ls), EvalResult::String(rs)) = (&l, &r) {
                if let BinOp::Add = op {
                    return EvalResult::String(format!("{}{}", ls, rs));
                }
            }

            let ln = to_num(&l);
            let rn = to_num(&r);

            match op {
                BinOp::Add => EvalResult::Number(ln + rn),
                BinOp::Sub => EvalResult::Number(ln - rn),
                BinOp::Mul => EvalResult::Number(ln * rn),
                BinOp::Div => EvalResult::Number(if rn != 0.0 { ln / rn } else { 0.0 }),
                BinOp::Eq => EvalResult::Bool(ln == rn),
                BinOp::Ne => EvalResult::Bool(ln != rn),
                BinOp::Lt => EvalResult::Bool(ln < rn),
                BinOp::Gt => EvalResult::Bool(ln > rn),
                BinOp::Le => EvalResult::Bool(ln <= rn),
                BinOp::Ge => EvalResult::Bool(ln >= rn),
                BinOp::And => EvalResult::Bool(ln != 0.0 && rn != 0.0),
                BinOp::Or => EvalResult::Bool(ln != 0.0 || rn != 0.0),
            }
        }
        Expr::UnaryOp(op, e) => {
            let v = eval_ast(e, row, rows, path);
            match op {
                UnaryOp::Neg => EvalResult::Number(-to_num(&v)),
                UnaryOp::Not => EvalResult::Bool(to_num(&v) == 0.0),
            }
        }
        Expr::FunctionCall(name, args) => {
            let name_upper = name.to_uppercase();
            match name_upper.as_str() {
                "SUM" | "AVG" | "MIN" | "MAX" | "COUNT" => {
                    if let Some(Expr::Variable(field)) = args.get(0) {
                        let vals: Vec<f64> =
                            rows.iter().filter_map(|r| get_num(r, field)).collect();
                        let res = match name_upper.as_str() {
                            "SUM" => vals.iter().sum(),
                            "AVG" => {
                                if vals.is_empty() {
                                    0.0
                                } else {
                                    vals.iter().sum::<f64>() / vals.len() as f64
                                }
                            }
                            "MIN" => vals.iter().cloned().fold(f64::INFINITY, f64::min),
                            "MAX" => vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                            "COUNT" => vals.len() as f64,
                            _ => 0.0,
                        };
                        EvalResult::Number(res)
                    } else {
                        EvalResult::Number(0.0)
                    }
                }
                "IF" => {
                    if args.len() == 3 {
                        let cond = eval_ast(&args[0], row, rows, path);
                        if to_num(&cond) != 0.0 {
                            eval_ast(&args[1], row, rows, path)
                        } else {
                            eval_ast(&args[2], row, rows, path)
                        }
                    } else {
                        EvalResult::Error("IF requires 3 args".to_string())
                    }
                }
                "CONCAT" => {
                    let parts: Vec<String> = args
                        .iter()
                        .map(|a| match eval_ast(a, row, rows, path) {
                            EvalResult::String(s) => s,
                            EvalResult::Number(n) => format!("{:.2}", n),
                            EvalResult::Bool(b) => if b { "TRUE" } else { "FALSE" }.to_string(),
                            EvalResult::Error(e) => e,
                        })
                        .collect();
                    EvalResult::String(parts.join(""))
                }
                _ => EvalResult::Error(format!("Unknown function: {}", name)),
            }
        }
    }
}

fn to_num(res: &EvalResult) -> f64 {
    match res {
        EvalResult::Number(n) => *n,
        EvalResult::Bool(b) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

// ==========================================
// 6. Helpers
// ==========================================

pub fn get_all_rows(data: &Value, path: &str) -> Option<Vec<Value>> {
    let mut current = data;
    for part in path.split('.') {
        match current {
            Value::Object(map) => current = map.get(part)?,
            Value::Array(arr) => current = arr.get(part.parse::<usize>().ok()?)?,
            _ => return None,
        }
    }
    if let Value::Array(arr) = current {
        Some(arr.clone())
    } else {
        None
    }
}

fn get_num(row: &Value, key: &str) -> Option<f64> {
    row.get(key).and_then(|v| {
        if let Some(n) = v.as_f64() {
            return Some(n);
        }
        if let Some(s) = v.as_str() {
            return s.replace(',', "").parse::<f64>().ok();
        }
        None
    })
}

// ==========================================
// LOCALE-AWARE NUMBER FORMATTER
// ==========================================

pub struct LocaleConfig {
    pub thousands_sep: char,
    pub decimal_sep: char,
    /// True for Devanagari-script locales (Nepali, Hindi, etc.)
    /// which use asymmetric grouping (e.g., 10,00,000 for Lakhs/Crores).
    pub devanagari_grouping: bool,
}

pub fn get_locale_config(locale: &str) -> LocaleConfig {
    match locale.to_lowercase().as_str() {
        // Devanagari / South Asian (Lakhs & Crores: 10,00,000.00)
        // Covers Nepal (ne-NP, en-NP), India (hi-IN, en-IN), etc.
        "ne-np" | "en-np" | "hi-in" | "en-in" | "mr-in" => LocaleConfig {
            thousands_sep: ',',
            decimal_sep: '.',
            devanagari_grouping: true,
        },
        // European (1.000.000,00)
        "de-de" | "fr-fr" | "es-es" | "it-it" | "pt-br" => LocaleConfig {
            thousands_sep: '.',
            decimal_sep: ',',
            devanagari_grouping: false,
        },
        // Swiss (1'000'000.00)
        "fr-ch" | "de-ch" => LocaleConfig {
            thousands_sep: '\'',
            decimal_sep: '.',
            devanagari_grouping: false,
        },
        // Default to International/US (1,000,000.00)
        _ => LocaleConfig {
            thousands_sep: ',',
            decimal_sep: '.',
            devanagari_grouping: false,
        },
    }
}

/// Formats a number dynamically based on locale and decimal precision.
pub fn format_number(num: f64, locale: &str, decimal_places: Option<usize>) -> String {
    let config = get_locale_config(locale);
    let is_negative = num < 0.0;
    let abs_num = num.abs();

    // Round to specified decimal places (default 2)
    let decimals = decimal_places.unwrap_or(2);
    let multiplier = 10f64.powi(decimals as i32);
    let rounded = (abs_num * multiplier).round() / multiplier;

    // Split into integer and decimal parts
    let formatted_str = format!("{:.prec$}", rounded, prec = decimals);
    let parts: Vec<&str> = formatted_str.split('.').collect();
    let int_part = parts[0].to_string();
    let dec_part = if parts.len() > 1 && decimals > 0 {
        parts[1]
    } else {
        ""
    };

    let len = int_part.len();
    let mut result = String::new();

    if config.devanagari_grouping {
        // Devanagari system: First 3 digits, then groups of 2 (e.g., 10,00,000)
        if len <= 3 {
            result = int_part;
        } else {
            result.push_str(&int_part[len - 3..]);
            let mut remaining = &int_part[..len - 3];
            while remaining.len() > 2 {
                result = format!(
                    "{}{}{}",
                    &remaining[remaining.len() - 2..],
                    config.thousands_sep,
                    result
                );
                remaining = &remaining[..remaining.len() - 2];
            }
            if !remaining.is_empty() {
                result = format!("{}{}{}", remaining, config.thousands_sep, result);
            }
        }
    } else {
        // Standard 3-digit international grouping (e.g., 1,000,000)
        let mut chars = int_part.chars().rev().peekable();
        let mut count = 0;
        while let Some(c) = chars.next() {
            if count == 3 {
                result.push(config.thousands_sep);
                count = 0;
            }
            result.push(c);
            count += 1;
        }
        result = result.chars().rev().collect();
    }

    let final_str = if dec_part.is_empty() || decimals == 0 {
        result
    } else {
        format!("{}{}{}", result, config.decimal_sep, dec_part)
    };

    if is_negative {
        format!("-{}", final_str)
    } else {
        final_str
    }
}
