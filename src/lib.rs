//! A small, embedded Lucide icon pack for GPUI.
//!
//! `gpui-icons` is an unofficial GPUI port of the pinned Lucide 1.33.0 assets
//! listed in [`RELEASE_MANIFEST`]. It intentionally contains only the icons
//! needed by gpuicn's shipped component catalog.

use std::{borrow::Cow, io};

use gpui::{AssetSource, Result, SharedString, Svg, svg};

/// The asset namespace served by [`LucideAssetSource`].
pub const LUCIDE_ASSET_NAMESPACE: &str = "icons/lucide/";

/// The release provenance manifest bundled with this crate.
pub const RELEASE_MANIFEST: &str = include_str!("../RELEASE-MANIFEST.json");

/// An allow-listed Lucide icon.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LucideIcon {
    /// The Lucide `bold` icon.
    Bold,
    /// The Lucide `check` icon.
    Check,
    /// The Lucide `chevron-down` icon.
    ChevronDown,
    /// The Lucide `chevron-right` icon.
    ChevronRight,
    /// The Lucide `chevron-up` icon.
    ChevronUp,
    /// The Lucide `circle` icon.
    Circle,
    /// The Lucide `copy` icon.
    Copy,
    /// The Lucide `italic` icon.
    Italic,
    /// The Lucide `minus` icon.
    Minus,
    /// The Lucide `plus` icon.
    Plus,
    /// The Lucide `search` icon.
    Search,
    /// The Lucide `text-align-center` icon.
    TextAlignCenter,
    /// The Lucide `text-align-end` icon.
    TextAlignEnd,
    /// The Lucide `text-align-start` icon.
    TextAlignStart,
    /// The Lucide `underline` icon.
    Underline,
    /// The Lucide `x` icon.
    X,
}

impl LucideIcon {
    /// Every canonical icon in this release, in release-manifest order.
    pub const ALL: [Self; 16] = [
        Self::Bold,
        Self::Check,
        Self::ChevronDown,
        Self::ChevronRight,
        Self::ChevronUp,
        Self::Circle,
        Self::Copy,
        Self::Italic,
        Self::Minus,
        Self::Plus,
        Self::Search,
        Self::TextAlignCenter,
        Self::TextAlignEnd,
        Self::TextAlignStart,
        Self::Underline,
        Self::X,
    ];

    /// Returns the pinned Lucide canonical name.
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::Bold => "bold",
            Self::Check => "check",
            Self::ChevronDown => "chevron-down",
            Self::ChevronRight => "chevron-right",
            Self::ChevronUp => "chevron-up",
            Self::Circle => "circle",
            Self::Copy => "copy",
            Self::Italic => "italic",
            Self::Minus => "minus",
            Self::Plus => "plus",
            Self::Search => "search",
            Self::TextAlignCenter => "text-align-center",
            Self::TextAlignEnd => "text-align-end",
            Self::TextAlignStart => "text-align-start",
            Self::Underline => "underline",
            Self::X => "x",
        }
    }

    /// Returns this icon's stable, namespaced asset path.
    pub const fn asset_path(self) -> &'static str {
        match self {
            Self::Bold => "icons/lucide/bold.svg",
            Self::Check => "icons/lucide/check.svg",
            Self::ChevronDown => "icons/lucide/chevron-down.svg",
            Self::ChevronRight => "icons/lucide/chevron-right.svg",
            Self::ChevronUp => "icons/lucide/chevron-up.svg",
            Self::Circle => "icons/lucide/circle.svg",
            Self::Copy => "icons/lucide/copy.svg",
            Self::Italic => "icons/lucide/italic.svg",
            Self::Minus => "icons/lucide/minus.svg",
            Self::Plus => "icons/lucide/plus.svg",
            Self::Search => "icons/lucide/search.svg",
            Self::TextAlignCenter => "icons/lucide/text-align-center.svg",
            Self::TextAlignEnd => "icons/lucide/text-align-end.svg",
            Self::TextAlignStart => "icons/lucide/text-align-start.svg",
            Self::Underline => "icons/lucide/underline.svg",
            Self::X => "icons/lucide/x.svg",
        }
    }

    /// Returns a GPUI SVG element for this icon.
    ///
    /// The element inherits its surrounding text color. Apply normal GPUI
    /// styling such as `.size(px(16.))` or `.text_color(color)` at the call
    /// site. The application must compose [`LucideAssetSource`] into the
    /// [`gpui::App`] asset source before rendering it.
    pub fn element(self) -> Svg {
        svg().path(self.asset_path())
    }
}

/// Returns a GPUI SVG element for an allow-listed Lucide icon.
pub fn lucide(icon: LucideIcon) -> Svg {
    icon.element()
}

/// An embedded asset source for the [`LUCIDE_ASSET_NAMESPACE`] namespace.
///
/// It returns `Ok(None)` for unrelated paths so an application can delegate
/// only this namespace and retain ownership of every other asset. Unknown
/// paths inside this namespace return a not-found error instead of a fallback
/// glyph.
#[derive(Clone, Copy, Debug, Default)]
pub struct LucideAssetSource;

impl LucideAssetSource {
    /// Creates the stateless embedded Lucide asset source.
    pub const fn new() -> Self {
        Self
    }
}

impl AssetSource for LucideAssetSource {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let bytes = match path {
            "icons/lucide/bold.svg" => Some(include_bytes!("../assets/lucide/bold.svg").as_slice()),
            "icons/lucide/check.svg" => {
                Some(include_bytes!("../assets/lucide/check.svg").as_slice())
            }
            "icons/lucide/chevron-down.svg" => {
                Some(include_bytes!("../assets/lucide/chevron-down.svg").as_slice())
            }
            "icons/lucide/chevron-right.svg" => {
                Some(include_bytes!("../assets/lucide/chevron-right.svg").as_slice())
            }
            "icons/lucide/chevron-up.svg" => {
                Some(include_bytes!("../assets/lucide/chevron-up.svg").as_slice())
            }
            "icons/lucide/circle.svg" => {
                Some(include_bytes!("../assets/lucide/circle.svg").as_slice())
            }
            "icons/lucide/copy.svg" => Some(include_bytes!("../assets/lucide/copy.svg").as_slice()),
            "icons/lucide/italic.svg" => {
                Some(include_bytes!("../assets/lucide/italic.svg").as_slice())
            }
            "icons/lucide/minus.svg" => {
                Some(include_bytes!("../assets/lucide/minus.svg").as_slice())
            }
            "icons/lucide/plus.svg" => Some(include_bytes!("../assets/lucide/plus.svg").as_slice()),
            "icons/lucide/search.svg" => {
                Some(include_bytes!("../assets/lucide/search.svg").as_slice())
            }
            "icons/lucide/text-align-center.svg" => {
                Some(include_bytes!("../assets/lucide/text-align-center.svg").as_slice())
            }
            "icons/lucide/text-align-end.svg" => {
                Some(include_bytes!("../assets/lucide/text-align-end.svg").as_slice())
            }
            "icons/lucide/text-align-start.svg" => {
                Some(include_bytes!("../assets/lucide/text-align-start.svg").as_slice())
            }
            "icons/lucide/underline.svg" => {
                Some(include_bytes!("../assets/lucide/underline.svg").as_slice())
            }
            "icons/lucide/x.svg" => Some(include_bytes!("../assets/lucide/x.svg").as_slice()),
            _ if path.starts_with(LUCIDE_ASSET_NAMESPACE) => return missing_asset(path),
            _ => None,
        };

        Ok(bytes.map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        match path {
            "icons/lucide" | LUCIDE_ASSET_NAMESPACE => Ok(LucideIcon::ALL
                .iter()
                .map(|icon| SharedString::from(icon.asset_path()))
                .collect()),
            _ if path.starts_with(LUCIDE_ASSET_NAMESPACE) => missing_asset(path),
            _ => Ok(Vec::new()),
        }
    }
}

fn missing_asset<T>(path: &str) -> Result<T> {
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("gpui-icons has no embedded asset at {path:?}"),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    fn sha256(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }

    #[test]
    fn release_contains_the_catalog_allow_list() {
        assert_eq!(LucideIcon::ALL.len(), 16);
    }

    #[test]
    fn canonical_names_and_asset_paths_are_stable() {
        for icon in LucideIcon::ALL {
            assert_eq!(
                icon.asset_path(),
                format!("icons/lucide/{}.svg", icon.canonical_name())
            );
        }
    }

    #[test]
    fn embedded_assets_match_the_release_manifest_hashes() {
        let manifest: serde_json::Value = serde_json::from_str(RELEASE_MANIFEST).unwrap();
        let entries = manifest["icons"].as_array().unwrap();

        for icon in LucideIcon::ALL {
            let entry = entries
                .iter()
                .find(|entry| entry["canonical_name"] == icon.canonical_name())
                .unwrap();
            let svg = LucideAssetSource.load(icon.asset_path()).unwrap().unwrap();
            let metadata =
                std::fs::read(format!("assets/lucide/{}.json", icon.canonical_name())).unwrap();

            assert_eq!(sha256(&svg), entry["svg_sha256"]);
            assert_eq!(sha256(&metadata), entry["json_sha256"]);
        }
    }

    #[test]
    fn bundled_lucide_license_matches_the_pinned_release() {
        assert_eq!(
            sha256(include_bytes!("../LICENSE")),
            "b495047bd93a9b06913511076f504daba17d5bbeb3e0650f3bb53a4220329c57"
        );
    }

    #[test]
    fn source_loads_each_allow_listed_path() {
        let source = LucideAssetSource;

        assert!(
            LucideIcon::ALL
                .iter()
                .all(|icon| source.load(icon.asset_path()).unwrap().is_some())
        );
    }

    #[test]
    fn source_delegates_unrelated_paths() {
        assert_eq!(LucideAssetSource.load("fonts/app.woff2").unwrap(), None);
    }

    #[test]
    fn source_rejects_unknown_lucide_paths() {
        assert!(LucideAssetSource.load("icons/lucide/missing.svg").is_err());
    }

    #[test]
    fn source_lists_the_allow_listed_paths() {
        assert_eq!(
            LucideAssetSource
                .list(LUCIDE_ASSET_NAMESPACE)
                .unwrap()
                .iter()
                .map(|path| path.as_ref())
                .collect::<Vec<_>>(),
            LucideIcon::ALL.map(LucideIcon::asset_path)
        );
    }
}
