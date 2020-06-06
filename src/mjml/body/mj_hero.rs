use super::BodyElement;
use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::prelude::*;
use crate::util::{Context, Header, Size, Style, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJHero {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJHero {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJHero, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(BodyElement::parse(child, opts, None)?);
        }
        Ok(MJHero {
            attributes: get_node_attributes(&node),
            context: None,
            children,
        })
    }

    fn get_style_div(&self) -> Style {
        let mut res = Style::new();
        res.set("margin", "0 auto");
        res.maybe_set("max-width", self.get_container_width_str());
        res
    }

    fn get_style_table(&self) -> Style {
        let mut res = Style::new();
        res.set("width", "100%");
        res
    }

    fn get_style_tr(&self) -> Style {
        let mut res = Style::new();
        res.set("vertical-align", "top");
        res
    }

    fn get_style_td_fluid(&self) -> Style {
        let bg_ratio = self
            .get_size_attribute("background-height")
            .and_then(|height| {
                self.get_size_attribute("background-width")
                    .and_then(|width| Some(height.value() * 100.0 / width.value()))
            });
        let mut res = Style::new();
        res.set("mso-padding-bottom-alt", "0");
        res.maybe_set("padding-bottom", bg_ratio);
        res.set("width", "0.01%");
        res
    }

    fn get_style_outlook_table(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("width", self.get_container_width_str());
        res
    }

    fn get_style_outlook_inner_table(&self) -> Style {
        self.get_style_outlook_table()
    }

    fn get_style_outlook_inner_td(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set(
            "background-color",
            self.get_attribute("inner-background-color"),
        );
        res.maybe_set("padding", self.get_attribute("inner-padding"));
        res.maybe_set("padding-top", self.get_attribute("inner-padding-top"));
        res.maybe_set("padding-right", self.get_attribute("inner-padding-right"));
        res.maybe_set("padding-bottom", self.get_attribute("inner-padding-bottom"));
        res.maybe_set("padding-left", self.get_attribute("inner-padding-left"));
        res
    }

    fn get_style_inner_div(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set(
            "background-color",
            self.get_attribute("inner-background-color"),
        );
        res.maybe_set("float", self.get_attribute("align"));
        res.set("margin", "0px auto");
        res.maybe_set("width", self.get_attribute("width"));
        res
    }

    fn get_style_inner_table(&self) -> Style {
        let mut res = Style::new();
        res.set("width", "100%");
        res.set("margin", "0px");
        res
    }

    fn get_style_outlook_image(&self) -> Style {
        let mut res = Style::new();
        res.set("border", "0");
        res.maybe_set("height", self.get_attribute("background-height"));
        res.set("mso-position-horizontal", "center");
        res.set("position", "absolute");
        res.set("top", "0");
        res.maybe_set(
            "width",
            self.get_attribute("background-width")
                .or_else(|| self.get_container_width_str()),
        );
        res.set("z-index", "-3");
        res
    }

    fn get_style_outlook_td(&self) -> Style {
        let mut res = Style::new();
        res.set("line-height", "0");
        res.set("font-size", "0");
        res.set("mso-line-height-rule", "exactly");
        res
    }

    fn get_background(&self) -> Option<String> {
        let bg_color = self.get_attribute("background-color");
        match self.get_attribute("background-url") {
            Some(url) => Some(format!(
                "{} url({}) no-repeat {} / cover",
                // has default value
                self.get_attribute("background-color").unwrap(),
                url,
                // has default value
                self.get_attribute("background-position").unwrap()
            )),
            None => bg_color,
        }
    }

    fn get_style_hero(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("background", self.get_background());
        res.maybe_set(
            "background-position",
            self.get_attribute("background-position"),
        );
        res.set("background-repeat", "no-repeat");
        res.maybe_set("padding", self.get_attribute("padding"));
        res.maybe_set("padding-top", self.get_attribute("padding-top"));
        res.maybe_set("padding-right", self.get_attribute("padding-right"));
        res.maybe_set("padding-bottom", self.get_attribute("padding-bottom"));
        res.maybe_set("padding-left", self.get_attribute("padding-left"));
        res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
        res
    }

    fn render_child(&self, header: &Header, child: &BodyElement) -> Result<String, Error> {
        let tr = Tag::new("tr");
        let td = Tag::new("td")
            .maybe_set_style(
                "background",
                child.get_attribute("container-background-color"),
            )
            .set_style("font-size", "0px")
            .maybe_set_style("padding", child.get_attribute("padding"))
            .maybe_set_style("padding-top", child.get_attribute("padding-top"))
            .maybe_set_style("padding-right", child.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", child.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", child.get_attribute("padding-left"))
            .set_style("word-break", "break-word")
            .maybe_set_attribute("align", child.get_attribute("align"))
            .maybe_set_attribute(
                "background",
                child.get_attribute("container-background-color"),
            )
            .maybe_set_attribute("class", child.get_attribute("css-class"));
        Ok(tr.render(td.render(child.render(header)?)))
    }

    fn render_children(&self, header: &Header) -> Result<String, Error> {
        let mut res = String::from("");
        for child in self.get_children().iter() {
            let result = match child {
                BodyElement::Raw(_) => child.render(header)?,
                _ => self.render_child(header, child)?,
            };
            res.push_str(result.as_str());
        }
        Ok(res)
    }

    fn render_content(&self, header: &Header) -> Result<String, Error> {
        let table = Tag::new("table")
            .maybe_set_attribute("align", self.get_attribute("align"))
            .set_attribute("border", 0)
            .set_attribute("cellpadding", 0)
            .set_attribute("cellspacing", 0)
            .insert_style(self.get_style_outlook_inner_table().inner())
            .maybe_set_attribute("width", self.get_container_width_value());
        let tr = Tag::tr();
        let outlook_inner_td =
            Tag::new("td").insert_style(self.get_style_outlook_inner_td().inner());
        let outlook_inner_div = Tag::new("div")
            .maybe_set_attribute("width", self.get_attribute("align"))
            .set_class("mj-hero-content")
            .insert_style(self.get_style_inner_div().inner());
        let inner_table = Tag::table().insert_style(self.get_style_inner_table().inner());
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(outlook_inner_td.open());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(outlook_inner_div.render(inner_table.render(
            tr.render(Tag::td().render(inner_table.render(self.render_children(header)?))),
        )));
        res.push(START_CONDITIONAL_TAG.into());
        res.push(outlook_inner_td.close());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_mode_fluid(&self, header: &Header) -> Result<String, Error> {
        let td_fluid = Tag::td().insert_style(self.get_style_td_fluid().inner());
        let td = Tag::td()
            .maybe_set_attribute("background", self.get_attribute("background-url"))
            .insert_style(self.get_style_hero().inner());
        let mut res = vec![];
        res.push(td_fluid.closed());
        res.push(td.render(self.render_content(header)?));
        res.push(td_fluid.closed());
        Ok(res.join(""))
    }

    fn render_mode_fixed(&self, header: &Header) -> Result<String, Error> {
        // has a default value
        let height = self.get_size_attribute("height").unwrap();
        let padding = match self.get_padding_vertical() {
            Some(value) => value.value(),
            None => 0.0,
        };
        let height = Size::Pixel(height.value() - padding);
        let td = Tag::td()
            .maybe_set_attribute("background", self.get_attribute("background-url"))
            .insert_style(self.get_style_hero().inner())
            .set_attribute("height", height.value());
        Ok(td.render(self.render_content(header)?))
    }

    fn render_mode(&self, header: &Header) -> Result<String, Error> {
        match self.get_attribute("mode") {
            Some(mode) => match mode.as_str() {
                "fluid" => self.render_mode_fluid(header),
                _ => self.render_mode_fixed(header),
            },
            None => self.render_mode_fixed(header),
        }
    }
}

impl Component for MJHero {
    fn update_header(&self, header: &mut Header) {
        for child in self.children.iter() {
            child.update_header(header);
        }
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
        let sibling = self.get_siblings();
        let raw_sibling = self.get_raw_siblings();
        let container_width = self.get_container_width();
        for (idx, child) in self.children.iter_mut().enumerate() {
            let mut child_ctx =
                Context::from(&ctx, container_width.clone(), sibling, raw_sibling, idx);
            child_ctx.set("index", idx);
            child.set_context(child_ctx);
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let outlook_table = Tag::table()
            .set_attribute("align", "center")
            .insert_style(self.get_style_outlook_table().inner())
            .maybe_set_attribute("width", self.get_container_width_value());
        let outlook_tr = Tag::tr();
        let outlook_td = Tag::td().insert_style(self.get_style_outlook_td().inner());
        let v_image = Tag::new("v:image")
            .insert_style(self.get_style_outlook_image().inner())
            .maybe_set_attribute("src", self.get_attribute("background-url"))
            .set_attribute("xmlns:v", "urn:schemas-microsoft-com:vml");
        let div = Tag::div()
            .maybe_set_attribute("align", self.get_attribute("align"))
            .maybe_set_class(self.get_attribute("css-class"))
            .insert_style(self.get_style_div().inner());
        let table = Tag::table().insert_style(self.get_style_table().inner());
        let tr = Tag::tr().insert_style(self.get_style_tr().inner());
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(outlook_table.open());
        res.push(outlook_tr.open());
        res.push(outlook_td.open());
        res.push(v_image.closed());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(div.open());
        res.push(table.open());
        res.push(tr.open());
        res.push(self.render_mode(header)?);
        res.push(tr.close());
        res.push(table.close());
        res.push(div.close());
        res.push(START_CONDITIONAL_TAG.into());
        res.push(outlook_td.close());
        res.push(outlook_tr.close());
        res.push(outlook_table.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJHero {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "background-color" => Some("#ffffff".into()),
            "background-position" => Some("center center".into()),
            "height" => Some("0px".into()),
            "mode" => Some("fixed-height".into()),
            "padding" => Some("0px".into()),
            "vertical-align" => Some("top".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJHero {
    fn get_style(&self, name: &str) -> Style {
        match name {
            "div" => self.get_style_div(),
            "table" => self.get_style_table(),
            "tr" => self.get_style_tr(),
            "td-fluid" => self.get_style_td_fluid(),
            "hero" => self.get_style_hero(),
            "outlook-table" => self.get_style_outlook_table(),
            "outlook-td" => self.get_style_outlook_td(),
            "outlook-image" => self.get_style_outlook_image(),
            "outlook-inner-table" => self.get_style_outlook_inner_table(),
            // "outlook-inner-td" => self.get_style_outlook_inner_td(),
            "inner-div" => self.get_style_inner_div(),
            "inner-table" => self.get_style_inner_table(),
            _ => Style::new(),
        }
    }
}

impl ComponentWithChildren for MJHero {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }
}

impl BodyContainedComponent for MJHero {}
impl ComponentWithSizeAttribute for MJHero {}
impl BodyComponentWithPadding for MJHero {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-hero.mjml"),
            include_str!("../../../test/mj-hero.html"),
        );
    }

    #[test]
    fn with_background_color() {
        compare_render(
            include_str!("../../../test/mj-hero-background-color.mjml"),
            include_str!("../../../test/mj-hero-background-color.html"),
        );
    }

    #[test]
    fn with_background_height() {
        compare_render(
            include_str!("../../../test/mj-hero-background-height.mjml"),
            include_str!("../../../test/mj-hero-background-height.html"),
        );
    }

    #[test]
    fn with_background_position() {
        compare_render(
            include_str!("../../../test/mj-hero-background-position.mjml"),
            include_str!("../../../test/mj-hero-background-position.html"),
        );
    }

    #[test]
    fn with_background_url() {
        compare_render(
            include_str!("../../../test/mj-hero-background-url.mjml"),
            include_str!("../../../test/mj-hero-background-url.html"),
        );
    }

    #[test]
    fn with_background_width() {
        compare_render(
            include_str!("../../../test/mj-hero-background-width.mjml"),
            include_str!("../../../test/mj-hero-background-width.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../../test/mj-hero-class.mjml"),
            include_str!("../../../test/mj-hero-class.html"),
        );
    }

    #[test]
    fn with_height() {
        compare_render(
            include_str!("../../../test/mj-hero-height.mjml"),
            include_str!("../../../test/mj-hero-height.html"),
        );
    }

    #[test]
    fn with_mode() {
        compare_render(
            include_str!("../../../test/mj-hero-mode.mjml"),
            include_str!("../../../test/mj-hero-mode.html"),
        );
    }

    #[test]
    fn with_vertical_align() {
        compare_render(
            include_str!("../../../test/mj-hero-vertical-align.mjml"),
            include_str!("../../../test/mj-hero-vertical-align.html"),
        );
    }

    #[test]
    fn with_width() {
        compare_render(
            include_str!("../../../test/mj-hero-width.mjml"),
            include_str!("../../../test/mj-hero-width.html"),
        );
    }
}
