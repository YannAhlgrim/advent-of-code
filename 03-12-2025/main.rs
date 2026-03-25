use std::fs;

fn main() {
    // let input = [
    //     "987654321111111",
    //     "811111111111119",
    //     "234234234234278",
    //     "818181911112111",
    // ];

    let input: String =
        fs::read_to_string("input.txt").expect("text file cannot be read to string");
    let input = input.split("\n");
    let mut res = 0;

    for el in input {
        if el == "" {
            break;
        }
        let length = el.chars().count() - 1;
        let numbers: Vec<u32> = el
            .chars()
            .collect::<Vec<_>>()
            .iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let first_num = max_number(&numbers, 0, length);
        let second_num = max_number(&numbers, first_num.1 + 1, length + 1);
        res += concat_f(first_num.0, second_num.0);
    }
    println!("{}", res);
}

fn max_number(numbers: &Vec<u32>, start: usize, end: usize) -> (u32, usize) {
    let mut max: u32 = numbers[start];
    let mut max_idx: usize = 0;
    for i in start..end {
        if numbers[i] > max {
            max = numbers[i];
            max_idx = i;
        }
    }
    (max, max_idx)
}

fn concat_f(a: u32, b: u32) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}
