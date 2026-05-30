// Content is in docs/content/**/*.md and embedded at compile time via include_str!.
// This module owns only routing and navigation — no inline markdown.

pub fn get_static_content(section: &str, page: &str) -> Option<&'static str> {
    match (section, page) {
        // Getting started
        ("getting-started", "introduction") => Some(include_str!("../../content/getting-started/introduction.md")),
        ("getting-started", "installation") => Some(include_str!("../../content/getting-started/installation.md")),
        ("getting-started", "quickstart")   => Some(include_str!("../../content/getting-started/quickstart.md")),

        // Architecture
        ("architecture", "overview")          => Some(include_str!("../../content/architecture/overview.md")),
        ("architecture", "rendering-process") => Some(include_str!("../../content/architecture/rendering-process.md")),
        ("architecture", "compatibility")     => Some(include_str!("../../content/architecture/compatibility.md")),

        // Crates
        ("crates", "errors")      => Some(include_str!("../../content/crates/errors.md")),
        ("crates", "primitives")  => Some(include_str!("../../content/crates/primitives.md")),
        ("crates", "utils")       => Some(include_str!("../../content/crates/utils.md")),
        ("crates", "svg")         => Some(include_str!("../../content/crates/svg.md")),
        ("crates", "stylesheet")  => Some(include_str!("../../content/crates/stylesheet.md")),
        ("crates", "font")        => Some(include_str!("../../content/crates/font.md")),
        ("crates", "math")        => Some(include_str!("../../content/crates/math.md")),
        ("crates", "textkit")     => Some(include_str!("../../content/crates/textkit.md")),
        ("crates", "image")       => Some(include_str!("../../content/crates/image.md")),
        ("crates", "kit")         => Some(include_str!("../../content/crates/kit.md")),
        ("crates", "layout")      => Some(include_str!("../../content/crates/layout.md")),
        ("crates", "render")      => Some(include_str!("../../content/crates/render.md")),
        ("crates", "renderer")    => Some(include_str!("../../content/crates/renderer.md")),
        ("crates", "style")       => Some(include_str!("../../content/crates/style.md")),
        ("crates", "document")    => Some(include_str!("../../content/crates/document.md")),

        // graphitepdf (facade) — rich README on GitHub, fall through to fetch
        _ => None,
    }
}

pub fn github_raw_url(section: &str, page: &str) -> Option<String> {
    if section == "crates" {
        Some(format!(
            "https://raw.githubusercontent.com/admirsaheta/graphite-pdf/main/crates/{}/README.md",
            page
        ))
    } else {
        None
    }
}

pub fn github_edit_url(section: &str, page: &str) -> String {
    match section {
        "crates" => format!(
            "https://github.com/admirsaheta/graphite-pdf/edit/main/docs/content/crates/{}.md",
            page
        ),
        "architecture" => format!(
            "https://github.com/admirsaheta/graphite-pdf/edit/main/docs/content/architecture/{}.md",
            page
        ),
        _ => format!(
            "https://github.com/admirsaheta/graphite-pdf/edit/main/docs/content/{}/{}.md",
            section, page
        ),
    }
}

// ── Navigation ────────────────────────────────────────────────────────────────

#[derive(PartialEq)]
pub struct NavSection {
    pub label: &'static str,
    pub items: &'static [NavItem],
}

#[derive(PartialEq)]
pub struct NavItem {
    pub label: &'static str,
    pub section: &'static str,
    pub page: &'static str,
}

pub const NAV: &[NavSection] = &[
    NavSection {
        label: "Getting Started",
        items: &[
            NavItem { label: "Introduction", section: "getting-started", page: "introduction" },
            NavItem { label: "Installation",  section: "getting-started", page: "installation" },
            NavItem { label: "Quick Start",   section: "getting-started", page: "quickstart" },
        ],
    },
    NavSection {
        label: "Architecture",
        items: &[
            NavItem { label: "Overview",          section: "architecture", page: "overview" },
            NavItem { label: "Rendering Process", section: "architecture", page: "rendering-process" },
            NavItem { label: "Compatibility",     section: "architecture", page: "compatibility" },
        ],
    },
    NavSection {
        label: "Crates",
        items: &[
            NavItem { label: "document",   section: "crates", page: "document" },
            NavItem { label: "errors",     section: "crates", page: "errors" },
            NavItem { label: "font",       section: "crates", page: "font" },
            NavItem { label: "image",      section: "crates", page: "image" },
            NavItem { label: "kit",        section: "crates", page: "kit" },
            NavItem { label: "layout",     section: "crates", page: "layout" },
            NavItem { label: "math",       section: "crates", page: "math" },
            NavItem { label: "primitives", section: "crates", page: "primitives" },
            NavItem { label: "render",     section: "crates", page: "render" },
            NavItem { label: "renderer",   section: "crates", page: "renderer" },
            NavItem { label: "style",      section: "crates", page: "style" },
            NavItem { label: "stylesheet", section: "crates", page: "stylesheet" },
            NavItem { label: "svg",        section: "crates", page: "svg" },
            NavItem { label: "textkit",    section: "crates", page: "textkit" },
            NavItem { label: "utils",      section: "crates", page: "utils" },
        ],
    },
];
