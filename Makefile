.PHONY: dataset
dataset:
	@python evaluation/dataset/convert_to_qasm.py
	@python evaluation/dataset/generate_dataset.py

.PHONY: perf
perf:
	@python evaluation/run_lique_perf.py
	@python evaluation/run_lintq.py --perf

.PHONY: metrics
metrics:
	@python evaluation/extract_lintq_results.py
	@cargo run -p lique_evaluation -- --metrics

.PHONY: lique
lique:
	@cargo run -p lique_evaluation -- --lique
