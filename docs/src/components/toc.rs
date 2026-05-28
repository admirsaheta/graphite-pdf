use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub anchor: String,
}

#[derive(Properties, PartialEq)]
pub struct TocProps {
    pub entries: Vec<TocEntry>,
}

#[component]
pub fn Toc(props: &TocProps) -> Html {
    if props.entries.is_empty() {
        return html! {};
    }

    html! {
        <aside class="hidden xl:flex flex-col w-52 shrink-0 overflow-y-auto">
            <div class="sticky top-20 px-4 py-6">
                <p class="mb-3 text-[11px] font-semibold uppercase tracking-[0.1em] text-[#585856]">
                    {"On this page"}
                </p>
                <ul class="space-y-1">
                    { for props.entries.iter().map(|entry| {
                        let indent = if entry.level == 3 { "pl-3" } else { "pl-0" };
                        let anchor = format!("#{}", entry.anchor);
                        html! {
                            <li class={indent}>
                                <a
                                    href={anchor}
                                    class="block text-[13px] text-[#585856] hover:text-[#D4581A] transition-colors py-0.5 leading-snug"
                                >
                                    { &entry.text }
                                </a>
                            </li>
                        }
                    })}
                </ul>
            </div>
        </aside>
    }
}

pub fn extract_toc(markdown: &str) -> Vec<TocEntry> {
    let mut entries = Vec::new();
    let mut in_code_block = false;

    for line in markdown.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }

        let (level, text) = if let Some(t) = trimmed.strip_prefix("### ") {
            (3u8, t)
        } else if let Some(t) = trimmed.strip_prefix("## ") {
            (2u8, t)
        } else {
            continue;
        };

        let anchor = text
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' {
                    c
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");

        entries.push(TocEntry {
            level,
            text: text.to_string(),
            anchor,
        });
    }

    entries
}
