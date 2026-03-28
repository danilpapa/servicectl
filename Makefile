ROOT_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

container:
	@docker info > /dev/null 2>&1 || (echo "Starting Docker..." && open -a Docker)
	@echo "Waiting for Docker..."
	@while ! docker info > /dev/null 2>&1; do sleep 2; done
	@cargo run -q "$(ROOT_DIR)"
