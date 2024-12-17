.PHONY: dataset
dataset:
	@python evaluation/dataset/convert_to_qasm.py
	@python evaluation/dataset/generate_dataset.py

.PHONY: metrics
metrics:
	@python evaluation/extract_lintq_results.py
	@cargo run -p lique_evaluation -- --metrics

.PHONY: lique
lique:
	@cargo run -p lique_evaluation -- --lique
