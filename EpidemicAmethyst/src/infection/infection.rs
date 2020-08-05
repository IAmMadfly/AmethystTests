use std::rc::{Rc, Weak};

#[derive(Debug)]
enum DiseaseType {
    Virus
}

#[derive(Debug)]
pub struct Disease {
    disease_type:       DiseaseType,
    parent:             Option<Weak<Disease>>,
    children:           Vec<Rc<Disease>>,
    attributes:         Vec<Attribute>
}

#[derive(Debug)]
struct Attribute {
    //disease_types:      Vec<DiseaseType>,
    effect_interval:    f32,    // Amount of days before attribute effects
}
