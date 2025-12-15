RPS ?= 300
DURATION ?= 10
BENCH_PORT := 8090

.PHONY: bench-direct bench-tgin clean

clean:
	docker compose down

bench-direct: clean
	@echo "\n🤖 SCENARIO A: SINGLE BOT (DIRECT)"
	docker compose up -d worker_1 bench --build

	@sleep 10

	docker compose exec bench tgin-bench \
		--target http://10.5.0.11:8080/webhook \
		--port $(BENCH_PORT) \
		--rps $(RPS) \
		--duration $(DURATION)

bench-tgin: clean
	@echo "\n🚀 SCENARIO B: TGIN LOAD BALANCER"
	docker compose up -d tgin worker_1 worker_2 bench --build

	@sleep 10

	docker compose exec bench tgin-bench \
		--target http://10.5.0.2:3000/webhook \
		--port $(BENCH_PORT) \
		--rps $(RPS) \
		--duration $(DURATION)