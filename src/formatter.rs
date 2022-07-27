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

pub struct Formatter<'a> {
    title: &'a str,
    markers: Markers<'a>,
    nodes: IndexSet<(&'a str, usize)>,
}

impl Default for Formatter<'_> {
    fn default() -> Self {
        Self {
            nodes: IndexSet::new(),
            ..Default::default()
        }
    }
}

impl<'a> Formatter<'a> {
    pub fn new(nodes: IndexSet<(&'a str, usize)>, title: &'a str) -> Self {
        Self {
            title,
            nodes,
            markers: Markers::default(),
        }
    }

    // TODO: this needs a refactor...
    // Biggest "issue" is that when the last leaf of a feature happened we do not cut its line off
    // This means that a line will run until it hits a leaf where the depth is smaller than its own
    // depth
    pub fn write(&self) {
        let mut node_iter = self.nodes.iter().enumerate().peekable();
        println!("{}", self.title);
        while let Some((i, (name, current_depth))) = node_iter.next() {
            let first_char = if *current_depth == 0 && i != self.nodes.len() - 1 {
                self.markers.middle
            } else if *current_depth == 0 && i == self.nodes.len() - 1 {
                self.markers.end
            } else {
                self.markers.edge
            };

            let middle_char = match node_iter.peek() {
                Some((_, (_, next_depth))) => {
                    if *current_depth != 0 && next_depth >= current_depth {
                        self.markers.middle
                    } else if *current_depth != 0 && next_depth < current_depth {
                        self.markers.end
                    } else {
                        self.markers.indent
                    }
                }
                None => {
                    if *current_depth == 0 {
                        self.markers.indent
                    } else {
                        self.markers.end
                    }
                }
            };

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
