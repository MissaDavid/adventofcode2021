use crate::util::read_lines;

pub fn read_instructions() {
    let mut sub_horizontal: i32 = 0;
    let mut sub_depth: i32 = 0;

    if let Ok(lines) = read_lines("./puzzle_inputs/input_2.txt") {
        for line in lines {
            if let Ok(input) = line {
                let mut iter = input.split_whitespace();
                let command = iter.next().unwrap();
                let x = iter.next().unwrap().parse::<i32>().unwrap();

                if command == "forward" { sub_horizontal = sub_horizontal + x } else if command == "up" { sub_depth = sub_depth - x } else { sub_depth = sub_depth + x };
            }
        }
    }
    println!("Result is {}", sub_horizontal * sub_depth);
}
