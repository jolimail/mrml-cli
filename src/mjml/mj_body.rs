use super::prelude::*;
use super::{Component, Element, Error};
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Style};
use crate::{close_tag, open_tag};
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

const ALLOWED_ATTRIBUTES: [&'static str; 3] = ["background-color", "css-class", "width"];

#[derive(Clone, Debug)]
pub struct MJBody<'a, 'b> {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<Element<'a, 'b>>,
    exists: bool,
}

impl MJBody<'_, '_> {
    pub fn empty<'a, 'b>() -> MJBody<'a, 'b> {
        MJBody {
            attributes: HashMap::new(),
            children: vec![],
            context: None,
            exists: false,
        }
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJBody<'a, 'b>, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(Element::parse(child)?);
        }
        Ok(MJBody {
            attributes: get_node_attributes(&node),
            children,
            context: None,
            exists: true,
        })
    }
}

impl Component for MJBody<'_, '_> {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }

    fn allowed_attributes(&self) -> Option<Vec<&'static str>> {
        Some(ALLOWED_ATTRIBUTES.to_vec())
    }

    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "width" => Some("600px".into()),
            _ => None,
        }
    }

    fn to_header(&self) -> Header {
        let mut header = Header::new();
        for child in self.children.iter() {
            header.merge(&child.to_header());
        }
        header
    }

    fn get_style(&self, key: &str) -> Style {
        let mut res = Style::new();
        match key {
            "body" => {
                res.maybe_set("background-color", self.get_attribute("background-color"));
            }
            "div" => {
                res.maybe_set("background-color", self.get_attribute("background-color"));
            }
            _ => (),
        };
        res
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
        //
        let sibling = self.children.len();
        let raw_sibling = self
            .children
            .iter()
            .filter(|item| match item {
                Element::Raw(_) => true,
                _ => false,
            })
            .collect::<Vec<&Element>>()
            .len();
        //
        let container_width = self.get_size_attribute("width");
        //
        for (idx, child) in self.children.iter_mut().enumerate() {
            child.set_context(Context::from(
                &ctx,
                container_width.clone(),
                sibling,
                raw_sibling,
                idx,
            ));
        }
    }

    fn render(&self) -> Result<String, Error> {
        debug!("render");
        let mut res: Vec<String> = vec![];
        {
            let mut attrs = Attributes::new();
            let style = self.get_style("body");
            if !style.is_empty() {
                attrs.set("style", style.to_string());
            }
            res.push(open_tag!("body", attrs.to_string()));
        }
        if self.exists {
            let mut attrs = Attributes::new();
            attrs.maybe_set("class", self.get_attribute("css-class"));
            attrs.set("style", self.get_style("div").to_string());
            res.push(open_tag!("div", attrs.to_string()));
            for child in self.children.iter() {
                res.push(child.render()?);
            }
            res.push(close_tag!("div"));
        }
        res.push(close_tag!("body"));
        Ok(res.join(""))
    }
}

impl ComponentWithSizeAttribute for MJBody<'_, '_> {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn basic() {
        compare_render(
            include_str!("../../test/mj-body.mjml"),
            include_str!("../../test/mj-body.html"),
        );
    }

    #[test]
    fn with_background_color() {
        compare_render(
            include_str!("../../test/mj-body-background-color.mjml"),
            include_str!("../../test/mj-body-background-color.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../test/mj-body-class.mjml"),
            include_str!("../../test/mj-body-class.html"),
        );
    }

    #[test]
    fn with_width() {
        compare_render(
            include_str!("../../test/mj-body-width.mjml"),
            include_str!("../../test/mj-body-width.html"),
        );
    }
}
