use regex_lite::Regex;

const TRIVIAL_EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TRIVIAL_RESULT: usize = 161;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("couldn't compile regex");

    for (idx, capt) in re.captures_iter(TRIVIAL_EXAMPLE).enumerate() {
        let a = capt.get(1).expect(&format!("couldn't get capture 1 from line {}", idx));
        let b = capt.get(2).expect(&format!("couldn't get capture 2 from line {}", idx));
        println!("{} * {}", a.as_str(), b.as_str());
    }
}
