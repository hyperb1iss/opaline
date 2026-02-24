# SilkCircuit Themes

SilkCircuit is the signature theme family for Opaline — designed around the principle of **electric meets elegant**.

## Neon (Default)

The flagship dark theme. Electric purple and neon cyan on a deep, slightly purple-shifted dark background.

**Palette highlights:**

| Role | Color | Hex |
|------|-------|-----|
| Primary accent | <span class="color-swatch" style="background:#e135ff"></span> Electric Purple | `#e135ff` |
| Secondary accent | <span class="color-swatch" style="background:#80ffea"></span> Neon Cyan | `#80ffea` |
| Tertiary | <span class="color-swatch" style="background:#ff6ac1"></span> Coral | `#ff6ac1` |
| Success | <span class="color-swatch" style="background:#50fa7b"></span> Green | `#50fa7b` |
| Error | <span class="color-swatch" style="background:#ff6363"></span> Red | `#ff6363` |
| Warning | <span class="color-swatch" style="background:#f1fa8c"></span> Yellow | `#f1fa8c` |
| Background | <span class="color-swatch" style="background:#121218"></span> Deep Dark | `#121218` |

```rust
let theme = opaline::load_by_name("silkcircuit-neon").unwrap();
// or simply:
let theme = opaline::Theme::default();
```

## Soft

A muted, lower-contrast variant for extended coding sessions. Lavender and sage tones replace the electric palette.

```rust
let theme = opaline::load_by_name("silkcircuit-soft").unwrap();
```

## Glow

Hot pink meets warm amber. Higher saturation with a warm color temperature.

```rust
let theme = opaline::load_by_name("silkcircuit-glow").unwrap();
```

## Vibrant

Maximum contrast. Magenta primary with electric blue secondary — for when you want your terminal to pop.

```rust
let theme = opaline::load_by_name("silkcircuit-vibrant").unwrap();
```

## Dawn (Light)

The light variant of SilkCircuit. Deep purple accents on a lavender-white background. Designed for bright environments and daylight coding.

**Palette highlights:**

| Role | Color | Hex |
|------|-------|-----|
| Primary accent | <span class="color-swatch" style="background:#7e2bd5"></span> Deep Purple | `#7e2bd5` |
| Secondary accent | <span class="color-swatch" style="background:#007f8e"></span> Teal | `#007f8e` |
| Background | <span class="color-swatch" style="background:#faf8ff"></span> Lavender White | `#faf8ff` |
| Text | <span class="color-swatch" style="background:#2b2540"></span> Deep Indigo | `#2b2540` |

```rust
let theme = opaline::load_by_name("silkcircuit-dawn").unwrap();
assert!(theme.is_light());
```

## Design Principles

SilkCircuit follows these design rules:

1. **Purple-shifted darks** — Background hues lean slightly toward purple/blue rather than pure grey
2. **High-chroma accents** — Primary colors use near-maximum saturation
3. **Complementary pairs** — Primary and secondary accents sit across the color wheel
4. **Consistent luminance** — Text and UI elements maintain readable contrast ratios
5. **Gradient harmony** — Multi-stop gradients flow through analogous hues
