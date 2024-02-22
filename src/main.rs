use chrono::offset;
use clap::{ArgAction, Parser};
use fstrings;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::stdin;
use std::io::Result;
use std::path::Path;

#[derive(Parser)]
struct Args {
    #[arg(long, short = 'i', action=ArgAction::SetTrue)]
    input: bool,

    #[arg(long, short = 'd', action=ArgAction::SetTrue)]
    debug: bool,
}

struct Entry {
    trigger: String,
    obsession: String,
    compulsion: String,
    response: String,
    alternative: String,
    date_time: chrono::NaiveDate,
}

fn get_date_time() -> chrono::NaiveDate {
    let give_this = offset::Local::now().date_naive();
    give_this
}

fn get_obsession() -> String {
    println!("Please input any obs: ");
    let mut input = String::new();
    let input_in = io::stdin();
    input_in.read_line(&mut input).unwrap();
    input = String::from(input.trim());
    if input.is_empty() {
        println!("You did not input a value.");
    }
    input
}

fn get_trigger() -> String {
    println!("Please input any trigger: ");
    let mut input = String::new();
    let input_in = io::stdin();
    input_in.read_line(&mut input).unwrap();
    input = String::from(input.trim());
    if input.is_empty() {
        println!("You did not input a value.");
    }
    input
}

fn get_compulsion() -> String {
    println!("Please input your compulsion: ");
    let mut input = String::new();
    let input_in = io::stdin();
    input_in.read_line(&mut input).unwrap();
    input = String::from(input.trim());
    if input.trim().is_empty() {
        println!("You did not input a value.");
    }
    input
}

fn get_response() -> String {
    println!("Please input your response: ");
    let mut input = String::new();
    let input_in = io::stdin();
    input_in.read_line(&mut input).unwrap();
    input = String::from(input.trim());
    if input.is_empty() {
        println!("You did not input a value.");
    }
    input
}

fn get_alternative() -> String {
    println!("Please input your alternative thought: ");
    let mut input = String::new();
    let input_in = io::stdin();
    input_in.read_line(&mut input).unwrap();
    input = String::from(input.trim());
    if input.is_empty() {
        println!("You did not input a value.");
    }
    input
}

fn add_data(entry: Entry, file: &mut File, existing_data: Option<Vec<String>>) -> Result<()> {
    let mut write_content: &[u8];
    let mut formatted_string: String;
    if existing_data != None {
        let former_string: String = String::from("--FORMER DATA--\n");
        let former_data = existing_data.unwrap().join("\n") + "\n\n";
        let full_former_data = former_string + &former_data;
        formatted_string = format!(
            "Trigger: {y}\nObsession: {z}\nCompulsion: {v}\nResponse: {b}\nAlternative: {n}\n\n",
            y = entry.trigger,
            z = entry.obsession,
            v = entry.compulsion,
            b = entry.response,
            n = entry.alternative,
        );
        write_content = *&formatted_string.as_bytes();
        file.write_all(full_former_data.as_bytes()).unwrap();
        file.write_all(write_content).expect("Could not write.");
    } else {
        formatted_string = format!(
            "Trigger: {y}\nObsession: {z}\nCompulsion: {v}\nResponse: {b}\nAlternative: {n}",
            y = entry.trigger,
            z = entry.obsession,
            v = entry.compulsion,
            b = entry.response,
            n = entry.alternative,
        );
        write_content = *&formatted_string.as_bytes();
        file.write_all(write_content).expect("Could not write.");
    }
    Ok(())
}

fn exist_check(entry: Entry) -> Result<()> {
    let date_time_str = String::from(entry.date_time.to_string()) + ".txt";
    let path = Path::new(&date_time_str);
    let exists = path.exists();
    if exists {
        let file_data: Vec<String> = fs::read_to_string(path)
            .expect("Unable to open file.")
            .lines()
            .map(String::from)
            .collect();
        let mut file = File::create(path).expect("Unable to make file.");
        let _ = add_data(entry, &mut file, Some(file_data));
    } else {
        let mut file = File::create(path).expect("Unable to make file.");
        let _ = add_data(entry, &mut file, None);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.input == true {
        let entry = Entry {
            trigger: get_trigger(),
            obsession: get_obsession(),
            compulsion: get_compulsion(),
            response: get_response(),
            alternative: get_alternative(),
            date_time: get_date_time(),
        };
        let _ = exist_check(entry);
    }

    Ok(())
}
