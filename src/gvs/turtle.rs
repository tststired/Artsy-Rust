use std::fmt::Display;
use std::fmt::{self, Debug};
use unsvg::Image;

pub struct Turtle {
    pub image: unsvg::Image,
    pub pen_up: bool,
    pub pen_color: f32,
    pub degrees: f32,
    pub curr_x: f32,
    pub curr_y: f32,
    pub mark_x: f32,
    pub mark_y: f32,
}

impl Turtle {
    pub fn new(image: Image) -> Turtle {
        let x = image.get_dimensions().0 as f32;
        let y = image.get_dimensions().1 as f32;

        Turtle {
            image,
            pen_up: true,
            pen_color: 7.0,
            degrees: 0.0,
            curr_x: (x / 2.0),
            curr_y: (y / 2.0),
            mark_x: 0.0,
            mark_y: 0.0,
        }
    }
}

struct ColorWrapper(unsvg::Color);
impl Display for ColorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Color: red: {}, green: {}, blue: {}",
            self.0.red, self.0.green, self.0.blue
        )
    }
}

impl Debug for Turtle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Turtle: pen_up: {}, pen_color: {}, degrees: {}, curr_x: {}, curr_y: {}, mark_x: {}, mark_y: {}"
        , self.pen_up, self.pen_color, self.degrees, self.curr_x, self.curr_y, self.mark_x, self.mark_y)
    }
}
