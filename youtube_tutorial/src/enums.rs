// Enums are types which have a few definite values.
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match  m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")

    }
}
pub fn run() {
    let avatar_one = Movement::Left;
    let avatar_two = Movement::Up;
    let avatar_three = Movement::Right;
    let avatar_four = Movement::Down;

    move_avatar(avatar_one);
    move_avatar(avatar_two);
    move_avatar(avatar_three);
    move_avatar(avatar_four);


}
