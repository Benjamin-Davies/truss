use peroxide::prelude::*;

use crate::{model::TrussProblem, vec2::Vec2};

pub fn solve_misc_forces(problem: &mut TrussProblem) {
    // A * Force Magnitudes = 0
    // Row 1 = F_x coefficient
    // Row 2 = F_y coefficient
    // Row 3 = M_O coefficient
    let matrix_a = problem
        .misc_forces
        .iter()
        .map(|force| {
            let position = problem.pins.get(&force.pin).unwrap().0;
            let normal = Vec2::from_angle(force.direction);
            [normal.x(), normal.y(), position.cross(normal)]
        })
        .collect::<Vec<[f64; 3]>>();

    // B * Unknown Force Magnitudes + C * Known Force Magnitudes = 0
    let unknown_forces = problem
        .misc_forces
        .iter()
        .enumerate()
        .filter_map(|(i, f)| if f.magnitude.is_none() { Some(i) } else { None })
        .collect::<Vec<usize>>();
    let known_forces = problem
        .misc_forces
        .iter()
        .enumerate()
        .filter_map(|(i, f)| f.magnitude.map(|m| (i, m)))
        .collect::<Vec<(usize, f64)>>();

    let matrix_b = matrix(
        unknown_forces.iter().flat_map(|&i| matrix_a[i]).collect(),
        3,
        unknown_forces.len(),
        Col,
    );
    let matrix_c = matrix(
        known_forces
            .iter()
            .flat_map(|&(i, _)| matrix_a[i])
            .collect(),
        3,
        known_forces.len(),
        Col,
    );
    let known_force_manitudes = matrix(
        known_forces.iter().map(|&(_, f)| f).collect(),
        known_forces.len(),
        1,
        Row,
    );

    // B * b + C * c = 0 => B * b = -(C * c)
    let unknown_force_magnitudes = solve(&matrix_b, &-(matrix_c * known_force_manitudes));

    for (&i, magnitude) in unknown_forces.iter().zip(unknown_force_magnitudes.data) {
        problem.misc_forces[i].magnitude = Some(magnitude);
    }
}
