# Theme Gallery

Opaline ships with **20 builtin themes** covering a wide range of aesthetics — from electric neon to cozy forest greens.

## At a Glance

| Theme | Variant | Author | Accent |
|-------|---------|--------|--------|
| **SilkCircuit Neon** | Dark | hyperb1iss | <span class="color-swatch" style="background:#e135ff"></span>`#e135ff` |
| **SilkCircuit Soft** | Dark | hyperb1iss | <span class="color-swatch" style="background:#c49bff"></span>`#c49bff` |
| **SilkCircuit Glow** | Dark | hyperb1iss | <span class="color-swatch" style="background:#ff79c6"></span>`#ff79c6` |
| **SilkCircuit Vibrant** | Dark | hyperb1iss | <span class="color-swatch" style="background:#ff2d78"></span>`#ff2d78` |
| **SilkCircuit Dawn** | Light | hyperb1iss | <span class="color-swatch" style="background:#7e2bd5"></span>`#7e2bd5` |
| **Catppuccin Mocha** | Dark | catppuccin | <span class="color-swatch" style="background:#cba6f7"></span>`#cba6f7` |
| **Catppuccin Latte** | Light | catppuccin | <span class="color-swatch" style="background:#8839ef"></span>`#8839ef` |
| **Dracula** | Dark | dracula | <span class="color-swatch" style="background:#bd93f9"></span>`#bd93f9` |
| **Nord** | Dark | arcticicestudio | <span class="color-swatch" style="background:#88c0d0"></span>`#88c0d0` |
| **Tokyo Night** | Dark | enkia | <span class="color-swatch" style="background:#bb9af7"></span>`#bb9af7` |
| **Tokyo Night Storm** | Dark | enkia | <span class="color-swatch" style="background:#bb9af7"></span>`#bb9af7` |
| **Gruvbox Dark** | Dark | morhetz | <span class="color-swatch" style="background:#fe8019"></span>`#fe8019` |
| **One Dark** | Dark | atom | <span class="color-swatch" style="background:#c678dd"></span>`#c678dd` |
| **Solarized Light** | Light | ethanschoonover | <span class="color-swatch" style="background:#268bd2"></span>`#268bd2` |
| **Rose Pine** | Dark | rose-pine | <span class="color-swatch" style="background:#c4a7e7"></span>`#c4a7e7` |
| **Rose Pine Moon** | Dark | rose-pine | <span class="color-swatch" style="background:#c4a7e7"></span>`#c4a7e7` |
| **Rose Pine Dawn** | Light | rose-pine | <span class="color-swatch" style="background:#907aa9"></span>`#907aa9` |
| **Kanagawa Wave** | Dark | rebelot | <span class="color-swatch" style="background:#957fb8"></span>`#957fb8` |
| **Everforest Dark** | Dark | sainnhe | <span class="color-swatch" style="background:#a7c080"></span>`#a7c080` |
| **Everforest Light** | Light | sainnhe | <span class="color-swatch" style="background:#8da101"></span>`#8da101` |

## Loading a Theme

```rust
// By ID (kebab-case filename)
let theme = opaline::load_by_name("catppuccin-mocha").unwrap();
let theme = opaline::load_by_name("rose-pine").unwrap();
let theme = opaline::load_by_name("tokyo-night-storm").unwrap();

// "default" is an alias for SilkCircuit Neon
let theme = opaline::load_by_name("default").unwrap();
```

## Listing Themes

```rust
// Get all builtin theme metadata
let themes = opaline::list_available_themes();
for info in &themes {
    println!("{} by {} ({})",
        info.display_name,
        info.author.as_deref().unwrap_or("unknown"),
        if info.builtin { "builtin" } else { "custom" }
    );
}
```

## Theme Families

### SilkCircuit (5 variants)

The signature Opaline theme family. Electric meets elegant.

- **Neon** — Electric purple + neon cyan on deep dark
- **Soft** — Muted lavender for extended sessions
- **Glow** — Hot pink + warm tones
- **Vibrant** — High-contrast magenta + electric blue
- **Dawn** — Light variant with purple accents

### Catppuccin (2 variants)

The soothing pastel theme for the high-spirited.

- **Mocha** — Warm dark with lavender accents
- **Latte** — Creamy light with mauve accents

### Rose Pine (3 variants)

All natural pine, faux fur, and a bit of soho.

- **Rose Pine** — Dark base with iris and foam accents
- **Moon** — Darker, cooler variant
- **Dawn** — Light variant with warm undertones

### Tokyo Night (2 variants)

Inspired by the lights of Downtown Tokyo.

- **Tokyo Night** — Clean dark blue tones
- **Storm** — Deeper blues with sharper contrast

### Everforest (2 variants)

A comfortable green forest theme.

- **Dark** — Warm green palette for long sessions
- **Light** — Bright green palette for daylight

### Standalone Themes

- **Dracula** — The classic dark theme
- **Nord** — Arctic, north-bluish palette
- **Gruvbox Dark** — Retro groove with warm orange
- **One Dark** — Atom's iconic dark theme
- **Solarized Light** — Precision colors for readability
- **Kanagawa Wave** — Inspired by The Great Wave off Kanagawa
