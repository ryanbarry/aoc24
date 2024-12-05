const TRIVIAL_EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

fn main() {
    let num_safe = count_safe_lines_p1(TRIVIAL_EXAMPLE.split_terminator('\n'));
    println!("[trivial] number of safe reports: {}", num_safe);

    if let Ok(input_reports) = std::fs::read_to_string("input.txt") {
        let report_lines = input_reports.split_terminator('\n');
        let num_safe = count_safe_lines_p1(report_lines.clone());
        println!("[partone] number of safe reports: {}", num_safe);
        let num_safe = count_safe_lines_p2(report_lines.clone());
        println!("[parttwo] number of safe reports: {}", num_safe);
    } else {
        println!("could not read input file");
    }
}

fn count_safe_lines_p2<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .map(|line| {
            line
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("couldn't parse number"))
                .collect::<Vec<usize>>()
        })
        .filter_map(|report| {
            (0..report.len())
                .map(|idx| {
                    let with_one_removed = {
                        let mut tmp = report.clone();
                        tmp.remove(idx);
                        tmp
                    };
                    check_report_safety(&with_one_removed)
                })
                .any(|x| x).then_some(())
        }).count()
}

fn count_safe_lines_p1<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .map(|line| {
            line
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("couldn't parse number"))
                .collect::<Vec<usize>>()
        })
        .filter_map(|line| check_report_safety(&line).then_some(()))
        .count()
}

fn all_inc_or_dec(numlist: &Vec<usize>) -> bool {
    numlist.is_sorted_by(|a, b| a <= b) || numlist.is_sorted_by(|a, b| a >= b)
}

fn check_report_safety(report: &Vec<usize>) -> bool {
    let adj_diff_small = report.windows(2).all(|elems| {
        let ad = elems[0].abs_diff(elems[1]);
        ad <= 3 && ad >= 1
    });

    all_inc_or_dec(report) && adj_diff_small
}
