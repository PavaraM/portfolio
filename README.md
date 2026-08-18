# Portfolio

Personal DevOps portfolio for [Pavara Mirihagalla](https://github.com/PavaraM). Single-file static site — no build step, no frameworks.

**Live:** [pavaradev.duckdns.org](https://pavaradev.duckdns.org) · [GitHub Pages backup](https://pavaram.github.io/portfolio)

## Structure

```
portfolio/
├── index.html                        # entire site (HTML, CSS, JS)
├── Dockerfile                        # Nginx container image
├── docker-compose.yml                # Caddy reverse proxy + Nginx app
├── docker/
│   ├── nginx.conf                    # Nginx config for static site
│   └── Caddyfile                     # Reverse proxy (TLS, /birthday/* routing)
├── scripts/
│   ├── deploy.sh                     # SSH deploy with healthcheck + auto-rollback
│   └── rollback.sh                   # Rollback to previous image
├── Makefile                          # docker-build, deploy, rollback, ssh, status
└── .github/workflows/
    ├── deploy.yml                    # OCI deployment (Docker + SSH)
    └── static.yml                    # GitHub Pages (backup)
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

## Docker (local)

```bash
make docker-run     # build + start on http://localhost:8080
make docker-stop    # stop containers
```

## Deploy

Push to `main`. Two workflows run:

1. **`deploy.yml`** — builds a Docker image, pushes to GHCR, deploys to the OCI instance over SSH with healthcheck + auto-rollback.
2. **`static.yml`** — deploys to GitHub Pages as a backup.

### Manual deploy / rollback

```bash
# Deploy (requires VM_HOST, VM_USER, VM_SSH_PORT, VM_SSH_KEY)
make deploy

# Rollback to previous image
make rollback

# Check remote status
make status

# SSH into the instance
make ssh
```

### Architecture

```
Internet → :80/:443 → Caddy (reverse proxy + TLS)
    /           → Nginx container (portfolio)
    /birthday/* → Birthday Caddy container (host.docker.internal:8080)
```

## Features

- Light/dark theme toggle (persisted in `localStorage`, defaults to system preference)
- Scroll-reveal animations, scrollspy nav highlighting, and scroll-to-top — all respecting `prefers-reduced-motion`
- Fully static — no build step, no editor, no runtime state
