pub mod RBtree;

struct TextEditor {
    tree: RBtree::RedBlackTree<usize, String>,
    cursor: usize,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            tree: RBtree::RedBlackTree::new(),
            cursor: 0,
        }
    }

    pub fn insert(&mut self, text: &str) {
        let index = self.cursor;
        self.tree.insert(&index, &text.to_string());
        self.cursor += 1;
    }

    pub fn delete(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.tree.delete(&self.cursor);
        }
    }

    pub fn move_cursor(&mut self, position: usize) {
        if position < self.cursor {
            self.cursor = position;
        }
    }

    pub fn get_text(&self) -> String {
        let mut text = String::new();
        for i in 0..self.cursor {
            if let Some(line) = self.tree.get(&i) {
                text.push_str(line);
                text.push('\n');
            }
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_editor_operations() {
        let mut editor = TextEditor::new();

        // Insert some lines of text
        editor.insert("Hello, world!");
        editor.insert("This is a test.");
        editor.insert("Rust is awesome.");

        // Check the current text
        let text = editor.get_text();
        assert_eq!(text, "Hello, world!\nThis is a test.\nRust is awesome.\n");

        // Delete the last line
        editor.delete();
        let text = editor.get_text();
        assert_eq!(text, "Hello, world!\nThis is a test.\n");

        // Move cursor and insert a new line
        editor.move_cursor(1);
        editor.insert("Inserted line.");
        let text = editor.get_text();
        assert_eq!(text, "Hello, world!\nInserted line.\nThis is a test.\n");
    }
}

fn main() {
    // Run the tests
    let mut editor = TextEditor::new();

    // Insert some lines of text
    editor.insert("Hello, world!");
    editor.insert("This is a test.");
    editor.insert("Rust is awesome.");

    // Check the current text
    let text = editor.get_text();
    println!("Current text:\n{}", text);

    // Delete the last line
    editor.delete();
    let text = editor.get_text();
    println!("After deletion:\n{}", text);

    // Move cursor and insert a new line
    editor.move_cursor(1);
    editor.insert("Inserted line.");
    let text = editor.get_text();
    println!("After insertion at cursor:\n{}", text);
}
