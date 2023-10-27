use log::{error, info};

mod consts;
mod glossary;
mod interlink_md;
mod ruleset;
mod ruleset_load_error;
mod ruleset_meta;
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
            Ok(_) => {}
            Err(e) => {
                error!("failed to read ruleset for {}: {:?}", shortname, e);
                std::process::exit(1);
            }
        }
    }
}
