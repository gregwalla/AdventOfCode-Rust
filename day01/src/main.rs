use std::fs::File;
use std::io::{self, BufWriter, Write};

mod input_fetcher;

fn process_line(input_text: &str) -> Vec<u32> {
    input_text
        .lines()
        .map(|line| {
            let line_numbers: Vec<u32> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            if let Some(first) = line_numbers.first() {
                let last = *line_numbers.last().unwrap_or(&first);
                format!("{}{}", first, last).parse().unwrap_or(0)
            } else {
                0
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let day_to_fetch = 1; // Specify the day number you want to fetch

    if let Some(input) = input_fetcher::fetch_input(day_to_fetch) {
        let concatenated_numbers = process_line(&input);

        let output_path = format!("target/concatenated_numbers_day_{}.txt", day_to_fetch);
        let mut output_file = BufWriter::new(File::create(&output_path)?);

        let mut sum = 0;
        for number in concatenated_numbers.iter() {
            let line_output = format!("{}\n", number);
            output_file.write_all(line_output.as_bytes())?;
            sum += number;
        }

        let sum_output_path = format!("target/sum_concatenated_numbers_day_{}.txt", day_to_fetch);
        let mut sum_output_file = BufWriter::new(File::create(sum_output_path)?);
        sum_output_file.write_all(sum.to_string().as_bytes())?;

        println!("Concatenated numbers per line saved to 'target/concatenated_numbers_day_{}.txt'", day_to_fetch);
        println!("Sum of concatenated numbers saved to 'target/sum_concatenated_numbers_day_{}.txt'", day_to_fetch);
    } else {
        eprintln!("Error: Failed to fetch input data for Day {}", day_to_fetch);
        // Handle the error here if needed for the specified day
    }

    Ok(())
}
