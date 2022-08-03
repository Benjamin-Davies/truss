use std::collections::BTreeMap;

#[derive(Debug, Default, PartialEq)]
pub struct TrussProblem {
    pub pins: BTreeMap<char, Pin>,
    pub members: Vec<Member>,
    pub misc_forces: Vec<Force>,
}

#[derive(Debug, PartialEq)]
pub struct Pin(pub Vec2);

#[derive(Debug, PartialEq)]
pub struct Member {
    pub pin_a: char,
    pub pin_b: char,
    pub tension: Option<f64>,
}

#[derive(Debug, PartialEq)]
pub struct Force {
    pub pin: char,
    pub magnitude: Option<f64>,
    pub direction: f64,
}

#[derive(Debug, PartialEq)]
pub struct Vec2(pub [f64; 2]);
