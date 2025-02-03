pub struct Trie {
  child: [Option<Box<Trie>>; 26],
  word_end: bool,
}

impl Trie {
  pub fn new() -> Self {
    Trie {
      child: std::array::from_fn(|_| None),
      word_end: false
    }
  }
}

pub fn insert_key(root: &mut Box<Trie>, key: &String) -> () {
  let mut curr = root;

  for c in key.chars() {
    if curr.child[c as usize - 'a' as usize].is_none() {
      let new_node = Some(Box::new(Trie::new()));
      curr.child[c as usize - 'a' as usize] = new_node;
    }

    curr = curr.child[c as usize - 'a' as usize].as_mut().unwrap();
  }
  curr.word_end = true;
}

pub fn search_key(root: &mut Box<Trie>, key: &String) -> bool {
  let mut curr = root;
  for c in key.chars() {
    let index = c as usize - 'a' as usize;
    if curr.child[index].is_none() {
      return false
    }

    curr = curr.child[index].as_mut().unwrap();
  }

  curr.word_end
}

pub fn is_prefix(root: &mut Box<Trie>, key: &String) -> bool {
  let mut curr = root;
  for c in key.chars() {
    let index = c as usize - 'a' as usize;
    if curr.child[index].is_none() {
      return false
    }
    curr = curr.child[index].as_mut().unwrap();
  }

  true
}