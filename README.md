# Word Hunt Solver - Rust Implementation

This is a Rust implementation of a Word Hunt Solver. The solver is designed to solve a Word Hunt or Boggle style game, where the goal is to find words by connecting adjacent letters on a 2D grid. The solver utilizes a Trie data structure to efficiently check if a word is valid or if it's a possible prefix of any valid word.

## Features

- Efficient Search: Uses a Trie to store the dictionary for fast prefix and word lookups.
- Backtracking: Recursively searches the board, exploring all possible letter connections.
- Word Length Scoring: Assigns points based on the length of the found words (following a predefined scoring system).
- Board Display: Prints the current state of the game board in a readable format.
- Sorting: Sorts the found words by length, from longest to shortest.

## Prerequisites

- Rust 1.65 or newer (for compiling and running the program)
- A valid dictionary file (Dictionary.txt) containing a list of valid words. The dictionary must be formatted with one word per line.

## Setup and Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/word-hunt-solver.git
cd word-hunt-solver
```

2. Add the dictionary file: Place a Dictionary.txt file in the root directory of the project. The file should contain one word per line.

3. Build the project: Run the following command to compile the project:

```bash
cargo build --release
```

4. Run the Solver: Once the project is built, you can run the solver by using:

```bash
cargo run
```

The solver will attempt to find all valid words on a provided game board.

## Scoring System

Words are assigned points based on their length:

- Words of length 3: 100 points
- Words of length 4: 400 points
- Words of length 5: 800 points
- Words of length 6 and above: 1400 + 400 \* (length - 6)

## License

This project is licensed under the MIT License
