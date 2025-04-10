mod trie;
mod loader;
mod solver;

use std::io;

fn main() {
    let root = loader::load_trie();
    const GRID_SIZE: usize = 5;
    let mut word_hunt_solver = solver::Solver::new(root, GRID_SIZE, GRID_SIZE);

    println!("Welcome to the Word Hunt solver.\n");

    let mut input = String::new();

    loop {
        let mut game_board = vec![vec![' '; GRID_SIZE]; GRID_SIZE];

        println!("Enter the letters for the {}x{} game board:", GRID_SIZE, GRID_SIZE);

        for i in 0..GRID_SIZE {
            println!("Enter row {} ({} characters):", i + 1, GRID_SIZE);
            let mut row_input = String::new();
            io::stdin().read_line(&mut row_input).expect("Failed to read input");
            let row_input = row_input.trim();

            if row_input.len() != GRID_SIZE {
                println!("Each row must have exactly {} characters. Please try again.", GRID_SIZE);
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