import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Opaline',
  description: 'A token-based theme engine for Ratatui TUI applications',
  base: '/opaline/',

  head: [
    ['meta', { name: 'theme-color', content: '#e135ff' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Opaline — Theme Engine for Ratatui' }],
    [
      'meta',
      {
        property: 'og:description',
        content: 'Token-based theme engine with 20 builtin themes, gradients, and deep Ratatui integration',
      },
    ],
  ],

  themeConfig: {
    nav: [
      { text: 'Guide', link: '/getting-started/' },
      { text: 'Themes', link: '/themes/' },
      {
        text: 'Reference',
        items: [
          { text: 'API', link: '/reference/api' },
          { text: 'Feature Flags', link: '/reference/features' },
          { text: 'Token Contract', link: '/reference/tokens' },
          { text: 'Errors', link: '/reference/errors' },
        ],
      },
    ],

    sidebar: {
      '/getting-started/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Introduction', link: '/getting-started/' },
            { text: 'Installation', link: '/getting-started/installation' },
            { text: 'Quick Start', link: '/getting-started/quick-start' },
          ],
        },
      ],
      '/guide/': [
        {
          text: 'Guide',
          items: [
            { text: 'Theme System', link: '/guide/themes' },
            { text: 'Tokens', link: '/guide/tokens' },
            { text: 'Styles & Modifiers', link: '/guide/styles' },
            { text: 'Gradients', link: '/guide/gradients' },
            { text: 'Ratatui Adapter', link: '/guide/ratatui' },
            { text: 'CLI Adapter', link: '/guide/cli' },
            { text: 'ThemeBuilder', link: '/guide/builder' },
            { text: 'Custom Themes', link: '/guide/custom-themes' },
          ],
        },
      ],
      '/themes/': [
        {
          text: 'Theme Gallery',
          items: [
            { text: 'Overview', link: '/themes/' },
            { text: 'SilkCircuit', link: '/themes/silkcircuit' },
            { text: 'Community Themes', link: '/themes/community' },
          ],
        },
      ],
      '/reference/': [
        {
          text: 'Reference',
          items: [
            { text: 'API', link: '/reference/api' },
            { text: 'Feature Flags', link: '/reference/features' },
            { text: 'Token Contract', link: '/reference/tokens' },
            { text: 'Errors', link: '/reference/errors' },
          ],
        },
      ],
    },

    socialLinks: [{ icon: 'github', link: 'https://github.com/hyperb1iss/opaline' }],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright \u00a9 2025 Stefanie Jane',
    },

    search: {
      provider: 'local',
    },
  },

  markdown: {
    theme: {
      light: 'github-light',
      dark: 'one-dark-pro',
    },
  },
})
