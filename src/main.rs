use std::io;

fn main() {
    let mut game_field: [u8; 9] = [0;9];    
    println!("Welcome to Tic Tac Toe!");
    while check_winner(game_field) == 0 {
    	print_game_field(game_field);
    	next_round(&mut game_field);
    }
    println!("Game is Over!");
    print_game_field(game_field);
}

fn print_game_field(game_field: [u8; 9]) {
    fn br() {
        println!("------");
    }

    br();
    println!("{} {} {}", game_field[0], game_field[1], game_field[2]);
    println!("{} {} {}", game_field[3], game_field[4], game_field[5]);
    println!("{} {} {}", game_field[6], game_field[7], game_field[8]);
    br();
}

fn chose(game_field: &mut [u8; 9], player: u8, chosen_field: usize) {
    game_field[chosen_field] = player;
}

fn next_round(game_field: &mut [u8; 9]) {
    let mut input = String::new();
    println!("Chose from 1-9:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let chosen_field: usize = input.trim().parse().unwrap();
    println!("your input: {}", chosen_field);
    chose(game_field, 1, chosen_field);
    print_game_field(*game_field);
    println!("Computer is chosing...");
    chose(game_field, 2, 1);
    print_game_field(*game_field);
}

fn check_winner(game_field: [u8; 9]) -> u8 {
    println!("checking for a winner...");
    // check column
    for i in 0..3 {
        // player1
        if game_field[i] == 1 && game_field[i+3] == 1 && game_field[i+6] == 1 {
            return 1;
        }
        // computer
        if game_field[i] == 2 && game_field[i+3] == 2 && game_field[i+6] == 2 {
            return 2;
        }
    }

    // check row
    let mut i: usize = 0;
    loop {
        // player1
        if game_field[i] == 1 && game_field[i+1] == 1 && game_field[i+2] == 1 {
            return 1;
        }
        // computer
        if game_field[i] == 2 && game_field[i+1] == 2 && game_field[i+2] == 2 {
            return 2;
        }
        
        i += 3;
        if i > 6 {
            break;
        }
    }

    // check Diagonal
    for i in 0..2 {
        if game_field[0] == i && game_field[4] == i && game_field[8] == i {
            return i;
        }
        if game_field[2] == i && game_field[4] == i && game_field[6] == i {
            return i;
        }
    }

    return 0
}
