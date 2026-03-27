container:
	@docker info > /dev/null 2>&1 || (echo "Starting Docker..." && open -a Docker)
	@echo "Waiting for Docker..."
	@while ! docker info > /dev/null 2>&1; do sleep 2; done
	@echo "Docker is ready"
