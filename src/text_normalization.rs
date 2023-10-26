use once_cell::sync::Lazy;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};

const UNDERSCORE: char = '_';
const UNDERSCORE_REPLACEMENT: &str = "_";

// I subjectively don't love this stemmer (Snowball) for a lot of reasons:
// notably, it makes some silly stems like "penalti" for "penalty". However,
// it's fully self-contained by way of bundling its dictionaries in a
// Rust-friendly way, whereas Hunspell, which IMO makes better stems, is
// significantly more work to set up here. Further, rust-stemmers (which bundles
// Snowball's output) is pure Rust, whereas hunspell_rs would take a hunspell-
// sys dependency, which then requires C build tooling. "Good enough" is going
// to have to be good enough here, and we'll just have some awkward filenames
// and page anchors behind the scenes. This has no particular impact on end
// users anyway; the display text in the HTML is never stemmed.
static ENGLISH_STEMMER: Lazy<Stemmer> = Lazy::new(|| Stemmer::create(Algorithm::English));
static NON_WORD_CHARS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\W+").unwrap());

/// Take a human-readable title and convert it into something fit to be a page
/// anchor or file name. Leading whitespace is discarded, and inner whitespace
/// and special characters (anything that isn't a UTF-8 alpha-numeric codepoint,
/// really) are truncated to a single underscore, regardless of how many such
/// characters were found.
///
/// For example, "In (Inside) / Completely In (Completely Inside)" becomes
/// "in_inside_completely_in_completely_inside".
pub fn anchorize(src: &str) -> String {
    // First, let's de-junk-ify the inbound string by deduplicating whitespace,
    // eliminating special characters, etc.
    let unnormalized = NON_WORD_CHARS
        .replace_all(&src.trim().to_lowercase(), UNDERSCORE_REPLACEMENT)
        .trim_matches(UNDERSCORE) // don't want to start/end in _
        .to_string();

    // Now re-split the string by underscores and normalize each chunk to make
    // these anchors stable references even if the manual refers to a plural or
    // other non-normalized form.
    unnormalized
        .split(UNDERSCORE)
        .map(normalize)
        .collect::<Vec<String>>()
        .join(UNDERSCORE_REPLACEMENT) // can't use a char on a string vec
        .into()
}

fn normalize(src: &str) -> String {
    ENGLISH_STEMMER.stem(&src.to_lowercase()).into()
}

#[test]
fn test_ridiculous_glossary_term() {
    assert_eq!(
        "in_insid_complet_in_complet_insid",
        anchorize("In (Inside) / Completely In (Completely Inside)")
    );
}

#[test]
fn test_single_word_refs() {
    assert_eq!("robot", anchorize("robot"));
    assert_eq!("robot", anchorize("Robot"));
    assert_eq!("robot", anchorize("robots"));
    assert_eq!("robot", anchorize("Robots"));

    assert_eq!("penalti", anchorize("penalty"));
    assert_eq!("penalti", anchorize("Penalty"));
    assert_eq!("penalti", anchorize("penalties"));
    assert_eq!("penalti", anchorize("Penalties"));

    assert_eq!("propel", anchorize("propel"));
    assert_eq!("propel", anchorize("propelled"));
    assert_eq!("propel", anchorize("propelling"));
}

#[test]
fn test_multi_word_refs() {
    assert_eq!("autonom_period", anchorize("autonomous period"));
    assert_eq!("autonom_period", anchorize("Autonomous period"));
    assert_eq!("autonom_period", anchorize("Autonomous periods"));
    assert_eq!("autonom_period", anchorize("autonomous Period"));
    assert_eq!("autonom_period", anchorize("Autonomous Period"));
    assert_eq!("autonom_period", anchorize("Autonomous             Period"));
    assert_eq!(
        "autonom_period",
        anchorize("Autonomous             Periods")
    );
}
