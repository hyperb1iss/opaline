---
layout: home

hero:
  name: Opaline
  text: Theme Engine for Rust
  tagline: Token-based themes with 39 builtins, gradients, and adapters for ratatui, egui, crossterm, syntect, and more
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
    details: Three-layer resolution pipeline. Palette colors, semantic tokens, composed styles. Consistent and maintainable by design.
  - icon: "\U0001F308"
    title: 39 Builtin Themes
    details: SilkCircuit, Catppuccin, GitHub, Monokai Pro, Ayu, Night Owl, Flexoki, Palenight, Dracula, Nord, Rose Pine, Gruvbox, Solarized, Tokyo Night, Kanagawa, Everforest, One
  - icon: "\U0001F4A0"
    title: Smooth Gradients
    details: Multi-stop color gradients with linear interpolation. Perfect for progress bars, status indicators, and decorative elements.
  - icon: "\u26A1"
    title: Multi-Framework Adapters
    details: "First-class adapters for ratatui, egui, crossterm, owo-colors, syntect, and CSS. One theme, every target."
  - icon: "\U0001F527"
    title: TOML-Driven
    details: Define themes in clean TOML files. Palette, tokens, styles, and gradients all declaratively configured.
  - icon: "\U0001F6E1\uFE0F"
    title: Strict by Default
    details: Cycle detection, strict resolution, compile-time theme embedding. Errors surface early, not at runtime.
---

<style>
:root {
  --vp-home-hero-name-color: transparent;
  --vp-home-hero-name-background: linear-gradient(135deg, #e135ff 0%, #ff6ac1 40%, #80ffea 100%);
}

.dark {
  --vp-home-hero-image-background-image: linear-gradient(135deg, rgba(225, 53, 255, 0.25) 0%, rgba(255, 106, 193, 0.15) 50%, rgba(128, 255, 234, 0.25) 100%);
  --vp-home-hero-image-filter: blur(72px);
}

:root:not(.dark) {
  --vp-home-hero-name-background: linear-gradient(135deg, #7e2bd5 0%, #007f8e 100%);
  --vp-home-hero-image-background-image: linear-gradient(135deg, rgba(126, 43, 213, 0.15) 0%, rgba(0, 127, 142, 0.15) 100%);
  --vp-home-hero-image-filter: blur(72px);
}

@keyframes hero-glow {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

.VPHero .image-bg {
  animation: hero-glow 4s ease-in-out infinite;
}
</style>
