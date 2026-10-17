//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "EffectCraft";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://discord.gg/officetools";
pub const WEBSITE: &str = "https://officetools.yosuaf.com";
pub const APP_PAGE: &str = "https://officetools.yosuaf.com/effectcraft";
pub const GITHUB: &str = "https://github.com/fyosua/officetools-effectcraft";
pub const ISSUES: &str = "https://github.com/fyosua/officetools-effectcraft/issues";

/// Sibling apps of the ArtCraft family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("PhotoCraft", "photocraft"),
    ("VectorCraft", "vectorcraft"),
    ("FilmCraft", "filmcraft"),
    ("LightCraft", "lightcraft"),
    ("PdfCraft", "pdfcraft"),
    ("DesignCraft", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://officetools.yosuaf.com/apps/{slug}")
}
