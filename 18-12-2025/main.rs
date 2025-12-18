use std::fs;

fn main() {

    let mut pointer = 50;
    let mut zero_count = 0;

    let input: String = fs::read_to_string("input.txt").expect("text file cannot be read to string");
    let input = input.split("\n");
    // let input = ["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];

    for element in input {
        if element == "" {
            break;
        }
        let steps: i32 = (&element[1..]).parse().unwrap();
        if element.contains("L") {
           pointer -= steps; 
        } else {
            pointer += steps;
        }
        pointer = pointer % 100;
        if pointer < 0 {
            pointer += 100;
        }
        println!("The dial is rotated {element} to point at {pointer}");
        if pointer == 0 {
            zero_count += 1;
        }
    }
    println!("Number of Zeros: {zero_count}");
}

