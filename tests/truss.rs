use truss::{input::load_input_file, member, model::TrussProblem, rigid_body};

const EPSILON: f64 = 0.010;

#[test]
fn triangle() {
    let mut problem = load_input_file("examples/triangle.txt");
    let expected = load_input_file("examples/triangle-solution.txt");

    rigid_body::solve_misc_forces(&mut problem);
    member::solve_member_forces(&mut problem);

    assert_problem_equal(problem, expected);
}

#[test]
fn truss0() {
    let mut problem = load_input_file("examples/truss0.txt");
    let expected = load_input_file("examples/truss0-solution.txt");

    rigid_body::solve_misc_forces(&mut problem);
    member::solve_member_forces(&mut problem);

    assert_problem_equal(problem, expected);
}

fn assert_problem_equal(problem: TrussProblem, expected: TrussProblem) {
    assert_eq!(problem.pins, expected.pins);

    for (member, expected) in problem.members.iter().zip(&expected.members) {
        assert_eq!(member.pin_a, expected.pin_a);
        assert_eq!(member.pin_b, expected.pin_b);
        println!(
            "member {}{} {:?} {:?}",
            member.pin_a, member.pin_b, member.tension, expected.tension
        );
        assert!((member.tension.unwrap() - expected.tension.unwrap()).abs() < EPSILON);
    }

    for (force, expected) in problem.misc_forces.iter().zip(&expected.misc_forces) {
        assert_eq!(force.pin, expected.pin);
        println!(
            "force {} {:?},{} {:?},{}",
            force.pin, force.magnitude, force.direction, expected.magnitude, expected.direction,
        );
        assert!((force.magnitude.unwrap() - expected.magnitude.unwrap()).abs() < EPSILON);
        assert_eq!(force.direction, expected.direction);
    }
}
