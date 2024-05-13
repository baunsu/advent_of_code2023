mod read_lines;

fn main() {
    let mut two_digit_numbers: Vec<u32> = vec![];
    if let Ok(lines) = read_lines::read_lines("./input.txt") {
        for line in lines.flatten() {
            let mut numbers = vec![];
            for c in line.chars() {
                if c.is_numeric() {
                    numbers.push(c);
                }
            }

            let mut two_digits = String::new();
            two_digits.push(numbers[0]);

            if numbers.len() != 0 {
                two_digits.push(numbers[numbers.len()-1]);
            } else {
                two_digits.push(numbers[0]);
            }   

            two_digit_numbers.push(two_digits.parse::<u32>().unwrap());
        }
    }

    println!("Result: {:?}", two_digit_numbers.into_iter().fold(0, |i, acc| i + acc ));

}