use std::cmp;
use std::io;
use std::io::Write;

fn validate_int(prompt: String, default: i32) -> i32 {
    let mut str_inp = String::new();
    let ret: i32;
    print!("{}", prompt);
    io::stdout()
        .flush()
        .expect("Could not flush buffer!");

    // Make sure input is valid
    io::stdin()
        .read_line(&mut str_inp)
        .expect("Failed to read from stdin!");

    // Make sure string is a number
    let trimmed = str_inp.trim();
    if let Ok(i) = trimmed.parse::<i32>() {
        ret = i;
    } else {
        // It is not valid. Boowomp.
        ret = default;
        eprintln!("Invalid integer! Defaulting to {}", default);
    }

    return ret;
}

fn main() {
    println!("====================\nRUSTSWEEPER\nby Michael Francis\n\n");

    // Setup game info
    println!("- SETUP ------------");
    let width = validate_int("  Width: ".to_string(), 20);
    let height = validate_int("  Height: ".to_string(), 20);
    let num_mines = validate_int("  Mines: ".to_string(), cmp::min(width * height / 2, 99));

    // Generate board
    print!("DEBUG: w {} h {} m {}", width, height, num_mines);
}
