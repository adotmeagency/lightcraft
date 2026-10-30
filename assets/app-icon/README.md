# LightStudio app icons

All app icon formats use the same original Adotme mark, with only the color
changed across the suite. Windows, macOS, Linux and SVG assets are included.
`adotme-mark.rgba` is a 128x128 RGBA texture for the UI, cached once per context.
See ADOTME-BRANDING.txt. Do not regenerate from the old upstream artwork.

Rebuild the existing desktop package with:

```powershell
cargo run --release --locked -p lightcraft
```

Internal package names, file formats and settings paths remain compatible.
