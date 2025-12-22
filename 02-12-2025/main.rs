use std::fs;

fn main() {
    // let input = [
    //     "11-22",
    //     "95-115",
    //     "998-1012",
    //     "1188511880-1188511890",
    //     "222220-222224",
    //     "1698522-1698528",
    //     "446443-446449",
    //     "38593856-38593862",
    //     "565653-565659",
    //     "824824821-824824827",
    //     "2121212118-2121212124",
    // ];
    let input: String =
        fs::read_to_string("input.txt").expect("text file cannot be read to string");
    let input = input.split(",");

    let mut sum: i64 = 0;

    for range in input {
        if range == "" {
            break;
        }
        let range = range.replacen("\n", "", 1);
        let numbers = range.split("-");
        let numbers: Vec<&str> = numbers.collect();
        let start_number: i64 = numbers[0].parse::<i64>().unwrap();
        let end_number: i64 = numbers[1].parse::<i64>().unwrap();

        for n in start_number..(end_number + 1) {
            let n_string: String = n.to_string();
            if n_string.len() % 2 == 0 {
                let first_slice = &n_string[..(n_string.len() / 2)];
                let second_slice = &n_string[(n_string.len() / 2)..];

                if first_slice == second_slice {
                    println!("Found an id: {first_slice}{second_slice}");
                    sum += i64::from(n);
                    continue;
                }
            }
        }
    }
    println!("The sum of all invalid ids: {sum}");
}
