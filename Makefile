SHELL := /bin/bash
.DEFAULT_GOAL := help

APP_IMAGE   ?= ghcr.io/pavaram/portfolio
APP_VERSION ?= $(shell git rev-parse --short HEAD 2>/dev/null || echo dev)
COMPOSE     ?= docker compose
REMOTE_DIR  ?= /opt/portfolio

.PHONY: help docker-build docker-run docker-stop \
        deploy rollback ssh status clean

help: ## list available targets
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | \
		awk 'BEGIN{FS=":.*?## "}{printf "  \033[36m%-15s\033[0m %s\n",$$1,$$2}'

docker-build: ## build the production container image
	docker build -t $(APP_IMAGE):$(APP_VERSION) -t $(APP_IMAGE):latest .

docker-run: docker-build ## build + run the container locally (http://localhost:8080)
	$(COMPOSE) up -d

docker-stop: ## stop local containers
	$(COMPOSE) down

deploy: ## deploy latest image to the remote host (requires VM_* env vars)
	bash scripts/deploy.sh

rollback: ## roll the remote host back to the previous image (requires VM_* env vars)
	bash scripts/rollback.sh

ssh: ## open a shell on the remote host (requires VM_* env vars)
	ssh -p $${VM_SSH_PORT:-22} -i $${VM_SSH_KEY:-$$HOME/.ssh/id_ed25519} $${VM_USER:-opc}@$${VM_HOST}

status: ## show container status on the remote host
	ssh -p $${VM_SSH_PORT:-22} $${VM_USER:-opc}@$${VM_HOST} \
		"docker compose -f $(REMOTE_DIR)/docker-compose.yml ps"

clean: ## remove local containers and volumes
	$(COMPOSE) down -v --remove-orphans 2>/dev/null || true
