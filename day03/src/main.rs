use regex_lite::Regex;

const TRIVIAL_EXAMPLE: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TRIVIAL_RESULT: usize = 161;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("couldn't compile regex");

    let total: usize = re
        .captures_iter(TRIVIAL_EXAMPLE)
        .map(|m| m.extract().1)
        .enumerate()
        .map(|(idx, [a, b])| {
            let x: usize = a
                .parse()
                .expect(&format!("failed to parse first arg of match {}", idx));
            let y: usize = b
                .parse()
                .expect(&format!("failed to parse first arg of match {}", idx));
            println!("{} * {}", x, y);
            x * y
        })
        .sum();

    println!("got: {}", total);
}
