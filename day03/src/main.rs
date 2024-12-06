use regex_lite::Regex;

const TRIVIAL_EXAMPLE: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TRIVIAL_EXAMPLE2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() {
    let total = parse_and_exec(TRIVIAL_EXAMPLE);
    println!("[trivial] {}", total);

    let input = match std::fs::read_to_string("input.txt") {
        Ok(contents) => contents,
        Err(err) => panic!("failed to open input file: {err:?}"),
    };

    let total = parse_and_exec(&input);
    println!("[partone] {}", total);

    let total = parse_and_exec_p2(TRIVIAL_EXAMPLE2);
    println!("[2rivial] {}", total);
    let total = parse_and_exec_p2(&input);
    println!("[parttwo] {}", total);
}

#[test]
fn test_trivial() {
    let result = parse_and_exec(TRIVIAL_EXAMPLE);
    assert_eq!(result, 161, "trivial results is incorrect");
}

fn parse_and_exec(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .expect("couldn't compile regex");

    re.captures_iter(input)
        .map(|m| m.extract().1)
        .enumerate()
        .map(|(idx, [a, b])| {
            let x: usize = a
                .parse()
                .expect(&format!("failed to parse first arg of match {}", idx));
            let y: usize = b
                .parse()
                .expect(&format!("failed to parse first arg of match {}", idx));
            x * y
        })
        .sum()
}

fn parse_and_exec_p2(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do(\()(\))|don't(\()(\))")
        .expect("couldn't compile regex");
    let mut enabled = true;
    enum EnSignal {
        Enabled,
        Disabled,
        Unchanged,
    }

    re.captures_iter(input)
        .map(|m| {
            let (fullstr, capts) = m.extract();
            if fullstr.starts_with("don't") {
                (EnSignal::Disabled, capts)
            } else if fullstr.starts_with("do") {
                (EnSignal::Enabled, capts)
            } else {
                (EnSignal::Unchanged, capts)
            }
        })
        .enumerate()
        .filter_map(|(idx, (newen, [a, b]))| match newen {
            EnSignal::Enabled => {
                enabled = true;
                None
            }
            EnSignal::Disabled => {
                enabled = false;
                None
            }
            EnSignal::Unchanged => {
                let x: usize = a
                    .parse()
                    .expect(&format!("failed to parse first arg of match {}", idx));
                let y: usize = b
                    .parse()
                    .expect(&format!("failed to parse first arg of match {}", idx));
                if enabled {
                    Some(x * y)
                } else {
                    Some(0)
                }
            }
        })
        .sum()
}
