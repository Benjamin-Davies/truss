use std::{collections::BTreeMap, mem::swap};

use crate::vec2::Vec2;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrussProblem {
    pub pins: BTreeMap<char, Pin>,
    pub members: Vec<Member>,
    pub misc_forces: Vec<Force>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pin(pub Vec2);

#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    pub pin_a: char,
    pub pin_b: char,
    pub tension: Option<f64>,
}

impl Member {
    pub fn reversed(&self) -> Member {
        let mut new = self.clone();
        swap(&mut new.pin_a, &mut new.pin_b);
        new
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Force {
    pub pin: char,
    pub magnitude: Option<f64>,
    pub direction: f64,
}
