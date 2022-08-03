use std::{fs, path::Path};

use crate::model::{Force, Member, Pin, TrussProblem, Vec2};

pub fn load_input_file(path: impl AsRef<Path>) -> TrussProblem {
    let input = fs::read_to_string(path).unwrap();
    let mut problem = TrussProblem::default();

    for line in input.split('\n').filter(|l| l.len() > 0) {
        let mut parts = line.split_ascii_whitespace();

        match parts.next().unwrap() {
            "pin" => {
                problem.pins.insert(
                    parts.next().unwrap().chars().next().unwrap(),
                    Pin(Vec2([
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().parse().unwrap(),
                    ])),
                );
            }
            "member" => {
                let mut chars = parts.next().unwrap().chars();
                problem.members.push(Member {
                    pin_a: chars.next().unwrap(),
                    pin_b: chars.next().unwrap(),
                    tension: None,
                });
            }
            "force" => {
                problem.misc_forces.push(Force {
                    pin: parts.next().unwrap().chars().next().unwrap(),
                    magnitude: parts.next().unwrap().parse().ok(),
                    direction: parts.next().unwrap().parse().unwrap(),
                });
            }
            _ => unimplemented!(),
        }
    }

    problem
}

#[cfg(test)]
mod tests {
    use crate::model::{Force, Member, Pin, Vec2};

    use super::load_input_file;

    #[test]
    fn load_input() {
        let problem = load_input_file("examples/triangle.txt");

        assert_eq!(problem.pins.len(), 3);
        assert_eq!(problem.pins.get(&'A').unwrap(), &Pin(Vec2([0.0, 0.0])));
        assert_eq!(problem.pins.get(&'B').unwrap(), &Pin(Vec2([2.0, 0.0])));
        assert_eq!(problem.pins.get(&'C').unwrap(), &Pin(Vec2([1.0, 1.0])));

        assert_eq!(
            problem.members,
            vec![
                Member {
                    pin_a: 'A',
                    pin_b: 'B',
                    tension: None
                },
                Member {
                    pin_a: 'A',
                    pin_b: 'C',
                    tension: None
                },
                Member {
                    pin_a: 'B',
                    pin_b: 'C',
                    tension: None
                }
            ]
        );

        assert_eq!(
            problem.misc_forces,
            vec![
                Force {
                    pin: 'A',
                    magnitude: None,
                    direction: 0.0
                },
                Force {
                    pin: 'A',
                    magnitude: None,
                    direction: 90.0
                },
                Force {
                    pin: 'B',
                    magnitude: None,
                    direction: 90.0
                },
                Force {
                    pin: 'C',
                    magnitude: Some(1.0),
                    direction: 270.0
                }
            ]
        );
    }
}
