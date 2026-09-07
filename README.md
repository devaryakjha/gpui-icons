# gpui-icons

The complete Lucide icon set for GPUI: **1,776 canonical icons and 258 aliases**
from Lucide 1.33.0. Every icon uses the original upstream SVG, embedded in your
application. Native and WASM use the same assets without fetching anything at runtime.

This is a standalone icon library. You can use it with any GPUI application;
gpuicn is one consumer. It is not affiliated with Lucide.

## Use

Register the asset source before rendering icons:

```rust
use gpui::{Styled, px, rgb};
use gpui_icons::{LucideAssetSource, LucideIcon, lucide};

// On your Application builder:
// application.with_assets(LucideAssetSource).run(...);

let search = lucide(LucideIcon::Search).size(px(16.)).text_color(rgb(0x171717));
let folder = LucideIcon::Folder.element().size(px(24.)).text_color(rgb(0x171717));
let icon = LucideIcon::from_name("alarm-clock").unwrap();
```

Set `.text_color(...)` explicitly from your theme: GPUI does not paint an
SVG without it. Size icons at 16, 20, 24, or 32 pixels as appropriate. Give icon-only
buttons an accessible label on the button; a decorative SVG is not a control.

`LucideIcon::ALL` contains the full canonical set in name order. `from_name`
accepts canonical kebab-case names and upstream aliases, including deprecated
names such as `align-center`. Unknown names return `None`.

## Combine with your app's assets

`LucideAssetSource` owns `icons/lucide/`. It returns `Ok(None)` for unrelated
paths and an explicit error for an unknown Lucide icon. Delegate only when it
returns `None`:

```rust
use std::borrow::Cow;
use gpui::{AssetSource, Result, SharedString};
use gpui_icons::LucideAssetSource;

struct AppAssets<S> {
    application: S,
}

impl<S: AssetSource> AssetSource for AppAssets<S> {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        match LucideAssetSource.load(path)? {
            Some(asset) => Ok(Some(asset)),
            None => self.application.load(path),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        match path {
            "icons/lucide" | "icons/lucide/" => LucideAssetSource.list(path),
            _ => self.application.list(path),
        }
    }
}
```

## Reproducible source

GPUI is pinned to `59b2ebf10351b5c0b5cd4403f01ed0460eeec06d`. Use the same
revision in your application to avoid incompatible GPUI types.

The [release manifest](RELEASE-MANIFEST.json) records canonical names, aliases,
tags, categories, contributors, and hashes for every upstream SVG and metadata
file. Assets come from [Lucide 1.33.0](https://github.com/lucide-icons/lucide/tree/59978cecf84986af59f1f9f503bcebdc89c6d166).

To regenerate, run `python3 scripts/generate.py`. The script downloads the
pinned upstream archive and verifies its SHA-256 before reading it. You can
also pass a local copy of that archive as its first argument. Python and
rustfmt are the only generation tools; consumers do not run a build script.

Run `cargo test --lib` to check every icon's lookup, asset path, bytes, metadata
hash, aliases, and license against the pinned release. Run `cargo fmt --check`
and `cargo clippy --lib --tests -- -D warnings` before a release.

## License

The Rust code is MIT-licensed in [LICENSES/gpui-icons-MIT](LICENSES/gpui-icons-MIT).
The exact upstream [LICENSE](LICENSE) covers Lucide's ISC and Feather-derived
MIT terms. Package metadata declares `MIT AND ISC`.
