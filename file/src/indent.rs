use crate::tool::Constructor;

pub struct Indent {
    pub lines: Vec<(usize, String)>,
}

impl Indent {
    pub fn new(lines: Vec<(usize, String)>) -> Self {
        Self { lines }
    }

    /// Returns the modified line with new space indentation
    pub fn indent_lines(&self, _range : Option<(usize, usize)>, tab_size: usize) -> Vec<(usize, String)> {
        let lines = self.lines.clone();
        let indexes: Vec<usize>;
        if let Some((start, end)) = _range {
            indexes = (start..=end).collect();
        } else {
            indexes = Vec::new();
        }
        let mut new_lines: Vec<(usize, String)> = Vec::new();
        for (index, line) in lines.iter() {
            if !indexes.iter().any(|i| i == index) {
                new_lines.push((*index, line.to_owned()));
                continue;
            }
            let indent = " ".repeat(tab_size);
            new_lines.push((*index, format!("{}{}", indent, line.to_owned())));
        }
        new_lines
    }
}

impl Constructor for Indent {}
