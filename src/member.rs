use peroxide::{c, prelude::*};

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

    // B * Unknown Tensions + C * Known Tensions = 0
    let unknown_tensions = members
        .iter()
        .enumerate()
        .filter_map(|(i, (_, m))| if m.tension.is_none() { Some(i) } else { None })
        .collect::<Vec<usize>>();
    let known_tensions = members
        .iter()
        .enumerate()
        .filter_map(|(i, (_, m))| m.tension.map(|t| (i, t)))
        .collect::<Vec<(usize, f64)>>();

    let matrix_b = matrix(
        unknown_tensions
            .iter()
            .flat_map(|&i| matrix_a[i].0)
            .collect(),
        2,
        unknown_tensions.len(),
        Col,
    );
    let matrix_c = matrix(
        known_tensions
            .iter()
            .map(|&(i, _)| i)
            .chain(members.len()..members.len() + forces.len())
            .flat_map(|i| matrix_a[i].0)
            .collect(),
        2,
        known_tensions.len() + forces.len(),
        Col,
    );
    let known_force_magnitudes = matrix(
        known_tensions
            .iter()
            .map(|&(_, t)| t)
            .chain(forces.iter().map(|f| f.magnitude.unwrap()))
            .collect(),
        known_tensions.len() + forces.len(),
        1,
        Row,
    );

    // B * b + C * c = 0 => B * b = -(C * c)
    let unknown_tension_values = if matrix_b.col >= 2 {
        solve(&matrix_b, &-(matrix_c * known_force_magnitudes))
    } else {
        // If we only have one unknown, then pick the row that gives us the most accuracy
        let (i, matrix_b_entry) = matrix_b
            .data
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs()).unwrap())
            .unwrap();
        matrix(
            c![-(matrix_c * known_force_magnitudes).data[i] / matrix_b_entry],
            1,
            1,
            Col,
        )
    };

    for (&i, tension) in unknown_tensions.iter().zip(unknown_tension_values.data) {
        let (j, _) = members[i];
        problem.members[j].tension = Some(tension);
    }
}

fn find_solvable_pin(problem: &TrussProblem) -> Option<char> {
    problem
        .pins
        .keys()
        .find(|&&pin| {
            let unknowns = problem
                .members
                .iter()
                .filter(|member| {
                    (member.pin_a == pin || member.pin_b == pin) && member.tension.is_none()
                })
                .count();
            0 < unknowns && unknowns <= 2
        })
        .map(|&pin| pin)
}
