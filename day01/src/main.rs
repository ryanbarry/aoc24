const TRIVIAL_EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

fn main() {
    let (list1, list2) = collect_lists(TRIVIAL_EXAMPLE);
    list1.iter().zip(list2).for_each(|(x, y)| {
        println!("{} {}", x, y);
    });
}

/// given the raw input as a string, returns the two lists sorted ascending
fn collect_lists(input: &str) -> (Vec<isize>, Vec<isize>) {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for (idx, line) in input.split_terminator('\n').enumerate() {
        let lineno = idx+1;
        let strings: Vec<_> = line.split_ascii_whitespace().collect();
        assert_eq!(strings.len(), 2, "line {} doesn't have 2 numbers", lineno);
        let num1: isize = strings[0].parse().expect(format!("first number on line {} doesn't parse", lineno).as_str());
        let num2: isize = strings[1].parse().expect(format!("second number on line {} doesn't parse", lineno).as_str());
        list1.push(num1);
        list2.push(num2);
    }

    (list1, list2)
}
