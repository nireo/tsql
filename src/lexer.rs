#[derive(Debug)]
pub struct Lexer<'a> {
    curr_pos: usize,
    input: &'a str,
}

#[derive(Debug)]
enum Token {
    CreateTable,
    Insert,
    Select,
    Values,
    Where,
    From,
    Concat,
    Equal,
    Plus,
    Lt,
    LeftParen,
    RightParen,
    Comma,
    Identifer(String),
    Integer(i64),
    String(String),
}

struct BuiltIn {
    name: &'static str,
    tok: Token,
}

static BUILTINS: [BuiltIn; _] = [
    BuiltIn {
        name: "create table",
        tok: Token::CreateTable,
    },
    BuiltIn {
        name: "insert into",
        tok: Token::Insert,
    },
    BuiltIn {
        name: "select",
        tok: Token::Select,
    },
    BuiltIn {
        name: "values",
        tok: Token::Values,
    },
    BuiltIn {
        name: "where",
        tok: Token::Where,
    },
    BuiltIn {
        name: "from",
        tok: Token::From,
    },
];

fn case_sensitive_eq<'a, 'b>(left: &'a str, right: &'b str) -> bool {}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer { curr_pos: 0, input }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        Vec::new()
    }

    pub fn reset(&mut self) {
        self.curr_pos = 0;
    }

    fn eat_whitespace(&mut self) {
        while self.input[self.curr_pos] == ' '
            || self.input[self.curr_pos] == '\t'
            || self.input[self.curr_pos] == '\n'
            || self.input[self.curr_pos] == '\r'
        {
            self.curr_pos += 1;
            if self.curr_pos == self.input.len() {
                break;
            }
        }
    }

    fn keyword(&mut self) -> Option<Token> {
        let mut longest = 0;
        let mut ty = Token::Select;

        for b in BUILTINS {
            if self.curr_pos + b.name.len() >= self.input.len() {
                continue;
            }
        }
    }
}
