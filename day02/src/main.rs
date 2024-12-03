const TRIVIAL_EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

fn main() {
    let num_safe = count_safe_lines(TRIVIAL_EXAMPLE.split_terminator('\n'));
    println!("[trivial] number of safe reports: {}", num_safe);

    if let Ok(part1_reports) = std::fs::read_to_string("input.txt") {
        let num_safe = count_safe_lines(part1_reports.split_terminator('\n'));
        println!("[partone] number of safe reports: {}", num_safe);
    } else {
        println!("could not read input file");
    }
}

fn count_safe_lines<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .map(|line| check_report_safety(line))
        .filter(|x| *x)
        .count()
}

fn check_report_safety(line: &str) -> bool {
    let levels = line
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("couldn't parse number"))
        .collect::<Vec<usize>>();

    let a = levels[0];
    let b = levels[0 + 1];
    let diff = a.abs_diff(b);
    if diff < 1 || diff > 3 {
        return false;
    }
    let decreasing = if a > b { true } else { false };
    for i in 1..levels.len() - 1 {
        let a = levels[i];
        let b = levels[i + 1];
        let diff = a.abs_diff(b);
        if diff < 1 || diff > 3 {
            return false;
        }
        if decreasing && a < b {
            return false;
        } else if !decreasing && a > b {
            return false;
        }
    }

    true
}
