use reqwest::blocking::Client;

const SESSION_COOKIE: &str = "53616c7465645f5f047dfdc64e9b3eb1fbb994e245d3d4607c566131c8826871ce664aa82e82bcf0eb2992b835c3d82f862765aad2b493c4c0eb831609d98338";

pub fn fetch_input(day: u32) -> Option<String> {
    let client = Client::new();
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);

    if let Ok(response) = client.get(&url).header("Cookie", format!("session={}", SESSION_COOKIE)).send() {
        if response.status().is_success() {
            if let Ok(content) = response.text() {
                return Some(content);
            }
        }
    }

    None
}
