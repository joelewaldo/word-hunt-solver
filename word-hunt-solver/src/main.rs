mod trie;
mod loader;
mod solver;

use std::io;

fn main() {
    let root = loader::load_trie();
    let mut word_hunt_solver = solver::Solver::new(root, 4, 4);

    println!("Welcome to the Word Hunt solver.\n");

    let mut input = String::new();

    loop {
        let mut game_board = vec![vec![' '; 4]; 4];

        println!("Enter the letters for the 4x4 game board:");

        for i in 0..4 {
            println!("Enter row {} (4 characters):", i + 1);
            let mut row_input = String::new();
            io::stdin().read_line(&mut row_input).expect("Failed to read input");
            let row_input = row_input.trim();

            if row_input.len() != 4 {
                println!("Each row must have exactly 4 characters. Please try again.");
                return;
            }

            for (j, c) in row_input.chars().enumerate() {
                game_board[i][j] = c;
            }
        }

        word_hunt_solver.print_board(&game_board);

        let mut target = String::from("");
        let mut answers = word_hunt_solver.solve(&game_board, &mut target);

        word_hunt_solver.sort_array(&mut answers);
        println!("Found words:");
        word_hunt_solver.print_scores(&answers);

        println!("Do you want to do it again? (Y|N)");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let first_char = input.trim().chars().next().unwrap_or('N');
        if first_char != 'Y' && first_char != 'y' {
            break;
        }
    }
}