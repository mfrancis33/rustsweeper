mod board;

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

    // Error check stuff
    if width < 1 || height < 1 || num_mines < 1 || num_mines > width * height - 1 {
        eprintln!("Could not generate board from configuration!");
        return;
    }

    // Generate board
    let board = board::Board::generate_board(width, height, num_mines);

    // Explain how to play
    println!("\n\n- HOW TO PLAY ------");
    println!("  Reveal all spaces without blowing up!");
    println!("  Numbers tell you how many mines touch it.");
    println!("  Flag a spot to remind yourself not to reveal it.");

    println!("\n  On your turn, type a command. Commands are:");
    println!("    `SPACE`      - reveal the given space");
    println!("    `flag SPACE` - flag (or unflag) the given space");
    println!("    `quit`       - quits the game");

    // Main logic time
    println!("\n\n- GO! --------------");
    loop {
        // Print the board
        board.print();

        // Get user input and deal with it
        println!("What will you do?");
        let mut str_inp = String::new();
        io::stdin()
            .read_line(&mut str_inp)
            .expect("Failed to read from stdin!");
        break; // TODO: Remove once I am confident in my own code

        // Figure out if user lost and deal with that
    }
}
