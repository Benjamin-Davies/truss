use std::env;

use input::load_input_file;
use model::TrussProblem;

pub mod input;
pub mod member;
pub mod model;
pub mod rigid_body;
pub mod vec2;

fn main() {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "truss0.txt".to_owned());
    let path = env::current_dir().unwrap().join("examples").join(filename);
    let mut problem = load_input_file(path);

    rigid_body::solve_misc_forces(&mut problem);
    member::solve_member_forces(&mut problem);

    print_problem(&problem);
}

fn print_problem(problem: &TrussProblem) {
    for (name, pin) in &problem.pins {
        println!("pin {} {:5.2} {:5.2}", name, pin.0.x(), pin.0.y());
    }
    println!();
    for member in &problem.members {
        println!(
            "member {}{} {}",
            member.pin_a,
            member.pin_b,
            member
                .tension
                .map(|t| format!("{:6.3}", t))
                .unwrap_or("-".to_string())
        );
    }
    println!();
    for force in &problem.misc_forces {
        println!(
            "force {} {} {:5.1}",
            force.pin,
            force
                .magnitude
                .map(|t| format!("{:6.3}", t))
                .unwrap_or("-".to_string()),
            force.direction
        );
    }
    println!();
}
