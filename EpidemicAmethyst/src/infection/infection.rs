use std::rc::{Rc, Weak};

pub trait Disease {
    fn name(&self) -> &str;

    fn surface_chance(&self) -> f32;
}

pub struct Virus {
    parent:             Option<Weak<Virus>>,
    children:           Vec<Rc<Virus>>,

}

impl Disease for Virus {
    fn name(&self) -> &str {
        "Virus"
    }

    fn surface_chance(&self) -> f32 {
        0.24
    }
}