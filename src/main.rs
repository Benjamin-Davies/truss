use input::load_input_file;

pub mod input;
pub mod member;
pub mod model;
pub mod rigid_body;
pub mod vec2;

fn main() {
    let input = load_input_file("examples/triangle.txt");
    let mut problem = input.clone();
    println!("{:?}", problem);

    rigid_body::solve_misc_forces(&mut problem);
    println!("{:?}", problem);

    member::solve_member_forces(&mut problem);
    println!("{:?}", problem);
}
