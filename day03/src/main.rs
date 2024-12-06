use regex_lite::Regex;

const TRIVIAL_EXAMPLE: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn main() {
    let total = parse_and_exec(TRIVIAL_EXAMPLE);
    println!("[trivial] {}", total);
}

#[test]
fn test_trivial() {
    let result = parse_and_exec(TRIVIAL_EXAMPLE);
    assert_eq!(result, 161, "trivial results is incorrect");
}

fn parse_and_exec(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("couldn't compile regex");

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
