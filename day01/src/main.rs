mod input_fetcher;

fn count_characters_per_line(input_text: &str) -> Vec<usize> {
    input_text
        .lines()
        .map(|line| line.chars().count())
        .collect()
}

fn main() {
    let day_to_fetch = 1; // Specify the day number you want to fetch

    if let Some(input) = input_fetcher::fetch_input(day_to_fetch) {
        println!("Fetched input data for Day {}:", day_to_fetch);
        // println!("{}", input);

        let character_counts = count_characters_per_line(&input);

        for (line_number, &count) in character_counts.iter().enumerate() {
            println!("Line {}: {} characters", line_number + 1, count);
        }
        // Continue working with the fetched input data or character counts here
    } else {
        eprintln!("Error: Failed to fetch input data for Day {}", day_to_fetch);
        // Handle the error here if needed for the specified day
    }
}
