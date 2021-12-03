use crate::util::read_lines;


pub fn get_measurements() {
    let mut previous_value = 188; // starting value
    let mut current_value= 0;
    let mut counter = 0;

    // read lines from txt file
    if let Ok(lines) = read_lines("./puzzle_inputs/input_1.txt") {
        // Consumes the iterator
        for line in lines {
            if let Ok(input) = line {
                let sonar_sweep = input.parse::<i32>().unwrap();
                current_value = sonar_sweep;
                if current_value > previous_value {
                    counter = counter + 1;
                }
            }
            previous_value = current_value;
        }
    }
    println!("Measurements increased {} times !", counter);
}

