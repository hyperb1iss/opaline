# Community Themes

Opaline ships with 34 community themes ported from popular editor and terminal color schemes.

## Catppuccin (4 variants)

A soothing pastel theme. Available in **Mocha**, **Macchiato**, **Frappé** (all dark), and **Latte** (light).

```rust
let mocha = opaline::load_by_name("catppuccin-mocha").unwrap();
let macchiato = opaline::load_by_name("catppuccin-macchiato").unwrap();
let frappe = opaline::load_by_name("catppuccin-frappe").unwrap();
let latte = opaline::load_by_name("catppuccin-latte").unwrap();
```

| Variant   | Accent                                                                        |
| --------- | ----------------------------------------------------------------------------- |
| Mocha     | <span class="color-swatch" style="background:#cba6f7"></span> Mauve `#cba6f7` |
| Macchiato | <span class="color-swatch" style="background:#c6a0f6"></span> Mauve `#c6a0f6` |
| Frappé    | <span class="color-swatch" style="background:#ca9ee6"></span> Mauve `#ca9ee6` |
| Latte     | <span class="color-swatch" style="background:#8839ef"></span> Mauve `#8839ef` |

## Rose Pine (3 variants)

All natural pine, faux fur, and a bit of soho. Three variants: **Rose Pine**, **Moon**, and **Dawn**.

```rust
let pine = opaline::load_by_name("rose-pine").unwrap();
let moon = opaline::load_by_name("rose-pine-moon").unwrap();
let dawn = opaline::load_by_name("rose-pine-dawn").unwrap();
```

| Variant   | Base                                                                    | Accent                                                                       |
| --------- | ----------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Rose Pine | <span class="color-swatch" style="background:#191724"></span> `#191724` | <span class="color-swatch" style="background:#c4a7e7"></span> Iris `#c4a7e7` |
| Moon      | <span class="color-swatch" style="background:#232136"></span> `#232136` | <span class="color-swatch" style="background:#c4a7e7"></span> Iris `#c4a7e7` |
| Dawn      | <span class="color-swatch" style="background:#faf4ed"></span> `#faf4ed` | <span class="color-swatch" style="background:#907aa9"></span> Iris `#907aa9` |

## Dracula

The classic dark theme with bold, high-contrast colors.

```rust
let theme = opaline::load_by_name("dracula").unwrap();
```

| Role       | Color                                                                          |
| ---------- | ------------------------------------------------------------------------------ |
| Primary    | <span class="color-swatch" style="background:#bd93f9"></span> Purple `#bd93f9` |
| Secondary  | <span class="color-swatch" style="background:#8be9fd"></span> Cyan `#8be9fd`   |
| Background | <span class="color-swatch" style="background:#282a36"></span> `#282a36`        |

## Nord

An arctic, north-bluish color palette. Clean and minimal.

```rust
let theme = opaline::load_by_name("nord").unwrap();
```

## Tokyo Night (3 variants)

Inspired by the lights of Downtown Tokyo. Three variants: **Tokyo Night**, **Storm**, and **Moon**.

```rust
let night = opaline::load_by_name("tokyo-night").unwrap();
let storm = opaline::load_by_name("tokyo-night-storm").unwrap();
let moon = opaline::load_by_name("tokyo-night-moon").unwrap();
```

Storm uses deeper blue backgrounds (`#24283b` vs `#1a1b26`) for sharper contrast. Moon uses brighter accents (`#c099ff`).

## Kanagawa (3 variants)

Inspired by Katsushika Hokusai's _The Great Wave off Kanagawa_. Rich, painterly colors on dark sumi ink backgrounds.

```rust
let wave = opaline::load_by_name("kanagawa-wave").unwrap();
let dragon = opaline::load_by_name("kanagawa-dragon").unwrap();
let lotus = opaline::load_by_name("kanagawa-lotus").unwrap();
```

| Variant | Accent                                                                             | Notes               |
| ------- | ---------------------------------------------------------------------------------- | ------------------- |
| Wave    | <span class="color-swatch" style="background:#957fb8"></span> Oni Violet `#957fb8` | Classic dark        |
| Dragon  | <span class="color-swatch" style="background:#8992a7"></span> Ash `#8992a7`        | Cooler, desaturated |
| Lotus   | <span class="color-swatch" style="background:#624c83"></span> Violet `#624c83`     | Light variant       |

## Gruvbox (2 variants)

Retro groove colors with warm tones.

```rust
let dark = opaline::load_by_name("gruvbox-dark").unwrap();
let light = opaline::load_by_name("gruvbox-light").unwrap();
```

## One Dark / One Light

Atom's iconic editor themes.

```rust
let dark = opaline::load_by_name("one-dark").unwrap();
let light = opaline::load_by_name("one-light").unwrap();
```

## Solarized (2 variants)

Ethan Schoonover's precision color scheme optimized for readability.

```rust
let dark = opaline::load_by_name("solarized-dark").unwrap();
let light = opaline::load_by_name("solarized-light").unwrap();
```

## Everforest (2 variants)

A comfortable green forest theme.

```rust
let dark = opaline::load_by_name("everforest-dark").unwrap();
let light = opaline::load_by_name("everforest-light").unwrap();
```

| Variant | Base                                                                    | Accent                                                                        |
| ------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| Dark    | <span class="color-swatch" style="background:#2d353b"></span> `#2d353b` | <span class="color-swatch" style="background:#a7c080"></span> Green `#a7c080` |
| Light   | <span class="color-swatch" style="background:#fdf6e3"></span> `#fdf6e3` | <span class="color-swatch" style="background:#6b7d00"></span> Green `#6b7d00` |

## Monokai Pro

The famous dark theme. Hot pink keywords, green strings.

```rust
let theme = opaline::load_by_name("monokai-pro").unwrap();
```

## GitHub (2 variants)

From the Primer design system.

```rust
let dimmed = opaline::load_by_name("github-dark-dimmed").unwrap();
let light = opaline::load_by_name("github-light").unwrap();
```

## Night Owl / Light Owl

Accessible themes by Sarah Drasner.

```rust
let dark = opaline::load_by_name("night-owl").unwrap();
let light = opaline::load_by_name("light-owl").unwrap();
```

## Ayu (3 variants)

Clean, focused themes with warm accent tones.

```rust
let dark = opaline::load_by_name("ayu-dark").unwrap();
let mirage = opaline::load_by_name("ayu-mirage").unwrap();
let light = opaline::load_by_name("ayu-light").unwrap();
```

## Flexoki (2 variants)

Inky themes by Steph Ango with warm earth tones.

```rust
let dark = opaline::load_by_name("flexoki-dark").unwrap();
let light = opaline::load_by_name("flexoki-light").unwrap();
```

## Palenight

A soft, purple-toned Material Theme variant.

```rust
let theme = opaline::load_by_name("palenight").unwrap();
```
