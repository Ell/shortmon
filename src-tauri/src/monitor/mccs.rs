use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ParserError(&'static str);

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser Error: {}", self.0)
    }
}

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Atom(String),
    List(Vec<Expression>),
}

fn tokenize(input: &str) -> Vec<&str> {
    let split: Vec<&str> = input.split(" ").filter(|&s| s != "").collect();

    return split;
}

#[allow(dead_code, unused_variables)]
fn read_from_tokens(tokens: &[&str]) -> Result<Expression, ParserError> {
    let (_remaining, exp) = read_from_tokens_inner(&tokens)?;
    return Ok(exp);

    fn read_from_tokens_inner<'a>(
        tokens: &'a [&'a str],
    ) -> Result<(&'a [&'a str], Expression), ParserError> {
        let token = *tokens.first().ok_or(ParserError("unexpected eof"))?;
        let mut tokens = &tokens[1..];

        return if token == "(" {
            let mut exp_list: Vec<Expression> = vec![];

            while tokens.first() != Some(&")") {
                let (remaining, exp) = read_from_tokens_inner(tokens)?;
                tokens = remaining;

                exp_list.push(exp);
            }

            if tokens.first() != Some(&")") {
                return Err(ParserError("expected )"));
            }

            tokens = &tokens[1..];
            let exp_list = Expression::List(exp_list);

            Ok((tokens, exp_list))
        } else if token == ")" {
            Err(ParserError("invalid cap string"))
        } else {
            Ok((tokens, Expression::Atom(token.to_string())))
        };
    }
}

fn clean_input(input: String) -> String {
    return input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("  ", " ");
}

pub fn parse_cap_string(cap_string: String) -> Result<Vec<(String, Expression)>, ParserError> {
    let cleaned = clean_input(cap_string);
    let tokens = tokenize(&cleaned);

    let list = match read_from_tokens(&tokens)? {
        Expression::List(list) if list.len() % 2 == 0 => list,
        _ => return Err(ParserError("top level expr must be list")),
    };

    let mut chunks: Vec<(String, Expression)> = vec![];

    let mut iter = list.into_iter();
    while let Some(key) = iter.next() {
        let key = match key {
            Expression::Atom(k) => k,
            _ => return Err(ParserError("key isn't an atom")),
        };

        let value = iter.next().ok_or(ParserError("key without value"))?;

        let pair = (key, value);

        chunks.push(pair);
    }

    return Ok(chunks);
}

#[derive(Default, Debug, Clone)]
pub struct VCPCommand {
    pub command: String,
    pub values: Vec<VCPCommand>,
}

pub fn extract_atom(expression: Expression) -> String {
    match expression {
        Expression::List(list) => {
            let first = list.first().unwrap();

            return match first {
                Expression::Atom(value) => value.to_string(),
                _ => Default::default(),
            };
        }
        _ => Default::default(),
    }
}

pub fn extract_vcp_commands(expression: Expression) -> Vec<VCPCommand> {
    let mut cmds: Vec<VCPCommand> = vec![];

    match expression {
        Expression::List(list) => {
            let mut iter = list.into_iter().peekable();

            while let Some(e) = iter.next() {
                let mut new_cmd = VCPCommand {
                    command: Default::default(),
                    values: vec![],
                };

                match e {
                    Expression::Atom(inner_value) => {
                        if let Some(next) = iter.peek() {
                            match next {
                                Expression::List(list) => {
                                    for result in list {
                                        match result {
                                            Expression::Atom(value) => {
                                                let cmd = VCPCommand {
                                                    command: value.to_string(),
                                                    values: vec![],
                                                };

                                                new_cmd.values.push(cmd);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        new_cmd.command = inner_value;
                    }
                    _ => {}
                }

                cmds.push(new_cmd);
            }
        }
        _ => Default::default(),
    }

    return cmds;
}
