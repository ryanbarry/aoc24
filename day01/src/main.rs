const TRIVIAL_EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

fn main() {
    let (list1, list2) = collect_lists(TRIVIAL_EXAMPLE);
    let dtotal = total_distance(&list1, &list2);
    println!("[trivial] total distance = {}", dtotal);

    let input_text = String::from_utf8(std::fs::read("input.txt").expect("couldn't read input file"))
        .expect("couldn't parse input file into utf-8");

    let (list1, list2) = collect_lists(&input_text);
    let dtotal = total_distance(&list1, &list2);
    println!("[input]   total distance = {}", dtotal);
}

/// given the raw input as a string, returns the two lists sorted ascending
fn collect_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for (idx, line) in input.split_terminator('\n').enumerate() {
        let lineno = idx + 1;
        let strings: Vec<_> = line.split_ascii_whitespace().collect();
        assert_eq!(strings.len(), 2, "line {} doesn't have 2 numbers", lineno);
        let num1: usize = strings[0]
            .parse()
            .expect(format!("first number on line {} doesn't parse", lineno).as_str());
        let num2: usize = strings[1]
            .parse()
            .expect(format!("second number on line {} doesn't parse", lineno).as_str());
        list1.push(num1);
        list2.push(num2);
    }

    (list1, list2)
}

/// given two SORTED lists, return sum of pairwise distances
fn total_distance(list1: &Vec<usize>, list2: &Vec<usize>) -> usize {
    let list1 = {
        let mut tmp = list1.clone();
        tmp.sort();
        tmp
    };
    let list2 = {
        let mut tmp = list2.clone();
        tmp.sort();
        tmp
    };
    let total_distance = list1
        .iter()
        .zip(list2.clone())
        .fold(0, |acc, (x, y)| acc + x.abs_diff(y));

    total_distance
}
