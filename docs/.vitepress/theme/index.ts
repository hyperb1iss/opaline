import DefaultTheme from 'vitepress/theme'
import type { Theme } from 'vitepress'
import './silkcircuit.css'

declare global {
  interface Window {
    renderMermaidNow?: () => Promise<void>
  }
}

export default {
  extends: DefaultTheme,
  setup() {
    if (typeof window !== 'undefined') {
      const isDarkMode = () =>
        document.documentElement.classList.contains('dark') ||
        document.documentElement.getAttribute('data-theme') === 'dark'

      // SilkCircuit Dawn (light)
      const lightThemeVariables = {
        background: '#faf8ff',
        primaryColor: '#7e2bd5',
        primaryBorderColor: '#5c1a9e',
        primaryTextColor: '#2b2540',
        lineColor: '#007f8e',
        secondaryColor: '#f1ecff',
        tertiaryColor: '#efeaff',
        mainBkg: '#9e4df3',
        secondBkg: '#f1ecff',
        tertiaryBkg: '#efeaff',
        clusterBkg: '#f1ecff',
        clusterBorder: '#7e2bd5',
        edgeLabelBackground: '#faf8ff',
        nodeTextColor: '#2b2540',
        titleColor: '#7e2bd5',
        fontSize: '16px',
        fontFamily: 'JetBrains Mono, Fira Code, SF Mono, monospace',
      }

      // SilkCircuit Neon (dark)
      const darkThemeVariables = {
        background: '#121218',
        primaryColor: '#e135ff',
        primaryBorderColor: '#bd93f9',
        primaryTextColor: '#f8f8f2',
        lineColor: '#80ffea',
        secondaryColor: '#1e1e28',
        tertiaryColor: '#181820',
        mainBkg: '#e135ff',
        secondBkg: '#1e1e28',
        tertiaryBkg: '#181820',
        clusterBkg: '#1e1e28',
        clusterBorder: '#80ffea',
        edgeLabelBackground: '#121218',
        nodeTextColor: '#f8f8f2',
        titleColor: '#80ffea',
        fontSize: '16px',
        fontFamily: 'JetBrains Mono, Fira Code, SF Mono, monospace',
      }

      const getMermaidConfig = () => ({
        startOnLoad: false,
        theme: isDarkMode() ? 'dark' : 'neutral',
        themeVariables: isDarkMode() ? darkThemeVariables : lightThemeVariables,
        securityLevel: 'loose' as const,
        flowchart: { htmlLabels: true, curve: 'basis' },
      })

      let mermaidLoadPromise: Promise<any> | null = null
      const ensureMermaid = async () => {
        if (!mermaidLoadPromise) {
          mermaidLoadPromise = import(/* @vite-ignore */ 'mermaid/dist/mermaid.esm.mjs').then(
            (mod) => mod.default ?? mod,
          )
        }
        return mermaidLoadPromise
      }

      const convertCodeFences = () => {
        let converted = 0

        // VitePress language-mermaid divs
        for (const wrap of Array.from(
          document.querySelectorAll<HTMLDivElement>('div.language-mermaid'),
        )) {
          const code = wrap.querySelector('code')
          const text = (code?.textContent ?? wrap.textContent ?? '').trim()
          if (!text) continue
          const container = document.createElement('div')
          container.className = 'mermaid-diagram'
          container.dataset.mermaidSource = text
          container.textContent = text
          wrap.replaceWith(container)
          converted++
        }

        // Pre elements with mermaid
        for (const pre of Array.from(document.querySelectorAll<HTMLPreElement>('pre'))) {
          const code = pre.querySelector('code')
          const isMermaid =
            pre.className.includes('language-mermaid') ||
            code?.className.includes('language-mermaid')
          if (!isMermaid) continue
          const text = (code?.textContent ?? pre.textContent ?? '').trim()
          if (!text) continue
          const container = document.createElement('div')
          container.className = 'mermaid-diagram'
          container.dataset.mermaidSource = text
          container.textContent = text
          pre.replaceWith(container)
          converted++
        }

        return converted
      }

      const resetExistingDiagrams = () => {
        let reset = 0
        for (const diagram of document.querySelectorAll<HTMLElement>('.mermaid-diagram')) {
          const source = diagram.dataset.mermaidSource
          if (!source) continue
          diagram.textContent = source
          reset++
        }
        return reset
      }

      const addZoomListeners = () => {
        setTimeout(() => {
          for (const diagram of document.querySelectorAll('.mermaid-diagram')) {
            const svg = diagram.querySelector('svg')
            if (!svg || svg.dataset.zoomEnabled) continue

            svg.dataset.zoomEnabled = 'true'
            svg.style.cursor = 'zoom-in'

            svg.addEventListener('click', () => {
              const modal = document.createElement('div')
              modal.className = 'mermaid-zoom-modal active'
              modal.appendChild(svg.cloneNode(true))

              modal.addEventListener('click', () => modal.remove())
              const handleEscape = (e: KeyboardEvent) => {
                if (e.key === 'Escape') {
                  modal.remove()
                  document.removeEventListener('keydown', handleEscape)
                }
              }
              document.addEventListener('keydown', handleEscape)
              document.body.appendChild(modal)
            })
          }
        }, 100)
      }

      const renderMermaid = async (force = false) => {
        const newlyConverted = convertCodeFences()
        const resetCount = force ? resetExistingDiagrams() : 0
        if (newlyConverted === 0 && resetCount === 0) return
        try {
          const mermaid = await ensureMermaid()
          mermaid.initialize(getMermaidConfig())
          await mermaid.run({ querySelector: '.mermaid-diagram' })
          addZoomListeners()
        } catch (err) {
          console.warn('Mermaid failed to render:', err)
        }
      }

      window.renderMermaidNow = () => renderMermaid(true)

      // Theme change debounce
      const scheduleThemeRender = (() => {
        let timer: number | null = null
        return () => {
          if (timer) window.clearTimeout(timer)
          timer = window.setTimeout(() => {
            timer = null
            renderMermaid(true)
          }, 120)
        }
      })()

      // Watch for theme changes
      const themeObserver = new MutationObserver((mutations) => {
        for (const mutation of mutations) {
          if (mutation.type === 'attributes') {
            scheduleThemeRender()
            break
          }
        }
      })
      themeObserver.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ['class', 'data-theme'],
      })

      // System preference changes
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      if (typeof mediaQuery.addEventListener === 'function') {
        mediaQuery.addEventListener('change', scheduleThemeRender)
      }

      // Initial + navigation renders
      const doRender = () => setTimeout(renderMermaid, 100)
      if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', doRender)
      } else {
        doRender()
      }
      window.addEventListener('vitepress:after-route-changed', doRender)
    }
  },
} satisfies Theme
