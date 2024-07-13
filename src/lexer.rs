use std::fmt::Debug;



#[derive(Debug, Clone)]
pub enum NumType {
    FLOAT,
    INT
}




#[derive(Clone)]
pub enum Token {
    NUM(f32, NumType),
    STRING(String),
    NAME(String),
    PRINT,
    INPUT,

    LPAR,
    RPAR,
    LSQB,
    RSQB,
    COLON,
    COMMA,
    SEMI,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    VBAR,
    AMPER,
    LESS,
    GREATER,
    EQUAL,
    DOT,
    PERCENT,
    LBRACE,
    RBRACE,
    EQEQUAL,
    NOTEQUAL,
    LESSEQUAL,
    GREATEREQUAL,
    TILDE,
    CIRCUMFLEX,
    LEFTSHIFT,
    RIGHTSHIFT,
    DOUBLESTAR,
    PLUSEQUAL,
    MINEQUAL,
    STAREQUAL,
    SLASHEQUAL,
    PERCENTEQUAL,
    AMPEREQUAL,
    VBAREQUAL,
    CIRCUMFLEXEQUAL,
    LEFTSHIFTEQUAL,
    RIGHTSHIFTEQUAL,
    DOUBLESTAREQUAL,
    DOUBLESLASH,
    DOUBLESLASHEQUAL,
    AT,
    ATEQUAL,
    RARROW,
    ELLIPSIS,
    COLONEQUAL,
    OP,
    AWAIT,
    ASYNC,
    TYPE_IGNORE,
    TYPE_COMMENT,
    SOFT_KEYWORD,

}

pub static mut AST_PRINT: bool = false;


impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            if AST_PRINT {
                return match self {
                    Self::NUM(inner, _) => write!(f, "{}", inner),
                    Self::STRING(inner) => write!(f, "{}", inner),
                    Self::NAME(inner) => write!(f, "{}", inner),
                    other => write!(f, ""),
                };
            }
            else {
                return match self {
                    Self::NUM(inner, inner2) => write!(f, "NUM({}, {:?})", inner, inner2),
                    Self::STRING(inner) => write!(f, "STRING({})", inner),
                    Self::NAME(inner) => write!(f, "NAME({})", inner),
                    other => write!(f, "{}", other.name()),
                };
            }
        }
    }
}

macro_rules! impl_variant_name {
    ($enum:ident, $($variant:ident$(($($field:ident),+))?),*) => {
        impl $enum {
            fn variant_name(&self) -> &str {
                match self {
                    $(
                        $enum::$variant$(($($field),+))? => stringify!($variant),
                    )*
                    _ => panic!("Variant not included in macro call"),
                }
            }
        }
    };
}


impl Token {
    pub fn get_num(&self) -> Option<f32> {
        match self {
            Token::NUM(number, type_of_number) => Some(*number),
            other => None
        }
    }
    pub fn get_num_type(&self) -> Option<NumType> {
        match self {
            Token::NUM(number, type_of_number) => Some(type_of_number.clone()),
            other => None
        }
    }
    pub fn get_string(&self) -> Option<String> {
        match self {
            Token::STRING(string) => Some(string.to_string()),
            other => None
        }
    }
    pub fn get_name(&self) -> Option<String> {
        match self {
            Token::NAME(name) => Some(name.to_string()),
            other => None
        }
    }
    pub fn value(&self) -> String {
        match self {
            Token::NAME(_) => self.get_name().unwrap(),
            Token::STRING(_) => self.get_string().unwrap(),
            Token::NUM(_, _) => self.get_num().unwrap().to_string(),
            Token::SLASH => "/".to_string(),
            Token::STAR => "*".to_string(),
            Token::MINUS => "-".to_string(),
            Token::PLUS => "+".to_string(),
            Token::EQUAL => "=".to_string(),
            Token::PRINT => "print".to_string(),
            Token::INPUT => "input".to_string(),
            other => todo!()
        }
    }
    pub fn name(&self) -> String {
        match self {
            Token::NAME(_) => "NAME".to_string(),
            Token::STRING(_) => "STRING".to_string(),
            Token::NUM(_, _) => "NUM".to_string(),
            other => other.variant_name().to_string()
        }
    }
}

impl_variant_name!(Token, 
    PRINT,
    INPUT,
    LPAR,
    RPAR,
    LSQB,
    RSQB,
    COLON,
    COMMA,
    SEMI,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    VBAR,
    AMPER,
    LESS,
    GREATER,
    EQUAL,
    DOT,
    PERCENT,
    LBRACE,
    RBRACE,
    EQEQUAL,
    NOTEQUAL,
    LESSEQUAL,
    GREATEREQUAL,
    TILDE,
    CIRCUMFLEX,
    LEFTSHIFT,
    RIGHTSHIFT,
    DOUBLESTAR,
    PLUSEQUAL,
    MINEQUAL,
    STAREQUAL,
    SLASHEQUAL,
    PERCENTEQUAL,
    AMPEREQUAL,
    VBAREQUAL,
    CIRCUMFLEXEQUAL,
    LEFTSHIFTEQUAL,
    RIGHTSHIFTEQUAL,
    DOUBLESTAREQUAL,
    DOUBLESLASH,
    DOUBLESLASHEQUAL,
    AT,
    ATEQUAL,
    RARROW,
    ELLIPSIS,
    COLONEQUAL,
    OP,
    AWAIT,
    ASYNC,
    TYPE_IGNORE,
    TYPE_COMMENT,
    SOFT_KEYWORD,
    NUM(i32, NumType),
    STRING(String),
    NAME(String));

macro_rules! add_num_to_tokens {
    ($num: expr, $tokens: expr, $index: expr) => {
        if $num != "" {
                    $tokens.push(Token::NUM(
            $num.parse().expect("couldnt parse num"), 
            
            if $num.contains(".") 
            {
                NumType::FLOAT
            } 
            else {
                NumType::INT
            }
        ));

        $num = "".to_string();
        }
    };
}

pub fn lex(line: &str) -> Vec<Token> {
    let mut index = 0;

    let mut tokens = vec![];
    
    let mut num = "".to_string();

    while index < line.len() {
        let char = line.chars().collect::<Vec<char>>()[index];

        //println!("char: {} num: {}, line: {}", char, num, line);

        if char == ' ' {
            add_num_to_tokens!(num, tokens, index);
        }
        else if char == '=' {
            add_num_to_tokens!(num, tokens, index);
            tokens.push(Token::EQUAL);
        }
        else if char == '+' {
            add_num_to_tokens!(num, tokens, index);
            tokens.push(Token::PLUS);
        }
        else if char == '-' {
            add_num_to_tokens!(num, tokens, index);
            tokens.push(Token::MINUS);
        }
        else if char == '*' {
            add_num_to_tokens!(num, tokens, index);
            tokens.push(Token::STAR);
        }
        else if char == '/' {
            add_num_to_tokens!(num, tokens, index);
            tokens.push(Token::SLASH);
        }
        else if (num.len() == 0 && "-0123456789".contains(char)) || (num.len() > 0 && ".0123456789".contains(char)) {
            num += &char.to_string();
        }
        else if char == '"' {
            add_num_to_tokens!(num, tokens, index);
            let mut string = "".to_string();
            index += 1;
            loop {
                let char = line.chars().collect::<Vec<char>>()[index];

                if r"^1234567890ß´qwertzuiopü+asdfghjklöä#asdfghjklöä#<yxcvbnm,.- °!§$%&/()=?`*_:;>@€{}[\~'|".contains(char) {
                    string += &char.to_string();
                }
                else {
                    tokens.push(Token::STRING(string));
                    break;
                }

                index += 1;
            }
        }
        else if "qwertzuiopüasdfghjklöäyxcvbnmQWERTZUIOPÜASDFGHJKLÖÄYXCVBNMß".contains(char) {
            add_num_to_tokens!(num, tokens, index);
            let mut name = char.to_string();
            index += 1;
            loop {
                let char = line.chars().collect::<Vec<char>>()[index];

                if r"^1234567890qwertzuiopüasdfghjklöäyxcvbnmQWERTZUIOPÜASDFGHJKLÖÄYXCVBNMß".contains(char) {
                    name += &char.to_string();
                }
                else {
                    match name.as_str() {
                        "print" => tokens.push(Token::PRINT),
                        "input" => tokens.push(Token::INPUT),
                        name => tokens.push(Token::NAME(name.to_string()))
                    }
                    
                    index -= 1;
                    break;
                }

                index += 1;
            }
        }

        

        index += 1;
    }

    add_num_to_tokens!(num, tokens, index);

    tokens
} 