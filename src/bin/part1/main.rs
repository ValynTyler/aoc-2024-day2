use aoc_2024_day2::puzzle_input::PuzzleInput;

fn main() {
    println!("solving part 1...");

    let input_string = include_str!("../../../input/numbers.txt");
    let input = PuzzleInput::from(input_string);

    println!("{}", input);

    let mut sum = 0;
    for item in input.reports() {
        let is_safe = item.is_safe();
        if is_safe { sum += 1 };

        println!("{}", is_safe);
    }

    println!("{}", sum);
}
