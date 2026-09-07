//! The complete, embedded Lucide icon set for GPUI.
//!
//! All canonical icons and aliases come from the pinned Lucide 1.33.0 release.
//! Native and WASM applications render the same upstream SVG bytes, with no
//! network requests, font glyphs, or substitute shapes.

use std::{borrow::Cow, io};

use gpui::{AssetSource, Result, SharedString, Svg, svg};

include!("generated.rs");

/// The asset namespace served by [`LucideAssetSource`].
pub const LUCIDE_ASSET_NAMESPACE: &str = "icons/lucide/";

/// The bundled manifest with upstream metadata and hashes for every icon.
pub const RELEASE_MANIFEST: &str = include_str!("../RELEASE-MANIFEST.json");

impl LucideIcon {
    /// Returns the pinned canonical kebab-case name.
    pub const fn canonical_name(self) -> &'static str {
        ICONS[self as usize].0
    }

    /// Returns this icon's stable, namespaced SVG path.
    pub const fn asset_path(self) -> &'static str {
        ICONS[self as usize].1
    }

    /// Returns the exact upstream SVG bytes embedded in the application.
    pub const fn svg_bytes(self) -> &'static [u8] {
        ICONS[self as usize].2
    }

    /// Looks up a canonical name or an upstream alias, including legacy names.
    ///
    /// Matching is case-sensitive. Unknown names return `None`.
    pub fn from_name(name: &str) -> Option<Self> {
        ICONS
            .binary_search_by_key(&name, |entry| entry.0)
            .ok()
            .map(|index| Self::ALL[index])
            .or_else(|| {
                ALIASES
                    .binary_search_by_key(&name, |entry| entry.0)
                    .ok()
                    .map(|index| ALIASES[index].1)
            })
    }

    /// Creates a GPUI SVG element.
    ///
    /// Set both `.size(px(16.))` and `.text_color(...)` with GPUI styling.
    /// GPUI requires an explicit color to paint an SVG.
    /// Register [`LucideAssetSource`] with the application before rendering.
    /// Give an icon-only control an accessible label on its parent control.
    pub fn element(self) -> Svg {
        svg().path(self.asset_path())
    }
}

/// Creates a GPUI SVG element for an icon.
pub fn lucide(icon: LucideIcon) -> Svg {
    icon.element()
}

/// The complete embedded Lucide asset source.
///
/// Returns `Ok(None)` for unrelated paths so your app can delegate to another
/// asset source. An unknown Lucide path is an explicit not-found error.
#[derive(Clone, Copy, Debug, Default)]
pub struct LucideAssetSource;

impl LucideAssetSource {
    /// Creates the stateless embedded asset source.
    pub const fn new() -> Self {
        Self
    }
}

impl AssetSource for LucideAssetSource {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let Some(name) = path.strip_prefix(LUCIDE_ASSET_NAMESPACE) else {
            return Ok(None);
        };
        match name.strip_suffix(".svg").and_then(LucideIcon::from_name) {
            Some(icon) => Ok(Some(Cow::Borrowed(icon.svg_bytes()))),
            None => missing_asset(path),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        match path {
            "icons/lucide" | LUCIDE_ASSET_NAMESPACE => Ok(LucideIcon::ALL
                .iter()
                .map(|icon| icon.asset_path().into())
                .collect()),
            _ if path.starts_with(LUCIDE_ASSET_NAMESPACE) => missing_asset(path),
            _ => Ok(Vec::new()),
        }
    }
}

fn missing_asset<T>(path: &str) -> Result<T> {
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("No embedded Lucide asset at {path:?}"),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn full_release_is_searchable_and_matches_upstream() {
        let manifest: serde_json::Value = serde_json::from_str(RELEASE_MANIFEST).unwrap();
        let entries = manifest["icons"].as_array().unwrap();
        let renderer = gpui::SvgRenderer::new(std::sync::Arc::new(LucideAssetSource));
        assert_eq!(entries.len(), 1776);
        assert_eq!(LucideIcon::ALL.len(), entries.len());
        for (icon, entry) in LucideIcon::ALL.into_iter().zip(entries) {
            assert_eq!(icon.canonical_name(), entry["canonical_name"]);
            assert_eq!(LucideIcon::from_name(icon.canonical_name()), Some(icon));
            assert_eq!(icon.asset_path(), entry["asset_path"]);
            let bytes = LucideAssetSource.load(icon.asset_path()).unwrap().unwrap();
            assert_eq!(format!("{:x}", Sha256::digest(&bytes)), entry["svg_sha256"]);
            let rendered = renderer.render_single_frame(&bytes, 1.).unwrap();
            assert!(
                rendered
                    .as_bytes(0)
                    .unwrap()
                    .chunks_exact(4)
                    .any(|pixel| pixel[3] != 0),
                "{} renders blank",
                icon.canonical_name()
            );
            let metadata = std::fs::read(format!(
                "{}/assets/lucide/{}.json",
                env!("CARGO_MANIFEST_DIR"),
                icon.canonical_name()
            ))
            .unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(metadata)),
                entry["json_sha256"]
            );
        }
        for (alias, icon) in ALIASES {
            assert_eq!(LucideIcon::from_name(alias), Some(icon));
        }
        assert_eq!(
            LucideAssetSource
                .list(LUCIDE_ASSET_NAMESPACE)
                .unwrap()
                .len(),
            entries.len()
        );
        assert!(LucideIcon::from_name("not-an-icon").is_none());
        assert!(
            LucideAssetSource
                .load("icons/lucide/not-an-icon.svg")
                .is_err()
        );
        assert!(LucideAssetSource.load("icons/lucide/../check.svg").is_err());
        assert_eq!(LucideAssetSource.load("fonts/app.ttf").unwrap(), None);
        assert_eq!(
            format!("{:x}", Sha256::digest(include_bytes!("../LICENSE"))),
            "b495047bd93a9b06913511076f504daba17d5bbeb3e0650f3bb53a4220329c57"
        );
    }
}
