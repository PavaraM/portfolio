# Portfolio

Personal DevOps portfolio for [Pavara Mirihagalla](https://github.com/PavaraM). Single-file static site — no build step, no frameworks.

**Live:** [pavaram.github.io/portfolio](https://pavaram.github.io/portfolio)

## Structure

```
portfolio/
├── index.html                    # entire site (HTML, CSS, JS)
└── .github/workflows/static.yml  # GitHub Pages deploy on push to main
```

## Editing content

All content lives in `index.html`.

### Projects and skills

Edit `PROJECTS` and `SKILLS` in the `<script>` block:

- **`PROJECTS`** — project cards in the Projects section
- **`SKILLS`** — stack pills shown in the hero

Each project object:

| Field   | Description                          |
|---------|--------------------------------------|
| `id`    | Unique slug                          |
| `name`  | Display name                         |
| `type`  | Small tag (e.g. `bash · infra`)      |
| `desc`  | Card description                     |
| `tags`  | Array of tech tags                   |
| `stats` | Array of metadata lines              |
| `url`   | GitHub repo or project link          |

### Hero, about, and contact

Edit the HTML in `<body>` directly:

- **Hero** — `#hero` (eyebrow, headline, description, CTA buttons)
- **About** — `#about` (bio paragraphs and fact list)
- **Contact** — `#contact` (email and GitHub links)

## Local preview

```bash
python -m http.server 8080
```

Open [http://localhost:8080](http://localhost:8080).

## Deploy

Push to `main`. The GitHub Actions workflow uploads the repo and deploys to GitHub Pages. Allow ~30 s for the site to update.

## Features

- Light/dark theme toggle (persisted in `localStorage`, defaults to system preference)
- Scroll-reveal animations, scrollspy nav highlighting, and scroll-to-top — all respecting `prefers-reduced-motion`
- Fully static — no build step, no editor, no runtime state
