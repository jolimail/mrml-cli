use super::MJHead;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJHead {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        print::open(super::NAME, None, false, pretty, level, indent_size)
            + &self
                .children
                .iter()
                .map(|child| child.print(pretty, level + 1, indent_size))
                .collect::<String>()
            + &print::close(super::NAME, pretty, level, indent_size)
    }
}

impl fmt::Display for MJHead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_head::MJHead::default();
        assert_eq!("<mj-head></mj-head>", item.dense_print());
    }

    #[test]
    fn with_title() {
        let mut item = crate::mj_head::MJHead::default();
        item.children.push(crate::mj_head::MJHeadChild::MJTitle(
            crate::mj_title::MJTitle::from("Hello World!"),
        ));
        assert_eq!(
            "<mj-head><mj-title>Hello World!</mj-title></mj-head>",
            item.dense_print()
        );
    }
}
