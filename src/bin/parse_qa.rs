use std::{io::Write, num::NonZeroU8, path::Path};

use log::info;
use scraper::{Html, Selector};

const FEED_PATH_TODO_NO_HARDCODE: &str = "./answers.rss";

pub fn main() {
    // This arcane incantation specifies that log level "info" should be the
    // default if RUST_LOG env var is not set.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!(
        "{}::parse_qa v{} says hello",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("attempting to parse feed {}", FEED_PATH_TODO_NO_HARDCODE);
    let rss_str = std::fs::read_to_string(FEED_PATH_TODO_NO_HARDCODE).unwrap();
    let feed_from_xml = feed_rs::parser::parse(rss_str.as_bytes()).unwrap();

    for entry in feed_from_xml.entries {
        let parsed_content = entry.summary.map(|s| Html::parse_document(&s.content));

        if let Some(content) = parsed_content {
            let number_selector = Selector::parse("[name=\"number\"]").unwrap();
            let question_selector = Selector::parse("[name=\"question\"]").unwrap();
            let answer_selector = Selector::parse("[name=\"answer\"]").unwrap();

            let mut number_nodes = content.select(&number_selector);
            let mut question_nodes = content.select(&question_selector);
            let mut answer_nodes = content.select(&answer_selector);

            let qa_num = number_nodes
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .parse::<NonZeroU8>()
                .unwrap();
            let question = question_nodes.next().unwrap().text().next().unwrap();
            let answer = answer_nodes
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .replace('*', "|");

            // TODO: Not hard-coded path.
            let qa_file = format!("./rulesets/2023-centerstage/qa/q{:0>3}.md", qa_num);

            if Path::new(&qa_file).exists() {
                info!("Q{} has already been written to disk, skipping!", qa_num);
                continue;
            }

            let mut file = std::fs::File::create(&qa_file).unwrap();
            file.write_all(
                &format!(
                    "---\nqa_reviewed = false\n---\n\n# Q{:0>3}: {}\n\nQ: {}\n\nA: {}",
                    qa_num,
                    entry
                        .title
                        // Next up, remove the "Qxxx " prefix each of these title lines has
                        .map(|t| t
                            .content
                            .as_str()
                            .trim()
                            .split_once(' ')
                            .unwrap()
                            .1
                            .to_string())
                        .unwrap_or("No Title Provided".into()),
                    question,
                    answer
                )
                .as_bytes(),
            )
            .unwrap();

            info!("wrote new {} to disk", &qa_file);
        }
    }
}
