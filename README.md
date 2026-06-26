# pavaram.github.io/portfolio

Single-file static site. No build step, no frameworks.

## Editing

All portfolio content lives in `index.html` under `DEFAULTS` in the `<script>` block. Edit the source directly:

- **Projects** — `DEFAULTS.projects` array
- **Skills** — `DEFAULTS.skills` array
- **About / Hero** — HTML in the `<body>`

The admin panel overlay is still in source but is unreachable from the public UI (the Edit FAB was removed).

## Deploy

Push to `main`. GitHub Pages serves from `pavaram.github.io/portfolio`. Allow ~30 s for redeploy.
