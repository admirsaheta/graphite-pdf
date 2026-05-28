# GraphitePDF Brand Assets

This directory contains the brand system for GraphitePDF.

It is intended to support:

- repository branding
- documentation styling
- package and release graphics
- app icon and social asset generation
- future website and docs site implementation

## Asset Index

### Core Guides

- Palette guide: `design-pallete.md`
- Visual identity guide: `visual-identity.md`

### Token Sources

- JSON tokens: `tokens.json`
- YAML tokens: `tokens.yaml`
- CSS variables: `colors.css`

### Logos and Marks

- Full logo: `logo/graphitepdf-logo-oss.svg`
- Standalone mark: `mark/graphitepdf-mark.svg`

### Icon Variants

- Favicon mark: `icon/graphitepdf-favicon.svg`
- App icon: `icon/graphitepdf-app-icon.svg`

## Usage Guidance

- Use the full logo for README headers, release graphics, and wide placements
- Use the standalone mark for avatars, favicons, and compact brand surfaces
- Use the token files as the source of truth for any future docs, marketing, or UI work
- Keep the docs site implementation for later so the brand system remains stable first

## Brand Summary

GraphitePDF should feel:

- precise
- industrial
- trustworthy
- Rust-native
- modern open source

The brand system is built around:

- deep graphite neutrals
- focused rust-orange accents
- `Space Grotesk` for the brand wordmark and headline tone
- geometric, engineered visual structure

## Maintenance Notes

- Update `tokens.json`, `tokens.yaml`, and `colors.css` together when the palette changes
- Derive new icon and badge assets from the existing mark instead of redrawing from scratch
- Keep the logo palette consistent with the approved graphite and rust color system
