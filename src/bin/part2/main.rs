use aoc_2024_day2::{puzzle_input::PuzzleInput, report::Report};

fn main() {
    println!("solving part 2...");

    let input_string = include_str!("../../../input/numbers.txt");
    let input = PuzzleInput::from(input_string);

    println!("{}", input);

    let mut sum = 0;
    for item in input.reports() {
        let levels = item.levels();
        let mut safe = false;

        println!("analizing: {}", item);
        for i in 0..levels.len() {
            let mut levels = levels.clone();
            levels.remove(i);

            let report = Report(levels);
            if report.is_safe() { safe = true };

            println!("  {} {}", report, safe);
        }

        println!("verdict: {}\n", safe);
        match safe {
            true => sum += 1,
            false => (),
        }
    }

    println!("{}", sum);
}
