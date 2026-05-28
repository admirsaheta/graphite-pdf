# GraphitePDF Design Palette

## Brand Direction

GraphitePDF should feel:

- precise
- industrial
- trustworthy
- systems-oriented
- modern open source

The logo language already defines the brand well:

- layered graphite grays communicate structure, durability, and calm technical confidence
- a rust-orange core signals energy, compilation, heat, and affinity with the Rust ecosystem
- the geometric mark feels engineered rather than decorative
- the wordmark in `Space Grotesk` keeps the identity contemporary and developer-friendly

This is not a soft consumer brand. It should feel like a high-quality Rust toolchain component: sharp, minimal, tactile, and reliable.

## Typography

- Wordmark: `Space Grotesk`
- UI fallback stack: `Space Grotesk, Inter, ui-sans-serif, system-ui, -apple-system, Segoe UI, sans-serif`
- Code stack: `JetBrains Mono, SFMono-Regular, Menlo, Monaco, Consolas, Liberation Mono, monospace`

### Type Character

- Use `Space Grotesk` for headlines, product branding, landing pages, and key navigation
- Prefer medium spacing and clean hierarchy over dense, decorative layouts
- Keep copy compact, technical, and confident
- Favor lowercase product naming where appropriate: `graphitepdf`

## Core Palette

These values are derived directly from the logo artwork.

### Primaries

| Name | Hex | Use |
| --- | --- | --- |
| Graphite 950 | `#060604` | outlines, deep borders, dark UI anchors |
| Graphite 900 | `#1E1E1C` | deepest dark neutral, footer/background contrast |
| Graphite 850 | `#2E2E2C` | primary dark neutral, tagline, dark text |
| Graphite 700 | `#3A3A38` | structural surfaces, strong neutral fills |
| Graphite 600 | `#606060` | mid-tone metallic neutral |
| Graphite 500 | `#6E6E6C` | secondary surfaces, dividers, UI framing |
| Mist 300 | `#C4C4C0` | light wordmark tone, muted foreground on dark backgrounds |

### Rust Core

| Name | Hex | Use |
| --- | --- | --- |
| Rust Ember 500 | `#D4581A` | core brand accent, `pdf` wordmark emphasis |
| Rust Glow 400 | `#F58040` | highlight accent, hover glow, warm highlights |
| Rust Ember 600 | `#C84C14` | primary action accent |
| Rust Oxide 700 | `#7C2806` | pressed states, deep accent contrast |

## Secondary Colors

These should support the identity without competing with the graphite and rust system.

| Name | Hex | Use |
| --- | --- | --- |
| Steel 400 | `#585856` | icons, inactive controls, thin strokes |
| Steel 500 | `#666664` | secondary borders, subdued UI emphasis |
| Alloy 500 | `#6A6A68` | metallic highlight within the emblem language |
| Slate Glow | `rgba(255,255,255,0.18)` | top-edge highlights on dark surfaces |
| Shadow Ink | `rgba(0,0,0,0.45)` | inner shadow lines and depth accents |

## Accent System

### Primary Accent

- `#D4581A`
- Use for calls to action, selected states, important highlights, and the `pdf` emphasis in the wordmark

### Warm Highlight Accent

- `#F58040`
- Use sparingly for hover states, glow effects, diagrams, active indicators, and highlighted nodes

### Dark Accent

- `#7C2806`
- Use for pressed states, deeper gradient stops, or subtle dramatic contrast in dark compositions

## Recommended UI Usage

### Backgrounds

- Default page background: `#111110` to `#1E1E1C`
- Elevated surfaces: `#2E2E2C` or `#3A3A38`
- Light panels, if needed: `#F3F2EE` or `#E7E4DD`

### Text

- Primary on light: `#1E1E1C`
- Primary on dark: `#C4C4C0`
- Secondary text: `#6E6E6C`
- Accent text: `#D4581A`

### Borders and Dividers

- Strong border: `#3A3A38`
- Standard divider: `#585856`
- Subtle divider: `rgba(255,255,255,0.10)` on dark backgrounds

### Interactive States

- Default action: `#D4581A`
- Hover action: `#F58040`
- Active action: `#C84C14`
- Disabled neutral: `#666664`

## Gradient Guidance

The logo is not flat. It uses restrained metallic and heat gradients. UI and brand assets can borrow this behavior carefully.

### Graphite Gradient

Use for premium surfaces, hero illustrations, or logo containers:

```css
linear-gradient(135deg, #3A3A38 0%, #6E6E6C 42%, #3A3A38 100%)
```

### Rust Core Gradient

Use for key badges, hero highlights, or branded visual nodes:

```css
radial-gradient(circle at 36% 30%, #F58040 0%, #C84C14 50%, #7C2806 100%)
```

## Style Principles

### Visual Tone

- engineered, not playful
- warm, but not loud
- premium open-source, not corporate enterprise gloss
- geometric and crisp rather than organic

### Shape Language

- hexagonal, radial, or faceted forms are on-brand
- beveled edges and subtle internal strokes fit the logo system
- avoid bubbly, rounded cartoon shapes

### Motion

- motion should feel smooth and deliberate
- prefer subtle glow, fade, rotation, or reveal effects
- avoid bouncy or exaggerated easing

### Texture

- matte graphite surfaces with restrained metallic highlights
- subtle depth is good
- noisy skeuomorphism is not

## Rust Community Tie-In

The brand should nod to the Rust ecosystem without copying Rust branding.

- Use rust-orange as a focused accent rather than a full-page wash
- Pair that warmth with dark graphite neutrals to signal performance and seriousness
- Keep layouts clean, documented, and systems-minded
- Favor identity cues that suggest reliability, speed, safety, and craftsmanship

GraphitePDF should read as:

`A modern Rust-native PDF engine with industrial polish and open-source credibility.`

## Do

- use deep graphite neutrals as the foundation
- reserve orange for emphasis and product energy
- maintain strong contrast and legibility
- keep spacing generous and layouts structured
- use `Space Grotesk` prominently in branded contexts

## Avoid

- oversaturating interfaces with orange
- bright rainbow accents unrelated to the core palette
- soft pastel branding
- playful illustration styles that clash with the engineered identity
- glossy gradients everywhere

## Quick Palette Summary

```text
Primary dark:   #1E1E1C
Primary mid:    #3A3A38
Primary light:  #C4C4C0
Accent main:    #D4581A
Accent glow:    #F58040
Accent deep:    #7C2806
Support gray:   #585856
```
