setup:
	@echo "Setting up environment"
	@pip install -r requirements.txt
	@curl -L https://risczero.com/install | bash
	@rzup

data:
	@echo "Generating data"
	python3 utils/generate_data.py config.yaml

build:
	@echo "Building the project"
	cargo build --release

run-local: setup
	@echo "Running the prover locally"
	RISC0_DEMO_CONFIG="config.yaml" cargo run --release

clean:
	@echo "Cleaning up"
	rm -rf data
	cargo clean
