use crate::gvs::enums::{Control, Input, Queries};
use crate::gvs::errors::ParseError;
use core::panic;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn parse_path(input: &PathBuf) -> Vec<Vec<String>> {
    let file = fs::File::open(input).expect("no such file");
    let mut buf = BufReader::new(file);
    let mut string_buf = String::new();

    buf.read_to_string(&mut string_buf)
        .unwrap_or_else(|err| panic!("Error reading file: {err}"));

    string_buf
        .lines()
        .filter(|x| !x.starts_with("//") && !x.is_empty())
        .map(|i| {
            i.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

//merge bunch of results into one, doesn't return any particular error
fn result_merger<T, E>(vec: &Vec<Result<T, E>>) -> Result<(), ()> {
    for i in vec {
        if i.is_ok() {
            return Ok(());
        }
    }
    Err(())
}

//checks if thing starts with " and is either a number or string type
fn valid_raw_string(vec: &Vec<String>, i: usize) -> Result<Input, ParseError> {
    let str = &vec[i];

    if !str.starts_with('\"') {
        Err(ParseError {
            msg: format!(
                "parsing: {} This isn't a raw string \n Original line ~{}~",
                vec[i],
                vec.join(" ")
            ),
        })
    } else if str[1..].parse::<f32>().is_ok() {
        Ok(Input::StringFloat(str[1..].parse::<f32>().unwrap()))
    } else if valid_bool(vec, i).is_ok() {
        Err(ParseError {
            msg: format!(
                "parsing: {} This a bool \n Original line ~{}~",
                vec[i],
                vec.join(" ")
            ),
        })
    } else {
        Ok(Input::StringVariableName(str.to_string()))
    }
}

//checks if thing starts with " and is parsable to float
fn valid_raw_float(vec: &Vec<String>, i: usize) -> Result<Input, ParseError> {
    let str = &vec[i];

    if str[1..].parse::<f32>().is_ok() {
        Ok(Input::StringFloat(str[1..].parse::<f32>().unwrap()))
    } else {
        Err(ParseError {
            msg: format!(
                "parsing: {} This isn't a raw float \n Original line ~{}~",
                vec[i],
                vec.join(" ")
            ),
        })
    }
}

//checks if thing starts with " and is a bool
fn valid_bool(vec: &Vec<String>, i: usize) -> Result<Input, ParseError> {
    let str = &vec[i];

    match str.as_str() {
        "\"TRUE" => Ok(Input::StringBool(true)),
        "\"FALSE" => Ok(Input::StringBool(false)),
        _ => Err(ParseError {
            msg: format!(
                "parsing: {} This isn't a raw bool \n Original line ~{}~",
                vec[i],
                vec.join(" ")
            ),
        }),
    }
}

//adds a var, checks if arg1 is validrawstring and errorcores arg2 then inserts
fn make_var(vec: &Vec<String>, dict: &mut HashMap<String, String>) -> Result<(), ParseError> {
    match valid_raw_string(vec, 1) {
        Ok(Input::StringVariableName(_)) => {
            if result_merger(&vec![
                valid_query(vec, 2),
                valid_raw_float(vec, 2),
                valid_bool(vec, 2),
            ]).is_err() {
                return Err(ParseError {
                    msg: format!(
                        "parsing: {} Invalid second arg (MAKE) ~{}~",
                        vec[1],
                        vec.join(" ")
                    ),
                });
            }
            dict.insert(vec[1].to_string(), vec[2].to_string());
            Ok(())
        }
        _ => Err(ParseError {
            msg: format!(
                "parsing: {} (MAKE) invalid raw string \n Original line ~{}~",
                vec[1],
                vec.join(" ")
            ),
        }),
    }
}

//DOESNT ADD JUST CHECKS ERROR
//update dictionary val, checks if arg1 is validrawstring and errorcores arg2 then updates
fn add_assign(vec: &Vec<String>, dict: &HashMap<String, String>) -> Result<(), ParseError> {
    if let Ok(Input::StringVariableName(_)) = valid_raw_string(vec, 1) {
        error_core(vec, dict, 2)?;
        Ok(())
    } else {
        Err(ParseError {
            msg: format!(
                "parsing: {} Input type isn't StringVariableName (ADDASSIGN) \n Original line ~{}~",
                vec[1],
                vec.join(" ")
            ),
        })
    }
}

//DOESNT CHANGE VEC
//checks if arg1 is validrawstring and errorcores arg2 and arg3
fn while_if(vec: &Vec<String>, dict: &HashMap<String, String>) -> Result<(), ParseError> {
    if vec.len() != 5_usize {
        return Err(ParseError {
            msg: format!(
                "parsing: {} takes {} argument(s) \n Original line ~{}~",
                vec[0],
                4,
                vec.join(" ")
            ),
        });
    };

    let i = 2;
    let x = vec![
        valid_bool(vec, i),
        valid_raw_float(vec, i),
        valid_query(vec, i),
        valid_var(vec, i, dict),
    ];
    if result_merger(&x).is_err() {
        return Err(ParseError {
            msg: format!(
                "parsing: {} Invalid Params  for ARG2 \n Original line ~{}~",
                vec[1],
                vec.join(" ")
            ),
        });
    };

    let i = 3;
    let x = vec![
        valid_bool(vec, i),
        valid_raw_float(vec, i),
        valid_query(vec, i),
        valid_var(vec, i, dict),
    ];
    if result_merger(&x).is_err() {
        return Err(ParseError {
            msg: format!(
                "parsing: {} Invalid Params for ARG3 \n Original line ~{}~",
                vec[1],
                vec.join(" ")
            ),
        });
    };

    if (vec[0] != "WHILE" && vec[0] != "IF") || vec[1] != "EQ" || vec[4] != "[" {
        Err(ParseError {
            msg: format!(
                "parsing: {} Control isn't WHILE/IF/EQ  \n Original line ~{}~",
                vec[1],
                vec.join(" ")
            ),
        })
    } else if result_merger(&vec![better_core(vec, dict, 2), better_core(vec, dict, 3)]).is_err()
    {
        Err(ParseError {
            msg: format!(
                "parsing: {} Invalid Params (WHILE/IF) \n Original line ~{}~",
                vec[1],
                vec.join(" ")
            ),
        })
    } else {
        Ok(())
    }
}

fn valid_var(
    vec: &Vec<String>,
    i: usize,
    dict: &HashMap<String, String>,
) -> Result<Input, ParseError> {
    if !vec[i].starts_with(':') && !dict.contains_key(&vec[i]) {
        return Err(ParseError {
            msg: format!(
                "parsing: {} This isn't a variable \n Original line ~{}~",
                vec[i],
                vec.join(" ")
            ),
        });
    }
    Ok(Input::Variable(vec[i].to_string()))
}

fn valid_query(vec: &Vec<String>, i: usize) -> Result<Input, ParseError> {
    let x = vec[i].parse::<Queries>();
    if x.is_err() {
        Err(ParseError {
            msg: format!(
                "parsing: {} This isn't a query \n Original line ~{}~",
                vec[0],
                vec.join(" ")
            ),
        })
    } else {
        Ok(Input::Query(x.unwrap()))
    }
}

fn no_arg(line: &Vec<String>) -> Result<(), ParseError> {
    if let 1 = line.len() {
        Ok(())
    } else {
        Err(ParseError {
            msg: format!(
                "parsing: Line takes no arguments \n Original line ~{}~",
                line.join(" ")
            ),
        })
    }
}

// fn parse_operations(line: &mut Vec<String>, dict: &mut HashMap<String, String>) -> Result<(), ParseError> {
//     todo!()
// }

// not deprecated version of checking validity
fn better_core(
    line: &Vec<String>,
    dict: &HashMap<String, String>,
    index: usize,
) -> Result<(), ParseError> {
    if result_merger(&vec![
        valid_raw_string(line, index),
        valid_var(line, index, dict),
        valid_query(line, index),
    ]).is_err() {
        Err(ParseError{msg: format!("parsing: {} takes variable/raw string/query as the first argument \n Original line ~{}~", line[0], line.join(" "))})
    } else {
        Ok(())
    }
}

//i hate this function but i no time please no punish :()
fn error_core(
    line: &Vec<String>,
    dict: &HashMap<String, String>,
    args: usize,
) -> Result<(), ParseError> {
    if line.len() != args + 1_usize {
        Err(ParseError {
            msg: format!(
                "parsing: {} takes {} argument(s) \n Original line ~{}~",
                line[0],
                args,
                line.join(" ")
            ),
        })
    } else if result_merger(&vec![
        valid_raw_string(line, args),
        valid_var(line, args, dict),
        valid_query(line, args),
    ]).is_err() {
        Err(ParseError{msg: format!("parsing: {} takes variable/raw string/query as the first argument \n Original line ~{}~", line[0], line.join(" "))})
    } else {
        Ok(())
    }
}

pub fn parse_error_check(
    vec: &mut Vec<Vec<String>>,
    dict: &mut HashMap<String, String>,
) -> Result<(), ParseError> {
    for line in vec.iter_mut() {
        //check if need to parse line
        let flag = line.iter().any(|i| match i.chars().next() {
            Some('+') | Some('-') | Some('*') | Some('/') => true,
            None => panic!("shouldn't ever be a blank string"),
            _ => false,
        });
        if flag {
            //parse_operations(line, dict)?;
        } else {
            let x = line[0].parse::<Control>()?;

            match x {
                Control::PENUP | Control::PENDOWN | Control::CLOSEBRACE => no_arg(line)?,
                Control::FORWARD
                | Control::BACK
                | Control::LEFT
                | Control::RIGHT
                | Control::SETPENCOLOR
                | Control::TURN
                | Control::SETHEADING
                | Control::SETX
                | Control::SETY => error_core(line, dict, 1)?,
                Control::MAKE => make_var(line, dict)?,
                Control::ADDASSIGN => add_assign(line, dict)?,
                Control::WHILE | Control::IF => while_if(line, dict)?,
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn test_helper(testvec: &mut Vec<Vec<String>>, line: &str) {
    for i in line.split('\n') {
        if !i.is_empty() {
            testvec.push(
                i.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            );
        }
    }
}

//todo implement true false for make

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut testvec: Vec<Vec<String>> = Vec::new();
        let mut testdict: HashMap<String, String> = HashMap::new();

        let line = "MAKE \"thisfails \"thisfails";

        test_helper(&mut testvec, line);

        let output = parse_error_check(&mut testvec, &mut testdict);

        dbg!(&output);

        assert!(output.is_err());
    }

    #[test]
    fn test_2() {
        let mut testvec: Vec<Vec<String>> = Vec::new();
        let mut testdict: HashMap<String, String> = HashMap::new();

        let line = "MAKE \"TRUE \"2";

        test_helper(&mut testvec, line);

        let output = parse_error_check(&mut testvec, &mut testdict);

        dbg!(&output);

        assert!(output.is_err());
    }

    #[test]
    fn test_3() {
        let mut testvec: Vec<Vec<String>> = Vec::new();
        let mut testdict: HashMap<String, String> = HashMap::new();

        let line = "MAKE \"TRUE \"TRUE";

        test_helper(&mut testvec, line);

        let output = parse_error_check(&mut testvec, &mut testdict);

        dbg!(&output);

        assert!(output.is_err());
    }

    #[test]
    fn test_4() {
        let mut testvec: Vec<Vec<String>> = Vec::new();
        let mut testdict: HashMap<String, String> = HashMap::new();

        let line = "MAKE \"a \"TRUE";

        test_helper(&mut testvec, line);

        let output = parse_error_check(&mut testvec, &mut testdict);

        dbg!(&output);

        assert!(output.is_ok());
    }
}
