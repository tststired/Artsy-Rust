use crate::gvs::errors::ParseError;
use std::str::FromStr;
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub enum Bools {
    TRUE,
    FALSE,
}

#[derive(Debug)]
pub enum Queries {
    XCOR,
    YCOR,
    HEADING,
    COLOR,
}

#[derive(Debug)]
pub enum Input {
    StringFloat(f32),
    StringBool(bool),
    StringVariableName(String),
    Variable(String),
    Query(Queries),
}
#[derive(Clone, Copy, Debug)]
pub enum Control {
    PENUP,
    PENDOWN,
    FORWARD,
    BACK,
    LEFT,
    RIGHT,
    SETPENCOLOR,
    TURN,
    SETHEADING,
    SETX,
    SETY,
    MAKE,
    ADDASSIGN,
    WHILE,
    IF,
    CLOSEBRACE,
}

impl FromStr for Bools {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Bools, Self::Err> {
        match input {
            "\"TRUE" => Ok(Bools::TRUE),
            "\"FALSE" => Ok(Bools::FALSE),
            _ => Err(ParseError {
                msg: ("Error parsing file: Boolean not recognized".to_string()),
            }),
        }
    }
}

impl FromStr for Queries {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Queries, Self::Err> {
        match input {
            "XCOR" => Ok(Queries::XCOR),
            "YCOR" => Ok(Queries::YCOR),
            "HEADING" => Ok(Queries::HEADING),
            "COLOR" => Ok(Queries::COLOR),
            _ => Err(ParseError {
                msg: ("Error parsing file: Query not recognized".to_string()),
            }),
        }
    }
}

impl FromStr for Control {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Control, Self::Err> {
        match input {
            "PENUP" => Ok(Control::PENUP),
            "PENDOWN" => Ok(Control::PENDOWN),
            "FORWARD" => Ok(Control::FORWARD),
            "BACK" => Ok(Control::BACK),
            "LEFT" => Ok(Control::LEFT),
            "RIGHT" => Ok(Control::RIGHT),
            "SETPENCOLOR" => Ok(Control::SETPENCOLOR),
            "TURN" => Ok(Control::TURN),
            "SETHEADING" => Ok(Control::SETHEADING),
            "SETX" => Ok(Control::SETX),
            "SETY" => Ok(Control::SETY),
            "MAKE" => Ok(Control::MAKE),
            "ADDASSIGN" => Ok(Control::ADDASSIGN),
            "WHILE" => Ok(Control::WHILE),
            "IF" => Ok(Control::IF),
            "]" => Ok(Control::CLOSEBRACE),
            _ => Err(ParseError {
                msg: ("Error parsing file: Control command not recognized".to_string()),
            }),
        }
    }
}

impl ToString for Control {
    fn to_string(&self) -> String {
        match self {
            Control::PENUP => "PENUP".to_string(),
            Control::PENDOWN => "PENDOWN".to_string(),
            Control::FORWARD => "FORWARD".to_string(),
            Control::BACK => "BACK".to_string(),
            Control::LEFT => "LEFT".to_string(),
            Control::RIGHT => "RIGHT".to_string(),
            Control::SETPENCOLOR => "SETPENCOLOR".to_string(),
            Control::TURN => "TURN".to_string(),
            Control::SETHEADING => "SETHEADING".to_string(),
            Control::SETX => "SETX".to_string(),
            Control::SETY => "SETY".to_string(),
            Control::MAKE => "MAKE".to_string(),
            Control::ADDASSIGN => "ADDASSIGN".to_string(),
            Control::WHILE => "WHILE".to_string(),
            Control::IF => "IF".to_string(),
            Control::CLOSEBRACE => "]".to_string(),
        }
    }
}
