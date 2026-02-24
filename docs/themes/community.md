# Community Themes

Opaline ships with 15 community themes ported from popular editor and terminal color schemes.

## Catppuccin

A soothing pastel theme. Available in **Mocha** (dark) and **Latte** (light).

```rust
let mocha = opaline::load_by_name("catppuccin-mocha").unwrap();
let latte = opaline::load_by_name("catppuccin-latte").unwrap();
```

| Mocha Accent | Latte Accent |
|--------------|--------------|
| <span class="color-swatch" style="background:#cba6f7"></span> Mauve `#cba6f7` | <span class="color-swatch" style="background:#8839ef"></span> Mauve `#8839ef` |

## Rose Pine

All natural pine, faux fur, and a bit of soho. Three variants: **Rose Pine**, **Moon**, and **Dawn**.

```rust
let pine = opaline::load_by_name("rose-pine").unwrap();
let moon = opaline::load_by_name("rose-pine-moon").unwrap();
let dawn = opaline::load_by_name("rose-pine-dawn").unwrap();
```

| Variant | Base | Accent |
|---------|------|--------|
| Rose Pine | <span class="color-swatch" style="background:#191724"></span> `#191724` | <span class="color-swatch" style="background:#c4a7e7"></span> Iris `#c4a7e7` |
| Moon | <span class="color-swatch" style="background:#232136"></span> `#232136` | <span class="color-swatch" style="background:#c4a7e7"></span> Iris `#c4a7e7` |
| Dawn | <span class="color-swatch" style="background:#faf4ed"></span> `#faf4ed` | <span class="color-swatch" style="background:#907aa9"></span> Iris `#907aa9` |

## Dracula

The classic dark theme with bold, high-contrast colors.

```rust
let theme = opaline::load_by_name("dracula").unwrap();
```

| Role | Color |
|------|-------|
| Primary | <span class="color-swatch" style="background:#bd93f9"></span> Purple `#bd93f9` |
| Secondary | <span class="color-swatch" style="background:#8be9fd"></span> Cyan `#8be9fd` |
| Background | <span class="color-swatch" style="background:#282a36"></span> `#282a36` |

## Nord

An arctic, north-bluish color palette. Clean and minimal.

```rust
let theme = opaline::load_by_name("nord").unwrap();
```

## Tokyo Night

Inspired by the lights of Downtown Tokyo. Two variants: **Tokyo Night** and **Storm**.

```rust
let night = opaline::load_by_name("tokyo-night").unwrap();
let storm = opaline::load_by_name("tokyo-night-storm").unwrap();
```

Storm uses deeper blue backgrounds (`#24283b` vs `#1a1b26`) for sharper contrast.

## Kanagawa Wave

Inspired by Katsushika Hokusai's *The Great Wave off Kanagawa*. Rich, painterly colors on dark sumi ink backgrounds.

```rust
let theme = opaline::load_by_name("kanagawa-wave").unwrap();
```

| Role | Color |
|------|-------|
| Primary | <span class="color-swatch" style="background:#957fb8"></span> Oni Violet `#957fb8` |
| Secondary | <span class="color-swatch" style="background:#7e9cd8"></span> Crystal Blue `#7e9cd8` |
| Background | <span class="color-swatch" style="background:#1f1f28"></span> Sumi Ink `#1f1f28` |

## Gruvbox Dark

Retro groove colors with warm orange accents on a dark background.

```rust
let theme = opaline::load_by_name("gruvbox-dark").unwrap();
```

## One Dark

Atom's iconic dark theme. Purple keywords, blue functions, green strings.

```rust
let theme = opaline::load_by_name("one-dark").unwrap();
```

## Solarized Light

Ethan Schoonover's precision color scheme optimized for readability.

```rust
let theme = opaline::load_by_name("solarized-light").unwrap();
```

## Everforest

A comfortable green forest theme. Two variants: **Dark** and **Light**.

```rust
let dark = opaline::load_by_name("everforest-dark").unwrap();
let light = opaline::load_by_name("everforest-light").unwrap();
```

| Variant | Base | Accent |
|---------|------|--------|
| Dark | <span class="color-swatch" style="background:#2d353b"></span> `#2d353b` | <span class="color-swatch" style="background:#a7c080"></span> Green `#a7c080` |
| Light | <span class="color-swatch" style="background:#fdf6e3"></span> `#fdf6e3` | <span class="color-swatch" style="background:#8da101"></span> Green `#8da101` |
