use std::collections::BTreeMap;

pub struct TrussProblem {
    pub pins: BTreeMap<char, Pin>,
    pub members: Vec<Member>,
    pub misc_forces: Vec<Force>,
}

pub struct Pin(Vec2);

pub struct Member {
    pub pin_a: char,
    pub pin_b: char,
    pub tension: Option<f64>,
}

pub struct Force {
    pub pin: char,
    pub magnitude: Option<f64>,
    pub direction: f64,
}

pub struct Vec2([f64; 2]);
