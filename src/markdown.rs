use std::fs::read_to_string;
use std::path::Path;

use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::plugins::cmark::inline::link::Link;
use markdown_it::{MarkdownIt, Node};
use markdown_it_front_matter::FrontMatter as FrontMatterNode;

use crate::text_normalization::anchorize;

pub const INTERLINK_MAGIC_HREF: &str = "!!";

pub struct AST(Node);

impl AST {
    pub fn read_from_file(path: &Path) -> Result<Self, std::io::Error> {
        Parser::new()
            .read_ast_from_file(path)
            .map(|node| Self(node))
    }

    pub fn extract_frontmatter_text(self: &Self) -> Option<String> {
        self.0
            .children
            .iter()
            .find(|c| c.is::<FrontMatterNode>())
            .map(|node| node.cast::<FrontMatterNode>().map(|fm| fm.content.clone()))
            .flatten()
    }

    pub fn extract_header(self: &Self) -> Option<String> {
        self.0
            .children
            .iter()
            .find(|c| c.is::<ATXHeading>())
            .map(|node| node.collect_text())
    }

    pub fn render(self: &Self) -> String {
        self.0.render()
    }
}

pub struct Parser(MarkdownIt);

impl Parser {
    pub fn new() -> Self {
        let mut parser = MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut parser);
        markdown_it_front_matter::add(&mut parser);
        Self(parser)
    }

    pub fn parse(self: &Self, src: &str) -> Node {
        let mut parsed = self.0.parse(src);

        parsed.walk_mut(|node, _| {
            // Don't waste allocations on things that aren't links anyway.
            if !node.is::<Link>() {
                return;
            }

            // If we are a link, we have to pre-calculate the child text for anchorization since
            // we're about to take a mutable reference to the node, and collect_text() takes out an
            // immutable reference and these can't coexist in Rust.
            let anchorized = anchorize(&node.collect_text());

            // Now, sub out the link destination with the correct interlink anchor.
            if let Some(link) = node.cast_mut::<Link>() {
                // Only swap out the magic string. Any other links must be left intact.
                if link.url != INTERLINK_MAGIC_HREF {
                    return;
                }

                link.url = format!("#{}", anchorized);
            }
        });

        parsed
    }

    pub fn read_ast_from_file(self: &Self, path: &Path) -> Result<Node, std::io::Error> {
        read_to_string(path).map(|data| self.parse(&data))
    }
}

#[test]
fn test_markdown_interlink() {
    // bang-bang is the new interlink syntax, it should be transformed just like how `|this
    // syntax|` used to be
    assert_eq!(
        Parser::new().parse(r"Hello [world](!!)").render().trim_end(),
        "<p>Hello <a href=\"#world\">world</a></p>"
    );

    // but any other number of bangs are left untouched
    assert_eq!(
        Parser::new().parse(r"Hello [world](!!!)").render().trim_end(),
        "<p>Hello <a href=\"!!!\">world</a></p>"
    );
}
