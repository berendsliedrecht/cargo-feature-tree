use colored::Colorize;
use indexmap::IndexSet;
use std::fmt;

pub struct Formatter<'a> {
    indent: &'a str,
    list_marker: &'a str,
    nodes: IndexSet<(&'a str, u8)>,
}

impl Default for Formatter<'_> {
    fn default() -> Self {
        Self {
            indent: "  ",
            list_marker: "-",
            nodes: IndexSet::new(),
        }
    }
}

impl fmt::Display for Formatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.nodes.iter().for_each(|(name, depth)| {
            if *name == "default" {
                let _ = writeln!(
                    f,
                    "{}{} {}",
                    self.indent.repeat((*depth).into()),
                    self.list_marker,
                    name.cyan()
                );
            } else {
                let _ = writeln!(
                    f,
                    "{}{} {}",
                    self.indent.repeat((*depth).into()),
                    self.list_marker,
                    name
                );
            }
        });
        Ok(())
    }
}

impl<'a> Formatter<'a> {
    pub fn new(nodes: IndexSet<(&'a str, u8)>) -> Self {
        Self {
            nodes,
            ..Default::default()
        }
    }

    pub fn write(&self) {
        println!("{self}");
    }
}
