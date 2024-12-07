use aoc_2024_day2::puzzle_input::PuzzleInput;

fn main() {
    println!("solving part 1...");

    let input_string = include_str!("../../../input/example.txt");
    let input = PuzzleInput::from(input_string);

    println!("{}", input.reports()[0]);
}
