use indexmap::IndexSet;

pub struct Markers<'a> {
    pub middle: &'a str,
    pub indent: &'a str,
    pub end: &'a str,
    pub edge: &'a str,
    pub whitespace: &'a str,
}

impl<'a> Default for Markers<'a> {
    fn default() -> Self {
        Self {
            indent: "━",
            middle: "┣",
            end: "┗",
            edge: "┃",
            whitespace: "   ",
        }
    }
}

pub struct TreeFormatter<'a> {
    markers: Markers<'a>,
    nodes: IndexSet<(String, usize)>,
}

impl<'a> TreeFormatter<'a> {
    pub fn new(nodes: IndexSet<(String, usize)>) -> Self {
        Self {
            nodes,
            markers: Markers::default(),
        }
    }

    // TODO: this needs a refactor...
    // Biggest "issue" is that when the last leaf of a feature happened we do not cut its line off
    // This means that a line will run until it hits a leaf where the depth is smaller than its own
    // depth
    // termtree has an implementation for this but they have an eniterly different sturcture
    fn get_first_and_middle_char(
        &self,
        current_depth: usize,
        is_last_node: bool,
        peeked_next_node: Option<&(usize, &(String, usize))>,
    ) -> (&str, &str) {
        let first_char = if current_depth == 0 && !is_last_node {
            self.markers.middle
        } else if current_depth == 0 && is_last_node {
            self.markers.end
        } else {
            self.markers.edge
        };

        let middle_char = match peeked_next_node {
            Some((_, (_, next_depth))) => {
                if current_depth != 0 && next_depth >= &current_depth {
                    self.markers.middle
                } else if current_depth != 0 && next_depth < &current_depth {
                    self.markers.end
                } else {
                    self.markers.indent
                }
            }
            None => {
                if current_depth == 0 {
                    self.markers.indent
                } else {
                    self.markers.end
                }
            }
        };

        (first_char, middle_char)
    }

    pub fn write(&self) {
        let mut node_iter = self.nodes.iter().enumerate().peekable();
        while let Some((i, (name, current_depth))) = node_iter.next() {
            let (first_char, middle_char) = self.get_first_and_middle_char(
                *current_depth,
                i == self.nodes.len() - 1,
                node_iter.peek(),
            );

            // -- print part --

            let mut line = String::from(first_char);

            line.push_str(
                &self
                    .markers
                    .whitespace
                    .repeat(std::cmp::min(*current_depth, 1)),
            );

            for _ in 0..current_depth.checked_sub(1).unwrap_or(0) {
                line.push_str(self.markers.edge);
                line.push_str(self.markers.whitespace);
            }

            line.push_str(middle_char);
            line.push_str(self.markers.indent);
            line.push(' ');
            line.push_str(name);

            println!("{}", line);
        }
    }
}
