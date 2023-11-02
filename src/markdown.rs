use std::fs::read_to_string;
use std::path::Path;

use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::{MarkdownIt, Node};
use markdown_it_front_matter::FrontMatter as FrontMatterNode;

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
        crate::interlink_md::add(&mut parser);
        Self(parser)
    }

    pub fn parse(self: &Self, src: &str) -> Node {
        self.0.parse(src)
    }

    pub fn read_ast_from_file(self: &Self, path: &Path) -> Result<Node, std::io::Error> {
        read_to_string(path).map(|data| self.parse(&data))
    }
}
