use crate::trie;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Solver {
  dictionary: Box<trie::Trie>,
  row_size: usize,
  col_size: usize,
  len_to_points: HashMap<usize, usize>,
}

impl Solver {
  pub fn new(dictionary: Box<trie::Trie>, row_size: usize, col_size: usize) -> Self {
    let mut len_to_points = HashMap::new();

    len_to_points.insert(3, 100);
    len_to_points.insert(4, 400);
    len_to_points.insert(5, 800);
    for i in 6..17 {
        len_to_points.insert(i, 1400 + (400 * (i - 6)));
    }

    Solver {
        dictionary,
        row_size,
        col_size,
        len_to_points,
    }
  }

  pub fn print_board(&self, board: &Vec<Vec<char>>) {
    println!("Current Game Board:");
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
    println!();
  }

  pub fn sort_array(&self, answers: &mut Vec<String>) {
    answers.sort_by(|a, b| b.len().cmp(&a.len()));
  }

  pub fn solve(&mut self, board: &Vec<Vec<char>>, target: &mut String) -> Vec<String> {
    let mut found = HashSet::<String>::new();
    let mut answers = Vec::<String>::new();
    let mut visited = vec![vec![false; self.col_size]; self.row_size];

    for r in 0..self.row_size {
      for c in 0..self.col_size {
        self.recurse(board, target, r as i32, c as i32, &mut visited, &mut found, &mut answers);
      }
    }

    answers
  }

  fn recurse(&mut self, board: &Vec<Vec<char>>, target: &mut String, row: i32, col: i32, visited: &mut Vec<Vec<bool>>, found: &mut HashSet<String>, answers: &mut Vec<String>) {
    if row < 0 || col < 0 || col as usize >= self.col_size || row as usize >= self.row_size || visited[row as usize][col as usize] {
      return;
    }

    if !trie::is_prefix(&mut self.dictionary, target) {
      return;
    }

    let letter = board[row as usize][col as usize];
    target.push(letter);
    visited[row as usize][col as usize] = true;

    self.recurse(board, target, row + 1, col + 1, visited, found, answers);
    self.recurse(board, target, row + 1, col, visited, found, answers);
    self.recurse(board, target, row + 1, col - 1, visited, found, answers);
    self.recurse(board, target, row, col + 1, visited, found, answers);
    self.recurse(board, target, row, col - 1, visited, found, answers);
    self.recurse(board, target, row - 1, col + 1, visited, found, answers);
    self.recurse(board, target, row - 1, col, visited, found, answers);
    self.recurse(board, target, row - 1, col - 1, visited, found, answers);

    if trie::search_key(&mut self.dictionary, target) && !found.contains(target) {
      found.insert(target.clone());
      answers.push(target.clone());
    }

    visited[row as usize][col as usize] = false;
    target.pop();
  }

  pub fn print_scores(&self, answers: &Vec<String>) {
    for word in answers {
      let word_length = word.len();
      if let Some(&points) = self.len_to_points.get(&word_length) {
        println!("Word: {}, Length: {}, Points: {}", word, word_length, points);
      } else {
        println!("Word: {}, Length: {}, Points: 0 (No points assigned)", word, word_length);
      }
    }
}
}