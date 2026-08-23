# gpui-icons

`gpui-icons` is an unofficial, asset-backed GPUI port of a small Lucide allow-list. It ships only the three icons needed by imajha/ui v0.1:

- `LucideIcon::Check`
- `LucideIcon::Minus`
- `LucideIcon::X`

It is not a full Lucide pack, a brand-icon pack, or a `gpui-component` adapter.

## Pins and provenance

This release pins [Lucide 1.33.0](https://github.com/lucide-icons/lucide/tree/59978cecf84986af59f1f9f503bcebdc89c6d166) at `59978cecf84986af59f1f9f503bcebdc89c6d166` and GPUI at `59b2ebf10351b5c0b5cd4403f01ed0460eeec06d`.

[`RELEASE-MANIFEST.json`](RELEASE-MANIFEST.json) records each upstream SVG and JSON source, hashes, contributors, aliases, deprecation state, and Feather-derived status. The SVGs are embedded with `include_bytes!`, so native and WASM use the same bytes without a network fetch.

## Use

Compose `LucideAssetSource` into the application asset source. It serves `icons/lucide/` and delegates unrelated paths to your application source. An unknown path inside `icons/lucide/` returns an explicit not-found error.

```rust
use std::borrow::Cow;

use gpui::{App, AssetSource, Result, SharedString, Styled, px};
use gpui_icons::{LucideAssetSource, LucideIcon, lucide};

struct AppAssets<S> {
    application: S,
    lucide: LucideAssetSource,
}

impl<S: AssetSource> AssetSource for AppAssets<S> {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        match self.lucide.load(path)? {
            Some(asset) => Ok(Some(asset)),
            None => self.application.load(path),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let lucide_assets = self.lucide.list(path)?;
        if lucide_assets.is_empty() {
            self.application.list(path)
        } else {
            Ok(lucide_assets)
        }
    }
}

let close = lucide(LucideIcon::X).size(px(16.));
let check = LucideIcon::Check.element().size(px(12.));
```

`Svg` inherits surrounding text color. Apply `.text_color(...)` for an explicit monochrome color. Build the `App` with `App::with_assets(AppAssets { ... })` before rendering these elements.

## License

The crate code is MIT-licensed in [`LICENSES/gpui-icons-MIT`](LICENSES/gpui-icons-MIT). The copied Lucide assets remain under Lucide's exact upstream ISC license, including the Feather-derived MIT terms, in [`LICENSE`](LICENSE). The package metadata therefore declares `MIT AND ISC`.

Lucide is a separate project. This crate is not affiliated with or endorsed by Lucide or its contributors.
