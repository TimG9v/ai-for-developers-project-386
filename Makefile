.PHONY: dev dev-backend dev-frontend test test-backend test-frontend lint lint-backend lint-frontend generate

dev:
	@cd backend && cargo run &
	@cd frontend && npm run dev &
	@wait

dev-backend:
	cd backend && cargo run

dev-frontend:
	cd frontend && npm run dev

generate:
	cd contracts && npx tsp compile .
	cd frontend && npm run generate:client
	cd backend && cargo build

test: test-backend test-frontend

test-backend:
	cd backend && cargo test

test-frontend:
	cd frontend && npm test

lint: lint-backend lint-frontend

lint-backend:
	cd backend && cargo fmt --check
	cd backend && cargo clippy --all-targets -- -D warnings

lint-frontend:
	cd frontend && npm run lint
