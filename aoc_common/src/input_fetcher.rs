use reqwest::blocking::Client;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{env, fs};

pub fn get_or_fetch_input(day: u8) -> Result<String, reqwest::Error> {
    let input_path = PathBuf::from(format!("input/day{:02}.txt", day));
    if input_path.exists() {
        let mut file = File::open(&input_path).expect("Failed to open input file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read input file");

        return Ok(contents);
    }

    let session = env::var("AOC_SESSION").expect("AOC_SESSION not set in .env file");
    let year = env::var("AOC_YEAR").expect("AOC_YEAR not set in .env file");
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()?
        .text()?;

    if let Some(parent_path) = input_path.parent() {
        fs::create_dir_all(parent_path).expect("Failed to create input directory");
    }

    let mut file = File::create(&input_path).expect("Failed to create input file");
    file.write_all(response.as_bytes())
        .expect("Failed to write input file");

    Ok(response)
}
