# Theme Gallery

Opaline ships with **39 builtin themes** covering a wide range of aesthetics, from electric neon to cozy forest greens.

## Complete Catalog

| Theme                    | Variant | Author            | Accent                                                                 |
| ------------------------ | ------- | ----------------- | ---------------------------------------------------------------------- |
| **SilkCircuit Neon**     | Dark    | hyperb1iss        | <span class="color-swatch" style="background:#e135ff"></span>`#e135ff` |
| **SilkCircuit Soft**     | Dark    | hyperb1iss        | <span class="color-swatch" style="background:#e892ff"></span>`#e892ff` |
| **SilkCircuit Glow**     | Dark    | hyperb1iss        | <span class="color-swatch" style="background:#ff00ff"></span>`#ff00ff` |
| **SilkCircuit Vibrant**  | Dark    | hyperb1iss        | <span class="color-swatch" style="background:#ff00ff"></span>`#ff00ff` |
| **SilkCircuit Dawn**     | Light   | hyperb1iss        | <span class="color-swatch" style="background:#7e2bd5"></span>`#7e2bd5` |
| **Catppuccin Mocha**     | Dark    | Catppuccin        | <span class="color-swatch" style="background:#cba6f7"></span>`#cba6f7` |
| **Catppuccin Macchiato** | Dark    | Catppuccin        | <span class="color-swatch" style="background:#c6a0f6"></span>`#c6a0f6` |
| **Catppuccin Frappé**    | Dark    | Catppuccin        | <span class="color-swatch" style="background:#ca9ee6"></span>`#ca9ee6` |
| **Catppuccin Latte**     | Light   | Catppuccin        | <span class="color-swatch" style="background:#8839ef"></span>`#8839ef` |
| **Dracula**              | Dark    | Dracula Theme     | <span class="color-swatch" style="background:#bd93f9"></span>`#bd93f9` |
| **Nord**                 | Dark    | Arctic Ice Studio | <span class="color-swatch" style="background:#88c0d0"></span>`#88c0d0` |
| **Tokyo Night**          | Dark    | enkia             | <span class="color-swatch" style="background:#bb9af7"></span>`#bb9af7` |
| **Tokyo Night Storm**    | Dark    | enkia             | <span class="color-swatch" style="background:#bb9af7"></span>`#bb9af7` |
| **Tokyo Night Moon**     | Dark    | folke             | <span class="color-swatch" style="background:#c099ff"></span>`#c099ff` |
| **Rose Pine**            | Dark    | Rose Pine         | <span class="color-swatch" style="background:#c4a7e7"></span>`#c4a7e7` |
| **Rose Pine Moon**       | Dark    | Rose Pine         | <span class="color-swatch" style="background:#c4a7e7"></span>`#c4a7e7` |
| **Rose Pine Dawn**       | Light   | Rose Pine         | <span class="color-swatch" style="background:#907aa9"></span>`#907aa9` |
| **Kanagawa Wave**        | Dark    | rebelot           | <span class="color-swatch" style="background:#957fb8"></span>`#957fb8` |
| **Kanagawa Dragon**      | Dark    | rebelot           | <span class="color-swatch" style="background:#8992a7"></span>`#8992a7` |
| **Kanagawa Lotus**       | Light   | rebelot           | <span class="color-swatch" style="background:#624c83"></span>`#624c83` |
| **Everforest Dark**      | Dark    | sainnhe           | <span class="color-swatch" style="background:#a7c080"></span>`#a7c080` |
| **Everforest Light**     | Light   | sainnhe           | <span class="color-swatch" style="background:#6b7d00"></span>`#6b7d00` |
| **Gruvbox Dark**         | Dark    | morhetz           | <span class="color-swatch" style="background:#fe8019"></span>`#fe8019` |
| **Gruvbox Light**        | Light   | morhetz           | <span class="color-swatch" style="background:#d65d0e"></span>`#d65d0e` |
| **Solarized Dark**       | Dark    | Ethan Schoonover  | <span class="color-swatch" style="background:#268bd2"></span>`#268bd2` |
| **Solarized Light**      | Light   | Ethan Schoonover  | <span class="color-swatch" style="background:#268bd2"></span>`#268bd2` |
| **One Dark**             | Dark    | Atom              | <span class="color-swatch" style="background:#61afef"></span>`#61afef` |
| **One Light**            | Light   | Atom              | <span class="color-swatch" style="background:#4078f2"></span>`#4078f2` |
| **Monokai Pro**          | Dark    | Monokai           | <span class="color-swatch" style="background:#ff6188"></span>`#ff6188` |
| **GitHub Dark Dimmed**   | Dark    | Primer            | <span class="color-swatch" style="background:#539bf5"></span>`#539bf5` |
| **GitHub Light**         | Light   | Primer            | <span class="color-swatch" style="background:#0969da"></span>`#0969da` |
| **Night Owl**            | Dark    | Sarah Drasner     | <span class="color-swatch" style="background:#c792ea"></span>`#c792ea` |
| **Light Owl**            | Light   | Sarah Drasner     | <span class="color-swatch" style="background:#994cc3"></span>`#994cc3` |
| **Ayu Dark**             | Dark    | Ayu Theme         | <span class="color-swatch" style="background:#e6b450"></span>`#e6b450` |
| **Ayu Mirage**           | Dark    | Ayu Theme         | <span class="color-swatch" style="background:#ffcc66"></span>`#ffcc66` |
| **Ayu Light**            | Light   | Ayu Theme         | <span class="color-swatch" style="background:#f29718"></span>`#f29718` |
| **Flexoki Dark**         | Dark    | Steph Ango        | <span class="color-swatch" style="background:#da702c"></span>`#da702c` |
| **Flexoki Light**        | Light   | Steph Ango        | <span class="color-swatch" style="background:#bc5215"></span>`#bc5215` |
| **Palenight**            | Dark    | Material Theme    | <span class="color-swatch" style="background:#c792ea"></span>`#c792ea` |

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
    let author = if info.author.is_empty() { "unknown" } else { &info.author };
    println!("{} by {} ({})",
        info.display_name,
        author,
        if info.builtin { "builtin" } else { "custom" }
    );
}
```

## Theme Families

### SilkCircuit (5 variants)

The signature Opaline theme family. Electric meets elegant.

- **Neon**: Electric purple + neon cyan on deep dark
- **Soft**: Muted lavender for extended sessions
- **Glow**: Hot pink + warm tones
- **Vibrant**: High-contrast magenta + electric blue
- **Dawn**: Light variant with purple accents

### Catppuccin (4 variants)

The soothing pastel theme for the high-spirited.

- **Mocha**: Warm dark with lavender accents
- **Macchiato**: Deep dark with lavender accents
- **Frappé**: Cool medium-dark with mauve accents
- **Latte**: Creamy light with mauve accents

### Rose Pine (3 variants)

All natural pine, faux fur, and a bit of soho.

- **Rose Pine**: Dark base with iris and foam accents
- **Moon**: Darker, cooler variant
- **Dawn**: Light variant with warm undertones

### Tokyo Night (3 variants)

Inspired by the lights of Downtown Tokyo.

- **Tokyo Night**: Clean dark blue tones
- **Storm**: Deeper blues with sharper contrast
- **Moon**: Moonlit variation with brighter accents

### Kanagawa (3 variants)

Inspired by Katsushika Hokusai's _The Great Wave off Kanagawa_.

- **Wave**: Rich painterly colors on sumi ink
- **Dragon**: Cooler, desaturated variant
- **Lotus**: Light variant with muted violet accents

### Everforest (2 variants)

A comfortable green forest theme.

- **Dark**: Warm green palette for long sessions
- **Light**: Bright green palette for daylight

### Ayu (3 variants)

Clean, focused themes with warm accent tones.

- **Dark**: Warm yellow accents on deep dark
- **Mirage**: Slightly lighter with golden tones
- **Light**: Bright with amber accents

### Gruvbox (2 variants)

Retro groove colors designed for long sessions.

- **Dark**: Warm orange accents on dark background
- **Light**: Bright with strong orange tones

### Solarized (2 variants)

Precision colors optimized for readability.

- **Dark**: Blue-accented dark theme
- **Light**: The classic light theme

### One (2 variants)

Atom's iconic editor themes.

- **Dark**: Blue functions, purple keywords
- **Light**: Bright with blue accents

### GitHub (2 variants)

GitHub's editor themes from the Primer design system.

- **Dark Dimmed**: Soft blue accents on muted dark
- **Light**: Clean blue accents on white

### Night Owl / Light Owl (2 variants)

Accessible themes by Sarah Drasner.

- **Night Owl**: Purple accents on deep blue
- **Light Owl**: Purple accents on clean white

### Flexoki (2 variants)

Inky themes by Steph Ango with warm earth tones.

- **Dark**: Burnt orange accents
- **Light**: Deep orange accents

### Standalone Themes

- **Dracula**: The classic purple-accented dark theme
- **Nord**: Arctic, north-bluish palette
- **Monokai Pro**: Hot pink accents on dark charcoal
- **Palenight**: Soft purple Material Theme variant
