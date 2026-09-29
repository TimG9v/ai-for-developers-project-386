.PHONY: dev dev-backend dev-frontend test test-backend test-frontend lint lint-backend lint-frontend

dev:
	@cd backend && cargo run &
	@cd frontend && npm run dev &
	@wait

dev-backend:
	cd backend && cargo run

dev-frontend:
	cd frontend && npm run dev

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
