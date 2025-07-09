# Makefile for elevator project

.PHONY: help build dev prod clean logs stop

help: ## Show this help message
	@echo "Available commands:"
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development
dev: ## Run in development mode with simulator
	@echo "Starting elevator system in development mode..."
	@docker compose --profile dev up --build

dev-detached: ## Run in development mode (detached)
	@echo "Starting elevator system in development mode (detached)..."
	@docker compose --profile dev up --build -d

##@ Production  
prod: ## Run in production mode for real hardware
	@echo "Starting elevator system in production mode..."
	@docker compose --profile prod up --build

prod-detached: ## Run in production mode (detached)
	@echo "Starting elevator system in production mode (detached)..."
	@docker compose --profile prod up --build -d

##@ Management
build: ## Build all Docker images
	@echo "Building Docker images..."
	@docker compose build

clean: ## Clean up containers and images
	@echo "Cleaning up..."
	@docker compose down -v --remove-orphans
	@docker system prune -f

stop: ## Stop all running containers
	@echo "Stopping all containers..."
	@docker compose down

logs: ## Show logs from all services
	@docker compose logs -f

logs-controller: ## Show logs from elevator controller only
	@docker compose logs -f elevator-controller

logs-simulator: ## Show logs from simulator only
	@docker compose logs -f elevator-simulator

##@ Utilities
shell-controller: ## Get shell access to controller container
	@docker exec -it elevator-controller bash

shell-simulator: ## Get shell access to simulator container  
	@docker exec -it elevator-simulator bash

config-check: ## Validate configuration files
	@echo "Checking configuration files..."
	@test -f config-dev.toml || echo "Warning: config-dev.toml not found"
	@test -f config-prod.toml || echo "Warning: config-prod.toml not found"
	@test -f SimElevatorServer || echo "Warning: SimElevatorServer executable not found"
	@echo "Configuration check complete"

docker-check: ## Check Docker and Docker Compose installation
	@echo "Checking Docker installation..."
	@docker --version || (echo "Docker not found. Please install Docker." && exit 1)
	@echo "Checking Docker Compose..."
	@docker compose version || (echo "Docker Compose plugin not found. Please update Docker." && exit 1)
	@echo "Docker setup is ready!"

##@ Development Tools
rust-fmt: ## Format Rust code
	@cargo fmt

rust-test: ## Run Rust tests
	@cargo test

rust-check: ## Check Rust code without building
	@cargo check