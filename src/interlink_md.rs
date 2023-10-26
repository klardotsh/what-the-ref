use log::debug;
use markdown_it::parser::inline::{InlineRule, InlineState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

const CHAR_NEWLINE: char = '\n';
const CHAR_PIPE: char = '|';

#[inline]
#[cold]
// Thanks, https://users.rust-lang.org/t/compiler-hint-for-unlikely-likely-for-if-branches/62102/4
fn cold() {}

#[inline]
fn parse_display_text(src: &str) -> String {
    let mut buf = String::with_capacity(src.len() - 2 - src.match_indices(CHAR_NEWLINE).count());

    for chr in src.chars() {
        if chr == CHAR_NEWLINE || chr == CHAR_PIPE {
            continue;
        }

        buf.push(chr);
    }

    buf
}

#[derive(Debug)]
pub struct Interlink {
    display_text: String,
}

impl NodeValue for Interlink {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        // `node.attrs` are custom attributes added by other plugins
        // (for example, source mapping information)
        let mut attrs = node.attrs.clone();

        attrs.push(("class", "interlink".into()));

        fmt.open("span", &attrs);
        fmt.text(&self.display_text);
        fmt.close("span");
    }
}

// This is an extension for the inline subparser.
struct InterlinkScanner;

impl InlineRule for InterlinkScanner {
    // A required const for the trait impl which tells the AST parser to trigger
    // this rule when this char is found.
    const MARKER: char = CHAR_PIPE;

    // This is a custom function that will be invoked on every character
    // in an inline context.
    //
    // It should look for `state.src` exactly at position `state.pos`
    // and report if your custom structure appears there.
    //
    // If custom structure is found, it:
    //  - creates a new `Node` in AST
    //  - returns length of it
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        let input = &state.src[state.pos..state.pos_max];
        let pipe_indicies: Vec<(usize, &str)> = input.match_indices(CHAR_PIPE).collect();

        if pipe_indicies.is_empty() {
            // Hint to the compiler that this branch should basically never
            // happen.
            cold();

            debug!(
                "interlink detected but no pipe indicies found in: {}",
                input
            );
            return None;
        }

        if let Some((fidx, _)) = pipe_indicies.first() {
            if *fidx > 0 {
                // This should also basically never happen unless there's been
                // some sort of bug in a prior node creation in our plugin here
                // (and even then, I'm not sure it can happen)
                cold();

                debug!(
                    "interlink detected but first pipe index is not 0 (found {} instead) in: {}",
                    fidx, input
                );
                return None;
            }
        }

        pipe_indicies.get(1).map(|(end_idx, _)| {
            let len = *end_idx + 1;
            let interlink_src = input[0..len].to_string();
            let display_text = parse_display_text(&interlink_src);
            debug!(
                "interlink detected. raw len: {}, display_text: {}",
                len, display_text
            );
            (Node::new(Interlink { display_text }), len)
        })
    }
}

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into inline subparser
    md.inline.add_rule::<InterlinkScanner>();
}
