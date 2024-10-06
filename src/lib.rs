use std::clone::Clone;
use std::{cmp::Ordering, collections::LinkedList};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<K, V>
where
    K: Ord + Clone + PartialEq + PartialOrd,
    V: Clone, // Value can be any type
{
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
    size: i32, // Size is the weight of the node (length of the string it represents)
}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    pub fn is_red(&self) -> bool {
        self.color.is_red()
    }
    pub fn size(&self) -> i32 {
        self.size
    }
}

impl Color {
    fn is_red(&self) -> bool {
        *self == Color::Red
    }
    fn flip_color(&self) -> Color {
        if *self == Color::Red {
            Color::Black
        } else {
            Color::Red
        }
    }
}

pub struct RedBlackTree<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord + Clone, V: Clone> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn is_red(&self, node: &Option<Box<Node<K, V>>>) -> bool {
        match node {
            Some(n) => n.color == Color::Red,
            None => false,
        }
    }

    fn size(&self, node: &Option<Box<Node<K, V>>>) -> i32 {
        match node {
            Some(n) => n.size(),
            None => 0,
        }
    }

    pub fn tree_size(&self) -> i32 {
        self.size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_key(self.root.as_ref(), key)
    }

    fn get_key<'a>(&'a self, node: Option<&'a Box<Node<K, V>>>, key: &K) -> Option<&V> {
        if let Some(n) = node {
            match key.cmp(&n.key) {
                Ordering::Less => self.get_key(n.left.as_ref(), key),
                Ordering::Greater => self.get_key(n.right.as_ref(), key),
                Ordering::Equal => Some(&n.value),
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.root = self.insert_rec(&mut self.root, key, value);
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    fn insert_rec(
        &mut self,
        node: &mut Option<Box<Node<K, V>>>,
        key: K,
        value: V,
    ) -> Option<Box<Node<K, V>>> {
        if let Some(mut n) = node.take() {
            match key.cmp(&n.key) {
                Ordering::Less => n.left = self.insert_rec(&mut n.left, key, value),
                Ordering::Equal => n.value = value,
                Ordering::Greater => n.right = self.insert_rec(&mut n.right, key, value),
            }

            if self.is_red(&n.right) && !self.is_red(&n.left) {
                n = self.rotate_left(n);
            }
            if self.is_red(&n.left) && self.is_red(&n.left.as_ref()?.left) {
                n = self.rotate_right(n);
            }
            if self.is_red(&n.left) && self.is_red(&n.right) {
                self.flip_colors(&mut Some(n.clone()));
            }

            n.size = 1 + self.size(&n.left) + self.size(&n.right);
            return Some(n);
        } else {
            return Some(Box::new(Node {
                key,
                value,
                color: Color::Red,
                size: 1,
                left: None,
                right: None,
            }));
        }
    }

    fn rotate_left(&mut self, node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if let Some(mut x) = node {
            let mut y = x.right.take().unwrap();
            x.right = y.left.take();
            y.left = Some(x);
            y.left.as_mut().unwrap().size = 1 + self.size(&y.left) + self.size(&y.right);
            y.size = 1 + self.size(&y.left) + self.size(&y.right);
            Some(y)
        } else {
            None
        }
    }

    fn rotate_right(&mut self, node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if let Some(mut x) = node {
            let mut y = x.left.take().unwrap();
            x.left = y.right.take();
            y.right = Some(x);
            y.right.as_mut().unwrap().size = 1 + self.size(&y.left) + self.size(&y.right);
            y.size = 1 + self.size(&y.left) + self.size(&y.right);
            Some(y)
        } else {
            None
        }
    }

    fn flip_colors(&mut self, node: &mut Option<Box<Node<K, V>>>) {
        if let Some(n) = node {
            n.color = n.color.flip_color();
            if let Some(left) = n.left.as_mut() {
                left.color = left.color.flip_color();
            }
            if let Some(right) = n.right.as_mut() {
                right.color = right.color.flip_color();
            }
        }
    }

    // Rope characteristics: Concatenate two Ropes
    pub fn concatenate(&mut self, other: &RedBlackTree<K, V>) {
        let new_root = self.root.take();
        self.root = Some(Box::new(Node {
            key: self.min().unwrap(), // A temporary key, can be any representative key
            value: "".to_string(),    // A placeholder value
            left: new_root,
            right: other.root.clone(), // Keep the other tree's root
            color: Color::Black,       // New root must be black
            size: self.tree_size() + other.tree_size(), // Update size
        }));
    }

    // Substring retrieval
    pub fn substring(&self, start: i32, end: i32) -> Option<String> {
        if start < 0 || end > self.tree_size() || start > end {
            return None; // Invalid range
        }
        let mut result = String::new();
        self.collect_substring(&self.root, &mut result, start, end);
        Some(result)
    }

    fn collect_substring(
        &self,
        node: &Option<Box<Node<K, V>>>,
        result: &mut String,
        start: i32,
        end: i32,
    ) {
        if let Some(n) = node {
            if start < n.size {
                self.collect_substring(&n.left, result, start, end);
            }

            if start < n.size && end > 0 {
                let key_str = n.value.clone();
                let len = key_str.len() as i32;

                // Append valid range from the current node's string
                if start < len && end > 0 {
                    let start_idx = start.max(0);
                    let end_idx = end.min(len);
                    result.push_str(&key_str[start_idx as usize..end_idx as usize]);
                }
            }

            if end > n.size {
                self.collect_substring(&n.right, result, start - n.size, end - n.size);
            }
        }
    }

    // Other Red-Black Tree methods (deletion, min, max, etc.) would remain here...
}

fn main() {
    let mut rbt1 = RedBlackTree::new();
    rbt1.insert("Hello", "World");
    rbt1.insert("Goodbye", "World");

    let mut rbt2 = RedBlackTree::new();
    rbt2.insert("Good", "Morning");

    rbt1.concatenate(&rbt2);
    let substring = rbt1.substring(0, 5).unwrap();
    println!("Substring: {}", substring); // Output: Substring: Hello
}
