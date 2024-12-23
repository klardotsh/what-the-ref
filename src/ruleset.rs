use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use log::{debug, error, info};
use serde::Deserialize;
use maud::{DOCTYPE, html, Markup, PreEscaped};

use crate::consts::DIRECTORY_GLOSSARY;
use crate::glossary::Glossary;
use crate::rule::Rule;
use crate::rule_number::RuleNumber;
use crate::ruleset_load_error::RulesetLoadError;
use crate::ruleset_meta::RulesetMeta;
use crate::summary::Summary;

const ASSUMED_NUMBER_OF_RULES: usize = 50;

// Change this annually to something reasonable. This is used to denote rules which are
// season-specific (as opposed to evergreen).
const SEASON_EMOJI: &str = "🪸";

type RulesByNumber = IndexMap<RuleNumber, Rule>;

#[derive(Debug, Deserialize)]
pub struct Ruleset {
    #[serde(skip_deserializing, default = "Default::default")]
    pub generated: chrono::DateTime<chrono::Utc>,
    pub glossary: Glossary,
    pub meta: RulesetMeta,
    #[serde(skip_deserializing)]
    pub rules: RulesByNumber,
}

impl Ruleset {
    pub fn load_using_meta(meta: RulesetMeta, path: &Path) -> Result<Self, RulesetLoadError> {
        let mut path = PathBuf::from(path);
        path.push(&meta.directory);
        if !path.exists() {
            return Err(RulesetLoadError::NoSuchRulesetDirectory(
                meta.directory.clone(),
                path.clone(),
            ));
        }

        path.push(DIRECTORY_GLOSSARY);
        let glossary = Glossary::from_markdown_directory(&path)?;
        assert!(path.pop());

        let mut rules: RulesByNumber = IndexMap::with_capacity(ASSUMED_NUMBER_OF_RULES);

        for source in meta.sources.iter() {
            path.push(&source.directory);

            if !path.exists() {
                return Err(RulesetLoadError::NoSuchRuleSourceDirectory(
                    meta.directory.clone(),
                    path.clone(),
                ));
            }

            info!("attempting to find rules in {}", path.to_string_lossy());

            let files = std::fs::read_dir(&path)?;
            for file in files {
                let file_path = file?.path();
                let rule = Rule::load_from_markdown_file(&file_path)?;

                // TODO: Cloning the rule number is a dirty hack because I
                // didn't feel like trying to get the data structures right to
                // allow it to be a pointer instead. Both the key and value of
                // this IndexMap would have to borrow from some higher-altitude
                // structure that would live at least as long as the IndexMap
                // to allow the key to reference part of the value. Why, then,
                // you might ask, is this not an IndexSet instead? Because
                // IndexMaps, required as part of Summary::PerSubRule, don't
                // implement Hash, and I was too lazy to write an impl myself
                // tonight. And so since that one leaf in the tree can't be
                // hashed, none of the structure can auto-derive Hash, and so I
                // gave up and did this the dirty, lazy, JavaScript/Python-like
                // way. Software's gotta ship, yo, and events are in... 3 days
                // from time of writing this comment.
                debug!("parsed rule: {:?}", &rule);

                // Q&A are special kinds of rules which augment other rules from the manuals. Go find those relevant rules and let them know the
                if let Some(Summary::QA(briefing)) = &rule.summary {
                    if let Some(references_rules) = &briefing.references_rules {
                        for refd_rule in references_rules {
                            if let Some(ref mut source_rule) = rules.get_mut(refd_rule) {
                                source_rule.backreferences.push(rule.number.clone());
                                source_rule.backreferences.sort();
                            } else {
                                error!(
                                    "{} refers to unrecognized rule {}",
                                    &rule.number, &refd_rule
                                );
                            }
                        }
                    }
                }

                rules.insert(rule.number.clone(), rule);
            }

            assert!(path.pop());
        }

        rules.sort_keys();

        Ok(Self {
            glossary,
            meta,
            rules,
            generated: chrono::Utc::now(),
        })
    }

    pub fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { (format!("{} | What The Ref?", self.meta.longname)) }
            style { (PreEscaped(include_str!("../main.css"))) }
            script type="text/javascript" { (PreEscaped(include_str!("../main.js"))) }

            header {
                nav {
                    h1 id="wtr-branding" {
                        "What The Ref?!"
                        br;
                        "augmented FIRST game manuals for referees"
                    }
                    h2 class="gamename" {
                        (self.meta.longname)
                    }
                    p {
                        (format!(
                            "This {} resource for {} generated {}.",
                            self.meta.program.display_name(),
                            self.meta.championship_year,
                            self.generated.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                        ))
                    }
                    p {
                        "All information within was pulled "
                        a href=(self.meta.source_material.href) { "from FIRST" }
                        (format!(
                            " on {} (UTC), which includes Team Update {} and all prior.",
                            self.meta.source_material.accessed.date_naive(),
                            self.meta.source_material.latest_team_update_included,
                        ))
                    }
                }

                blockquote {
                    p class="disclaimer" {
                        "Made with ♡ by "
                        a href="//klar.sh" { "Josh Klar (email: josh [at] klar [dot] sh)" }
                        " in Washington State. This "
                        a href="//codeberg.org/FriendsWithRobots/what-the-ref" {
                            "open-source project you can contribute to"
                        }
                        " is not affiliated with FIRST HQ, and is not an official game resource. "
                        em { "Always" }
                        " retain and refer to a copy of the Game Manual PDF the latest Q&A PDF as your final sources of truth, and use this only as a helpful index and reference."
                    }
                }

                div id="global-expansion-state-buttons" class="flexy" {
                    button onclick="openAllDetails()" class="flex flex-push-left-50" {
                        "Expand Everything"
                        br;
                        "(eg. for browser search)"
                    }
                    button onclick="closeAllDetails()" class="flex flex-push-left-50" {
                        "Collapse Everything"
                    }
                }
            }

            div id="content" {
                (self.render_rules())
                (self.render_glossary())
            }
        }
    }

    fn render_glossary(&self) -> Markup {
        html! {
            div id="glossary" {
                h1 { "Glossary" }
                (click_tap_expand_msg())
                @for term in &self.glossary.terms {
                    details class="rule" {
                        summary { span class="description" { (term.name) } }

                        @for anchor in &term.anchors {
                            (jumpable_anchor(anchor))
                        }

                        (maud::PreEscaped(&term.rendered_html))
                    }
                }
            }
        }
    }

    fn render_rules(&self) -> Markup {
        html! {
            div id="rules" {
                h1 { "Rules" }

                (click_tap_expand_msg())
                p class="centered" {
                    "Evergreen rules indicated by "
                    (evergreen_span())
                    ", season-specific rules indicated by "
                    (non_evergreen_span())
                    "."
                }
                p class="centered" {
                    "Consequence hints ending in a * indicate optional / head ref discretion."
                }

                blockquote {
                    p class="disclaimer" {
                        "Please note that G4xx (In-Match) rules are displayed at the top of the list on the premise that they are most relevant while refereeing a match. G1xx (Safety), G2xx (Conduct), G3xx (Pre-Match), and G5xx (Post-Match) rules are all here too, just keep scrolling!"
                    }
                }

                @for (_, rule) in &self.rules {
                    (render_rule(rule))
                }
            }
        }
    }
}

fn click_tap_expand_msg() -> Markup {
    html! {
        p class="centered" { "Click/tap any of these to expand." }
    }
}

fn render_rule(rule: &Rule) -> Markup {
    let classes = if rule.number.begins_new_section() {
        "rule begins-new-section"
    } else {
        "rule"
    };

    html! {
        details class=(classes) {
            (jumpable_anchor(&rule.number.anchor()))
            summary class="flexy" { (render_rule_header(&rule)) }
            (render_rule_details(&rule))
        }
    }
}

fn render_rule_header(rule: &Rule) -> Markup {
    html! {
        span class="flex push-left-50 description" {
            @match &rule.summary {
                Some(Summary::EntireRule(rb)) => {
                    @if rb.evergreen {
                        (evergreen_span())
                    } @else {
                        (non_evergreen_span())
                    }
                }
                _ => (non_evergreen_span())
            }
            " "

            strong { (format!("{}: ", rule.number)) }
            (rule.title)
        }

        div class="flex flexy" {
            @if let Some(summary) = &rule.summary {
                @match summary {
                    Summary::EntireRule(rb) => @for cs in &rb.matrix {
                        span class=(format!("flex consequence {}", cs.css_class())) {
                            (cs.pill_text())
                        }
                    },
                    Summary::PerSubRule(_) => span class="flex consequence per-subrule-interventions" {
                        "VARIOUS"
                    },
                    Summary::QA(qa) => @if !qa.reviewed {
                        span class="flex consequence qa-unreviewed" { "UNREVIEWED" }
                    }
                }
            }
        }
    }
}

fn render_rule_details(rule: &Rule) -> Markup {
    html! {
        (render_rule_detail_prelude(rule))
        (maud::PreEscaped(&rule.full_html))
        (render_rule_backreferences(rule))
    }
}

fn render_rule_detail_prelude(rule: &Rule) -> Markup {
    html! {
        @if let Some(summary) = &rule.summary {
            @match summary {
                Summary::EntireRule(rb) => div class="centered" {
                    (maud::PreEscaped(&rb.description))
                    hr;
                },
                Summary::PerSubRule(rbs) => @for (subrule, rb) in rbs {
                    div class="subrule flexy" {
                        span class="flex push-left-10" {
                            (format!("{}.{}", rule.number, subrule))
                        }
                        div class="flex flex-push-left-50 subrule-description" {
                            (maud::PreEscaped(&rb.description))
                        }
                        div class="flex flex-push-left-30 flexy" {
                            @for cs in &rb.matrix {
                                span class=(format!("flex consequence {}", cs.css_class())) {
                                    (cs.pill_text())
                                }
                            }
                        }
                    }
                },
                Summary::QA(qa) => {
                    div class="centered" {
                        "This Q&A references "
                        @match &qa.references_rules {
                            Some(r) => (r.len().to_string()),
                            None => ("0".to_string()),
                        }
                        " rules, linked within."
                    }
                }
            }
        }
    }
}

fn render_rule_backreferences(rule: &Rule) -> Markup {
    html! {
        @if !rule.backreferences.is_empty() {
            div class="backlinks" {
                h4 { "This rule is referenced by..." }
                @for refe in &rule.backreferences {
                    span class="backlink" {
                        a href=(format!("#{}", refe.anchor())) { (refe) }
                    }
                }
            }
        }
    }
}

fn jumpable_anchor(id: &str) -> Markup {
    html! {
        a id=(id) class="jump-to" {}
    }
}

fn evergreen_span() -> Markup {
    html! {
        span class="evergreen-status" { "🌲" }
    }
}

fn non_evergreen_span() -> Markup {
    html! {
        span class="evergreen-status" { (SEASON_EMOJI) }
    }
}
