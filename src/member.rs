use crate::{
    model::{Force, Member, TrussProblem},
    vec2::Vec2,
};

pub fn solve_member_forces(problem: &mut TrussProblem) {
    while let Some(pin) = find_solvable_pin(&*problem) {
        solve_pin(problem, pin);
    }
}

fn solve_pin(problem: &mut TrussProblem, pin: char) {
    println!("Solving pin {}", pin);
    let members = problem
        .members
        .iter()
        .enumerate()
        .filter(|(_, member)| member.pin_a == pin)
        .map(|(i, member)| (i, member.clone()))
        .chain(
            problem
                .members
                .iter()
                .enumerate()
                .filter(|(_, member)| member.pin_b == pin)
                .map(|(i, pin)| (i, pin.reversed())),
        )
        .collect::<Vec<(usize, Member)>>();
    let member_directions = members
        .iter()
        .map(|(_, member)| {
            let pin_a = problem.pins.get(&member.pin_a).unwrap();
            let pin_b = problem.pins.get(&member.pin_b).unwrap();
            (pin_b.0 - pin_a.0).normalized()
        })
        .collect::<Vec<Vec2>>();
    let forces = problem
        .misc_forces
        .iter()
        .filter(|force| force.pin == pin)
        .map(|force| force.clone())
        .collect::<Vec<Force>>();

    // A * Tensions = 0
    // Row 1 = F_x coefficient
    // Row 2 = F_y coefficient
    let matrix_a = member_directions
        .into_iter()
        .chain(forces.iter().map(|force| Vec2::from_angle(force.direction)))
        .collect::<Vec<Vec2>>();
    println!("{:?}", matrix_a);

    // B * Unknown Tensions + C * Known Tensions = 0
    let unknown_tensions = members
        .iter()
        .filter_map(|(i, m)| if m.tension.is_none() { Some(*i) } else { None })
        .collect::<Vec<usize>>();
    let known_tensions = members
        .iter()
        .filter_map(|(i, m)| m.tension.map(|t| (*i, t)))
        .collect::<Vec<(usize, f64)>>();

    todo!();
}

fn find_solvable_pin(problem: &TrussProblem) -> Option<char> {
    problem
        .pins
        .keys()
        .find(|&&pin| {
            problem
                .members
                .iter()
                .filter(|member| {
                    (member.pin_a == pin || member.pin_b == pin) && member.tension.is_none()
                })
                .count()
                == 2
        })
        .map(|&pin| pin)
}
