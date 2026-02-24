---
layout: home

hero:
  name: Opaline
  text: Theme Engine for Ratatui
  tagline: Token-based themes with 20 builtins, gradients, and deep Ratatui integration
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started/
    - theme: alt
      text: View on GitHub
      link: https://github.com/hyperb1iss/opaline

features:
  - icon: "\U0001F3A8"
    title: Token-Based Design
    details: Three-layer resolution — palette colors, semantic tokens, composed styles — keeps themes consistent and maintainable
  - icon: "\U0001F308"
    title: 20 Builtin Themes
    details: SilkCircuit, Catppuccin, Dracula, Nord, Tokyo Night, Rose Pine, Kanagawa, Gruvbox, and more
  - icon: "\U0001F4A0"
    title: Smooth Gradients
    details: Multi-stop color gradients with linear interpolation — perfect for progress bars, status indicators, and decorative elements
  - icon: "\u26A1"
    title: Deep Ratatui Integration
    details: "From impls, Styled trait, gradient spans — Opaline types slot directly into your Ratatui render pipeline"
  - icon: "\U0001F527"
    title: TOML-Driven
    details: Define themes in clean TOML files. Palette, tokens, styles, and gradients all declaratively configured
  - icon: "\U0001F6E1\uFE0F"
    title: Strict by Default
    details: Cycle detection, strict resolution, compile-time theme embedding — errors surface early, not at runtime
---

<style>
:root {
  --vp-home-hero-name-color: transparent;
  --vp-home-hero-name-background: linear-gradient(135deg, #e135ff 0%, #80ffea 100%);
}

.dark {
  --vp-home-hero-image-background-image: linear-gradient(135deg, rgba(225, 53, 255, 0.2) 0%, rgba(128, 255, 234, 0.2) 100%);
  --vp-home-hero-image-filter: blur(56px);
}
</style>
