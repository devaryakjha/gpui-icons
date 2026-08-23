//! A small, embedded Lucide icon pack for GPUI.
//!
//! `gpui-icons` is an unofficial GPUI port of the pinned Lucide 1.33.0 assets
//! listed in [`RELEASE_MANIFEST`]. It intentionally contains only the icons
//! needed by the imajha/ui v0.1 Button, Checkbox, and Dialog slice.

use std::{borrow::Cow, io};

use gpui::{AssetSource, Result, SharedString, Svg, svg};

/// The asset namespace served by [`LucideAssetSource`].
pub const LUCIDE_ASSET_NAMESPACE: &str = "icons/lucide/";

/// The release provenance manifest bundled with this crate.
pub const RELEASE_MANIFEST: &str = include_str!("../RELEASE-MANIFEST.json");

/// An allow-listed Lucide icon.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LucideIcon {
    /// The Lucide `check` icon.
    Check,
    /// The Lucide `minus` icon.
    Minus,
    /// The Lucide `x` icon.
    X,
}

impl LucideIcon {
    /// Every canonical icon in this release, in release-manifest order.
    pub const ALL: [Self; 3] = [Self::Check, Self::Minus, Self::X];

    /// Returns the pinned Lucide canonical name.
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::Check => "check",
            Self::Minus => "minus",
            Self::X => "x",
        }
    }

    /// Returns this icon's stable, namespaced asset path.
    pub const fn asset_path(self) -> &'static str {
        match self {
            Self::Check => "icons/lucide/check.svg",
            Self::Minus => "icons/lucide/minus.svg",
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
            "icons/lucide/check.svg" => {
                Some(include_bytes!("../assets/lucide/check.svg").as_slice())
            }
            "icons/lucide/minus.svg" => {
                Some(include_bytes!("../assets/lucide/minus.svg").as_slice())
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
    fn release_contains_only_the_v01_allow_list() {
        assert_eq!(
            LucideIcon::ALL,
            [LucideIcon::Check, LucideIcon::Minus, LucideIcon::X]
        );
    }

    #[test]
    fn canonical_names_and_asset_paths_are_stable() {
        assert_eq!(
            LucideIcon::ALL.map(|icon| (icon.canonical_name(), icon.asset_path())),
            [
                ("check", "icons/lucide/check.svg"),
                ("minus", "icons/lucide/minus.svg"),
                ("x", "icons/lucide/x.svg"),
            ]
        );
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
            let (svg, metadata) = match icon {
                LucideIcon::Check => (
                    include_bytes!("../assets/lucide/check.svg").as_slice(),
                    include_bytes!("../assets/lucide/check.json").as_slice(),
                ),
                LucideIcon::Minus => (
                    include_bytes!("../assets/lucide/minus.svg").as_slice(),
                    include_bytes!("../assets/lucide/minus.json").as_slice(),
                ),
                LucideIcon::X => (
                    include_bytes!("../assets/lucide/x.svg").as_slice(),
                    include_bytes!("../assets/lucide/x.json").as_slice(),
                ),
            };

            assert_eq!(sha256(svg), entry["svg_sha256"]);
            assert_eq!(sha256(metadata), entry["json_sha256"]);
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
        assert!(
            LucideAssetSource
                .load("icons/lucide/chevron-down.svg")
                .is_err()
        );
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
            [
                "icons/lucide/check.svg",
                "icons/lucide/minus.svg",
                "icons/lucide/x.svg",
            ]
        );
    }
}
