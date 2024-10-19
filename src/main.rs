use std::io::Write;

use log::{error, info};

mod consequences;
mod consts;
mod glossary;
mod markdown;
mod qa_briefing;
mod rule;
mod rule_briefing;
mod rule_number;
mod ruleset;
mod ruleset_load_error;
mod ruleset_meta;
mod summary;
mod term;
mod text_normalization;

use consts::DIRECTORY_RULESETS;
use ruleset::Ruleset;
use ruleset_meta::load_ruleset_metas_from_disk;

fn main() {
    // This arcane incantation specifies that log level "info" should be the
    // default if RUST_LOG env var is not set.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!(
        "{} v{} says hello",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // TODO: Don't hard-code this path.
    let rulesets_directory = {
        let mut path = std::env::current_dir().unwrap();
        path.push(DIRECTORY_RULESETS);
        path
    };

    // TODO: Don't panic.
    let metas = load_ruleset_metas_from_disk(&rulesets_directory).unwrap();

    for meta in metas.into_iter() {
        let shortname = meta.shortname.clone();
        match Ruleset::load_using_meta(meta, &rulesets_directory) {
            Ok(rs) => render_rules(&rs),
            Err(e) => {
                error!("failed to read ruleset for {}: {:?}", shortname, e);
                std::process::exit(1);
            }
        }
    }
}

// TODO: move me somewhere! at this point I'm racing against the clock so code
// cleanliness is out the window, just gotta ship
fn render_rules(rs: &Ruleset) {
    let mut file = std::fs::File::create(&format!("{}.html", rs.meta.championship_year)).unwrap();
    file.write_all(rs.render().into_string().as_bytes()).unwrap()
}
