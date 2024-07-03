//NOTE: ---------------------- NODE IMPLEMENTATION -----------------------------

use ::std::clone::Clone;
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
    V: Ord + Clone,
{
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
    size: i32,
}

impl<K: Ord + Clone, V: Ord + Clone> Node<K, V> {
    pub fn is_red(&self) -> bool {
        self.color.is_red()
    }
    pub fn size(&self) -> i32 {
        self.size
    }
}
//
// impl<K: Copy, V: Copy> Copy for Node<K, V> {
//     fn
// }

impl<K: Clone + Ord, V: Clone + Ord> Clone for Node<K, V> {
    fn clone(&self) -> Node<K, V> {
        Node {
            key: self.key.clone(),
            value: self.value.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            color: self.color.clone(), // Assuming Color is Copy; adjust if needed
            size: self.size.clone(),
        }
    }
}

impl Color {
    fn is_red(&self) -> bool {
        self == &Color::Red
    }
    fn flip_color(&self) -> Color {
        if self == &Color::Red {
            Color::Black
        } else {
            Color::Red
        }
    }
}
pub struct RedBlackTree<K, V>
where
    K: Ord + Clone,
    V: Ord + Clone,
{
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord + Clone, V: Ord + Clone> RedBlackTree<K, V>
where
    K: Ord,
{
    //NOTE:***************************************************************************
    //   *  Node helper methods.
    //   ***************************************************************************
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
            Some(n) => n.size,
            None => 0,
        }
    }

    pub fn tree_size(&self) -> i32 {
        self.size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    //NOTE:***************************************************************************
    //   *  Standard BST SEARCH
    //   ***************************************************************************
    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_key(self.root.as_ref(), key)
    }

    fn get_key<'a>(&'a self, mut node: Option<&'a Box<Node<K, V>>>, key: &K) -> Option<&V> {
        while let Some(n) = node {
            match key.cmp(&n.key) {
                std::cmp::Ordering::Less => node = n.left.as_ref(), // Move to the left child
                std::cmp::Ordering::Greater => node = n.right.as_ref(), // Move to the right child
                std::cmp::Ordering::Equal => return Some(&n.value), // Key found, return its value
            }
        }
        None // Key not found
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    //NOTE:***************************************************************************
    //   *  Red Black Tree Insertion
    //   ***************************************************************************
    pub fn insert(&mut self, key: &K, value: &V) {
        if let Some(k) = Some(key) {
            if let Some(val) = Some(value) {
                self.root = self.insert_rec(&mut self.root.clone(), k, val);
                if let Some(ref mut root) = self.root {
                    root.color = Color::Black;
                }
            } else {
                self.delete(k);
                return;
            }
        }
    }

    fn insert_rec(
        &mut self,
        node: &mut Option<Box<Node<K, V>>>,
        key: &K,
        value: &V,
    ) -> Option<Box<Node<K, V>>> {
        if let Some(mut n) = node.clone() {
            match key.cmp(&n.key) {
                Ordering::Less => n.left = self.insert_rec(&mut n.left, key, value),
                Ordering::Equal => n.value = value.clone(),
                Ordering::Greater => n.right = self.insert_rec(&mut n.right, key, value),
            }

            if self.is_red(&n.right) && !self.is_red(&n.left) {
                n = self.rotate_left(&mut Some(n.clone())).unwrap()
            }
            if self.is_red(&n.left) && self.is_red(&n.left.as_ref()?.left) {
                n = self.rotate_right(&mut Some(n.clone())).unwrap();
            }
            if self.is_red(&n.right) && self.is_red(&n.left) {
                self.flip_colors(&mut Some(n.clone()));
            }

            n.size = self.size(&n.left) + self.size(&n.right) + 1;
            return Some(n.clone());
        } else {
            return Some(Box::new(Node {
                key: key.clone(),
                value: value.clone(),
                color: Color::Red,
                size: 1,
                left: None,
                right: None,
            }));
        }
    }
    //NOTE:***************************************************************************
    //   *  Red Black Tree Deltetion
    //   ***************************************************************************
    pub fn delete(&mut self, key: &K) {
        if !self.contains(key) {
            return;
        }

        if self.root.as_ref().map_or(false, |n| !n.is_red()) {
            if let Some(ref mut root) = self.root {
                root.color = Color::Red;
            }
        }

        let mut temp_root = self.root.take();
        self.root = self.delete_node(&mut temp_root, key);

        if self.root.is_some() && !self.root.as_ref().unwrap().is_red() {
            if let Some(ref mut root) = self.root {
                root.color = Color::Black;
            }
        }
    }

    fn delete_node(
        &mut self,
        node: &mut Option<Box<Node<K, V>>>,
        key: &K,
    ) -> Option<Box<Node<K, V>>> {
        if let Some(ref mut _node) = node {
            if key < &_node.key {
                if _node.left.is_some()
                    && !_node.left.as_ref().unwrap().is_red()
                    && _node
                        .left
                        .as_ref()
                        .unwrap()
                        .left
                        .as_ref()
                        .map_or(true, |n| !n.is_red())
                {
                    _node.left = self.move_red_left(&mut _node.left.take());
                }
                _node.left = self.delete_node(&mut _node.left, key);
            } else {
                if _node.right.is_some()
                    && !_node.right.as_ref().unwrap().is_red()
                    && _node
                        .right
                        .as_ref()
                        .unwrap()
                        .left
                        .as_ref()
                        .map_or(true, |n| !n.is_red())
                {
                    _node.right = self.move_red_right(&mut _node.right.take());
                }
                if key == &_node.key && _node.right.is_none() {
                    return None; // Node to be deleted found and it has no right child.
                }
                if key == &_node.key {
                    let min = self.min_key(&_node.right).unwrap();
                    _node.key = min.key;
                    _node.value = min.value;
                    _node.right = self.delete_min_node(&mut _node.right.take());
                } else {
                    _node.right = self.delete_node(&mut _node.right, key);
                }
            }
            self.balance(node)
        } else {
            None
        }
    }

    pub fn delete_min(&mut self) {
        if !self.is_empty() {
            if !self.is_red(&self.root.as_ref().unwrap().left)
                && !self.is_red(&self.root.as_ref().unwrap().right)
            {
                if let Some(ref mut root) = self.root {
                    root.color = Color::Red;
                }
            }
            let mut temp_root = self.root.take();
            self.root = self.delete_min_node(&mut temp_root);

            if !self.is_empty() {
                if let Some(ref mut root) = self.root {
                    root.color = Color::Black;
                }
            }
        }
    }

    fn delete_min_node(&mut self, node: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        let mut n = node.take().unwrap();

        if !self.is_red(&n.left) && !self.is_red(&n.left.as_ref()?.left) {
            n = self.move_red_left(&mut Some(n))?;
        }

        n.left = self.delete_min_node(&mut n.left);

        self.balance(&mut Some(n))
    }

    //NOTE:***************************************************************************
    //   *  Standard BST HELPER FUNCTIONS
    //   ***************************************************************************
    fn rotate_right(&mut self, node: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if let Some(mut x) = node.take() {
            let mut y = x.left.take().unwrap();
            x.left = y.right.take();
            y.right = Some(x);
            Some(y)
        } else {
            None
        }
    }

    fn rotate_left(&mut self, node: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if let Some(mut x) = node.take() {
            let mut y = x.right.take().unwrap();
            x.right = y.left.take();
            y.left = Some(x);
            Some(y)
        } else {
            None
        }
    }

    fn move_red_left(&mut self, node: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        self.flip_colors(node);
        if let Some(ref mut n) = node {
            if self.is_red(&n.right.as_ref()?.left) {
                n.right = self.rotate_right(&mut n.right);
                *node = self.rotate_left(&mut node.take());
                self.flip_colors(node);
            }
        }
        node.take()
    }
    fn flip_colors(&mut self, node: &mut Option<Box<Node<K, V>>>) {
        node.as_mut().unwrap().color.flip_color();
        node.as_mut()
            .unwrap()
            .left
            .as_mut()
            .unwrap()
            .color
            .flip_color();
        node.as_mut()
            .unwrap()
            .right
            .as_mut()
            .unwrap()
            .color
            .flip_color();
    }

    fn move_red_right(&mut self, node: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        self.flip_colors(node);
        if let Some(ref mut n) = node {
            if self.is_red(&n.left.as_ref()?.left) {
                *node = self.rotate_right(&mut node.take());
                self.flip_colors(node);
            }
        }
        node.take()
    }

    fn balance(&mut self, node: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if let Some(_node) = node {
            if self.is_red(&_node.as_ref().right) && !self.is_red(&_node.as_ref().left) {
                *node = self.rotate_left(&mut node.take());
            }
        }

        if let Some(_node) = node {
            if self.is_red(&_node.as_ref().right)
                && self.is_red(&_node.as_ref().left.as_ref()?.left)
            {
                *node = self.rotate_right(&mut node.take());
            }
        }

        if self.is_red(&node.as_ref()?.left) && self.is_red(&node.as_ref()?.right) {
            self.flip_colors(node);
        }

        node.as_mut()?.size =
            self.size(&node.as_ref()?.left) + self.size(&node.as_ref()?.right) + 1;

        node.take()
    }

    //NOTE:***************************************************************************
    //   *  Ordered symbol table methods.
    //   ***************************************************************************/
    pub fn min(&self) -> K {
        if self.is_empty() {
            panic!("Empty Tree");
        }
        self.min_key(&self.root).unwrap().key
    }

    fn min_key(&self, node: &Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        let n = node.clone().unwrap();

        if n.left.is_none() {
            Some(n)
        } else {
            self.min_key(&n.left)
        }
    }

    pub fn max(&self) -> K {
        if self.is_empty() {
            panic!("Empty tree")
        }

        self.max_key(&self.root).unwrap().key
    }

    fn max_key(&self, node: &Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        let n = node.clone().unwrap();
        if n.right.is_none() {
            Some(n)
        } else {
            self.max_key(&n.right)
        }
    }

    pub fn floor(&self, key: &K) -> K {
        // if self.is_empty() {
        //
        // }

        self.floor_node(&self.root, key).unwrap().key
    }

    fn floor_node(&self, node: &Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>> {
        match node.as_ref()?.key.cmp(key) {
            Ordering::Less => self.floor_node(&node.as_ref()?.left, key),
            Ordering::Equal => node.clone(),
            Ordering::Greater => self.floor_node(&node.as_ref()?.right, key),
        }
    }

    pub fn Ceiling(&self, key: &K) -> K {
        //if self.is_empty() {
        //
        //}
        self.ceiling_node(&self.root, key).unwrap().key
    }

    fn ceiling_node(&self, node: &Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>> {
        match node.as_ref()?.key.cmp(key) {
            Ordering::Less => self.ceiling_node(&node.as_ref()?.left, key),
            Ordering::Equal => node.clone(),
            Ordering::Greater => self.ceiling_node(&node.as_ref()?.right, key),
        }
    }

    pub fn select(&self, rank: i32) -> K {
        if (rank < 0 || rank >= self.tree_size()) {
            panic!("Select function ERROR: rank < 0 or rank > tree size")
        }

        self.select_node(rank, &self.root).unwrap()
    }

    fn select_node(&self, rank: i32, node: &Option<Box<Node<K, V>>>) -> Option<K> {
        match node.as_ref()?.left.as_ref()?.size.cmp(&rank) {
            Ordering::Less => self.select_node(
                rank - node.as_ref()?.left.as_ref()?.size - 1,
                &node.as_ref()?.right,
            ),
            Ordering::Equal => Some(node.clone().unwrap().key),
            Ordering::Greater => self.select_node(rank, &node.as_ref()?.left),
        }
    }

    pub fn rank(&self, key: &K) -> i32 {
        self.rank_node(key, &self.root).unwrap()
    }

    fn rank_node(&self, key: &K, node: &Option<Box<Node<K, V>>>) -> Option<i32> {
        match key.cmp(&node.as_ref()?.key) {
            Ordering::Less => self.rank_node(key, &node.as_ref()?.left),
            Ordering::Equal => Some(self.size(&node.as_ref()?.left)),
            Ordering::Greater => Some(
                1 + self.size(&node.as_ref()?.left)
                    + self.rank_node(key, &node.as_ref()?.right).unwrap(),
            ),
        }
    }

    //NOTE:***************************************************************************
    //   *  Range Count and Range SEARCH
    //   ***************************************************************************/
    //
    //

    //FIXME: CONVERT TO A VEC
    pub fn keys(&self) -> impl Iterator<Item = K> {
        self.keys_range(self.min(), self.max())
    }

    pub fn keys_range(&self, low: K, high: K) -> impl Iterator<Item = K> {
        let mut q: LinkedList<K> = LinkedList::new();
        self.keys_queue(&self.root, &mut q, low, high);
        q.into_iter()
    }

    fn keys_queue(&self, node: &Option<Box<Node<K, V>>>, q: &mut LinkedList<K>, low: K, high: K) {
        if let Some(ref n) = node {
            match low.cmp(&n.key) {
                Ordering::Less => self.keys_queue(&n.left, q, low.clone(), high.clone()),
                _ => (),
            }

            if low <= n.key && high >= n.key {
                q.push_back(n.key.clone());
            }

            match high.cmp(&n.key) {
                Ordering::Greater => self.keys_queue(&n.right, q, low, high),
                _ => (),
            }
        }
    }
    pub fn size_size(&self, lo: &K, hi: &K) -> i32 {
        if lo > hi {
            return 0;
        }

        if self.contains(hi) {
            self.rank(hi) - self.rank(lo) + 1
        } else {
            self.rank(hi) - self.rank(lo)
        }
    }

    //NOTE:***************************************************************************
    //   *  CHECK INTEGRITY OF RED-BLACK TREE DATA STRUCTURE
    //   ***************************************************************************/
    // fn check(&self) -> bool {
    //     if()
    //
    // }
    fn is_bst(&self) -> bool {
        return self.is_bst_rec(&self.root, None, None);
    }

    fn is_bst_rec(&self, node: &Option<Box<Node<K, V>>>, min: Option<&K>, max: Option<&K>) -> bool {
        if let Some(n) = node {
            if let Some(min_key) = min {
                if n.key <= *min_key {
                    return false;
                }
            }
            if let Some(max_key) = max {
                if n.key >= *max_key {
                    return false;
                }
            }

            return self.is_bst_rec(&n.left, min, Some(&n.key))
                && self.is_bst_rec(&n.right, Some(&n.key), max);
        }
        return true;
    }

    fn is_size_consistent(&self) -> bool {
        return self.is_size_consistent_rec(&self.root);
    }

    fn is_size_consistent_rec(&self, node: &Option<Box<Node<K, V>>>) -> bool {
        if let Some(n) = node {
            if n.size() != self.size(&n.left) + self.size(&n.right) + 1 {
                return false;
            }
            return self.is_size_consistent_rec(&n.left) && self.is_size_consistent_rec(&n.right);
        }

        return true;
    }

    fn is_rank_consistent(&self) -> bool {
        for i in 0..self.tree_size() {
            if i != self.rank(&self.select(i)) {
                return false;
            }
        }
        for key in self.keys() {
            if key != self.select(self.rank(&key)) {
                return false;
            }
        }
        return true;
    }
    fn is23(&self) -> bool {
        return self.is23_rec(&self.root).unwrap();
    }
    fn is23_rec(&self, node: &Option<Box<Node<K, V>>>) -> Option<bool> {
        if let Some(n) = node {
            if self.is_red(&n.right) {
                return Some(false);
            }
            if n != &self.root.clone().unwrap() && self.is_red(node) && self.is_red(&n.left) {
                return Some(false);
            }
        }
        return Some(
            self.is23_rec(&node.as_ref()?.left).unwrap()
                && self.is23_rec(&node.as_ref()?.right).unwrap(),
        );
    }

    fn is_balanced(&self) -> bool {
        let mut black = 0;

        let mut node = self.root.clone();

        while node.is_some() {
            if !self.is_red(&node) {
                black = black + 1;
            }
            node = node.unwrap().left
        }
        return self.is_balanced_rec(&self.root, black);
    }

    fn is_balanced_rec(&self, node: &Option<Box<Node<K, V>>>, mut black: i32) -> bool {
        if let Some(n) = node {
            if !self.is_red(node) {
                black = black - 1;
            }
            return self.is_balanced_rec(&n.left, black) && self.is_balanced_rec(&n.right, black);
        }
        return black == 0;
    }
}
