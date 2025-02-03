use std::fs;
use dotenv_codegen;

use crate::trie;

fn load_dictionary() -> String {
  let dictionary_path = dotenv_codegen::dotenv!("DICTIONARY_PATH");
  let dictionary_content = fs::read_to_string(dictionary_path)
    .expect("Failed to read dictionary file");

  dictionary_content
}

pub fn load_trie() -> Box<trie::Trie> {
  let mut new_trie = Box::new(trie::Trie::new());
  let words = load_dictionary();

  for ln in words.lines() {
    trie::insert_key(&mut new_trie, &ln.to_string().to_lowercase());
  }

  new_trie
}