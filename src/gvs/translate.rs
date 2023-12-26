use crate::gvs::enums::{Control, Direction, Queries};
use crate::gvs::errors::ParseError;
use crate::gvs::turtle::Turtle;
use core::panic;
use std::collections::HashMap;

use super::enums::Bools;

fn end_calc(length: f32, turtle: &Turtle) -> (f32, f32) {
    unsvg::get_end_coordinates(
        turtle.curr_x,
        turtle.curr_y,
        turtle.degrees.round() as i32,
        length,
    )
}

fn set_color(color: f32, turtle: &mut Turtle) {
    turtle.pen_color = color;
}

fn draw_line(length: f32, turtle: &mut Turtle) {
    turtle
        .image
        .draw_simple_line(
            turtle.mark_x,
            turtle.mark_y,
            turtle.degrees.round() as i32,
            length,
            unsvg::COLORS[turtle.pen_color.round() as usize],
        )
        .unwrap_or_else(|err| panic!("Error drawing line: {err}"));

    turtle.mark_x = turtle.curr_x;
    turtle.mark_y = turtle.curr_y;
}

fn move_turtle(dir: Direction, length: f32, turtle: &mut Turtle) {
    turtle.degrees += match dir {
        Direction::North => 0.0,
        Direction::South => 180.0,
        Direction::East => 90.0,
        Direction::West => 270.0,
    };

    (turtle.curr_x, turtle.curr_y) = end_calc(length, turtle);

    if !turtle.pen_up {
        draw_line(length, turtle)
    };

    turtle.degrees -= match dir {
        Direction::North => 0.0,
        Direction::South => 180.0,
        Direction::East => 90.0,
        Direction::West => 270.0,
    };
}

fn set_turtle(turtle: &mut Turtle, x: f32, y: f32) {
    turtle.curr_x = x;
    turtle.curr_y = y;
    turtle.mark_x = x;
    turtle.mark_y = y;
}

//given a :variable, parse its f32 from a "raw_float / Query ONLY   (and can't be a variable name apparently)
fn lookup_var(
    turtle: &Turtle,
    dict: &HashMap<String, String>,
    str: &str,
    vec: &Vec<String>,
) -> Result<f32, ParseError> {
    if !str.starts_with(':') {
        return Err(ParseError {
            msg: format!(
                "parsing: {} not a variable \n Original line ~{}~",
                vec[1],
                vec.join(" ")
            ),
        });
    }

    match dict.get(&str.replace(':', "\"")) {
        Some(y) => {
            let yf = y[1..].parse::<f32>();
            let yq = y.parse::<Queries>();
            match (yf, yq) {
                (_, Ok(out)) => match out {
                    Queries::XCOR => Ok(turtle.curr_x),
                    Queries::YCOR => Ok(turtle.curr_y),
                    Queries::HEADING => Ok(turtle.degrees),
                    Queries::COLOR => Ok(turtle.pen_color),
                },
                (Ok(yf), _) => {
                    Ok(yf)
                }
                (Err(_), Err(_)) => {
                    Err(ParseError{msg: format!("parsing: {} Can't parse dict val (could be true/false/unknowns) \n Original line ~{}~", vec[1], vec.join(" "))})
                }
            }
        }
        None => Err(ParseError {
            msg: format!("parsing:  not in dict \n Original line ~{}~", vec.join(" ")),
        }),
    }
}

//given a :variable checks if is bool if is pass back Bools
fn lookup_var_bool(dict: &HashMap<String, String>, str: &str) -> Result<Bools, ParseError> {
    if !str.starts_with(':') {
        return Err(ParseError {
            msg: format!("parsing: {} not a variable", str),
        });
    }

    match dict.get(&str.replace(':', "\"")) {
        Some(y) => match y.parse::<Bools>() {
            Ok(x) => {
                Ok(x)
            }
            Err(_) => {
                Err(ParseError {
                    msg: format!("parsing: {} Can't parse dict val (could be float)", str),
                })
            }
        },
        None => Err(ParseError {
            msg: format!("parsing: {} not in dict", str),
        }),
    }
}

//given any "raw_string, parses its to f32, from a "raw_float / Query / or  :variable value
fn conv_str(
    turtle: &Turtle,
    dict: &HashMap<String, String>,
    str: &str,
    vec: &Vec<String>,
) -> Result<f32, ParseError> {
    let query = str.parse::<Queries>();
    let float = str[1..].parse::<f32>();
    let dict_val = lookup_var(turtle, dict, str, vec);

    match (query, float, dict_val) {
        (Ok(q), _, _) => match q {
            Queries::XCOR => Ok(turtle.curr_x),
            Queries::YCOR => Ok(turtle.curr_y),
            Queries::HEADING => Ok(turtle.degrees),
            Queries::COLOR => Ok(turtle.pen_color),
        },
        (_, Ok(x), _) => Ok(x),
        (_, _, Ok(x)) => Ok(x),
        (_, _, _) => Err(ParseError {
            msg: format!(
                "parsing: {} has no value (conv_str) \n Original line ~{}~",
                vec[0],
                vec.join(" ")
            ),
        }),
    }
}

//takes a "raw_string_variable and something that parses to a float
fn add_assign(
    turtle: &Turtle,
    dict: &mut HashMap<String, String>,
    vec: &Vec<String>,
) -> Result<(), ParseError> {
    if dict.get(&vec[1]).is_some() {
        let val1 = conv_str(turtle, dict, dict.get(&vec[1]).unwrap(), vec)?;
        let val2 = conv_str(turtle, dict, &vec[2], vec)?;
        dict.remove(&vec[1]);
        dict.insert(
            vec[1].clone(),
            "\"".to_string() + &(val1 + val2).to_string(),
        );
        Ok(())
    } else {
        Err(ParseError {
            msg: format!(
                "parsing: {} not in dict (ADDASSIGN) \n Original line ~{}~",
                vec[1],
                vec.join(" ")
            ),
        })
    }
}

pub fn execute(
    i: &Vec<String>,
    turtle: &mut Turtle,
    dict: &mut HashMap<String, String>,
) -> Result<(), ParseError> {
    let x = i[0].parse::<Control>()?;

    match x {
        Control::FORWARD => {
            move_turtle(Direction::North, conv_str(turtle, dict, &i[1], i)?, turtle)
        }
        Control::BACK => move_turtle(Direction::South, conv_str(turtle, dict, &i[1], i)?, turtle),
        Control::LEFT => move_turtle(Direction::West, conv_str(turtle, dict, &i[1], i)?, turtle),
        Control::RIGHT => move_turtle(Direction::East, conv_str(turtle, dict, &i[1], i)?, turtle),
        Control::SETPENCOLOR => set_color(conv_str(turtle, dict, &i[1], i)?, turtle),
        Control::TURN => turtle.degrees += conv_str(turtle, dict, &i[1], i)?,
        Control::SETHEADING => turtle.degrees = conv_str(turtle, dict, &i[1], i)?,
        Control::SETX => set_turtle(turtle, conv_str(turtle, dict, &i[1], i)?, turtle.curr_y),
        Control::SETY => set_turtle(turtle, turtle.curr_x, conv_str(turtle, dict, &i[1], i)?),
        Control::PENUP => {
            turtle.pen_up = true;
            turtle.mark_x = 0.0;
            turtle.mark_y = 0.0;
        }
        Control::PENDOWN => {
            turtle.pen_up = false;
            turtle.mark_x = turtle.curr_x;
            turtle.mark_y = turtle.curr_y;
        }
        Control::MAKE => {
            dict.insert(i[1].to_string(), i[2].to_string());
        }
        Control::ADDASSIGN => {
            add_assign(turtle, dict, i)?;
        }
        Control::CLOSEBRACE => (),
        _ => {
            panic!("shouldn't be here")
        }
    }
    Ok(())
}

//given an instruction set, find when it ends
pub fn find_sub_instru(instru: &Vec<Vec<String>>, curr_index: usize) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let mut bracket_count = 0;

    for i in curr_index..instru.len() {
        out.push(instru[i].clone());
        for j in instru[i].iter() {
            if j == "[" {
                bracket_count += 1;
            } else if j == "]" {
                bracket_count -= 1;
            }
        }
        if bracket_count == 0 {
            break;
        }
    }
    out
}

pub fn parse_bool(b1: Bools, b2: Bools) -> bool {
    match (b1, b2) {
        (Bools::TRUE, Bools::TRUE) => true,
        (Bools::FALSE, Bools::FALSE) => true,
        _ => false,
    }
}

pub fn check_equality(
    turtle: &mut Turtle,
    dict: &mut HashMap<String, String>,
    line: &Vec<String>,
) -> Result<bool, ParseError> {
    let first = &line[2];
    let second = &line[3];

    //parse as bool
    let fb = first.parse::<Bools>();
    let sb = second.parse::<Bools>();

    //var bool lookup
    let fvb = lookup_var_bool(dict, first);
    let svb = lookup_var_bool(dict, second);

    match (fb, sb, fvb, svb) {
        //1-2. 3-4. 1.4 2.3
        (Ok(f), Ok(s), _, _) => return Ok(parse_bool(f, s)),
        (Ok(f), _, _, Ok(s)) => return Ok(parse_bool(f, s)),
        (_, _, Ok(f), Ok(s)) => return Ok(parse_bool(f, s)),
        (_, Ok(f), Ok(s), _) => return Ok(parse_bool(f, s)),
        _ => (),
    }

    //convert both to floats
    let first_float = conv_str(turtle, dict, first, line);
    let second_float = conv_str(turtle, dict, second, line);

    match (first_float, second_float) {
        (Ok(f), Ok(s)) => {
            if f == s {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        (_, _) => {
            Err(ParseError {
                msg: format!(
                    "parsing: {} not a float n bool(check_equality) \n Original line ~{}~",
                    first,
                    line.join(" ")
                ),
            })
        }
    }
}

pub fn while_subprocess(
    instru: &Vec<Vec<String>>,
    turtle: &mut Turtle,
    dict: &mut HashMap<String, String>,
) -> Result<(), ParseError> {
    let mut cont_flag: usize = 0;

    let y = check_equality(turtle, dict, &instru[0])?;

    if y {
        for i in 1..instru.len() {
            if cont_flag > 0 {
                cont_flag -= 1;

                continue;
            }

            let x = instru[i][0].parse::<Control>()?;

            match x {
                Control::IF => {
                    let out = find_sub_instru(instru, i);
                    cont_flag = out.len() - 1;
                    if_subprocess(&out, turtle, dict)?;
                }
                Control::WHILE => {
                    let out = find_sub_instru(instru, i);
                    cont_flag = out.len() - 1;
                    while_subprocess(&out, turtle, dict)?;
                }
                _ => execute(&instru[i], turtle, dict)?,
            }
        }
    } else {
        return Ok(());
    }

    while_subprocess(instru, turtle, dict)?;

    Ok(())
}

pub fn if_subprocess(
    instru: &Vec<Vec<String>>,
    turtle: &mut Turtle,
    dict: &mut HashMap<String, String>,
) -> Result<(), ParseError> {
    let mut cont_flag: usize = 0;

    let y = check_equality(turtle, dict, &instru[0])?;

    if y {
        for i in 1..instru.len() {
            if cont_flag > 0 {
                cont_flag -= 1;

                continue;
            }

            let x = instru[i][0].parse::<Control>()?;

            match x {
                Control::IF => {
                    let out = find_sub_instru(instru, i);
                    cont_flag = out.len() - 1;
                    if_subprocess(&out, turtle, dict)?;
                }
                Control::WHILE => {
                    let out = find_sub_instru(instru, i);
                    cont_flag = out.len() - 1;
                    while_subprocess(&out, turtle, dict)?;
                }
                _ => execute(&instru[i], turtle, dict)?,
            }
        }
    } else {
        return Ok(());
    }
    Ok(())
}

pub fn translate(
    vec: &Vec<Vec<String>>,
    turtle: &mut Turtle,
    dict: &mut HashMap<String, String>,
) -> Result<(), ParseError> {
    let mut cont_flag: usize = 0;
    let mut line_count: i32 = -1;

    for i in vec.iter() {
        line_count += 1;

        if cont_flag > 0 {
            cont_flag -= 1;

            continue;
        }
        let x = i[0].parse::<Control>()?;

        match x {
            Control::IF => {
                let out = find_sub_instru(vec, line_count as usize);
                cont_flag = out.len() - 1;

                if_subprocess(&out, turtle, dict)?;
            }
            Control::WHILE => {
                let out = find_sub_instru(vec, line_count as usize);
                cont_flag = out.len() - 1;
                while_subprocess(&out, turtle, dict)?;
            }
            _ => execute(i, turtle, dict)?,
        }
    }
    Ok(())
}
