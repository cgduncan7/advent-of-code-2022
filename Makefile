day:
	cargo new day$$n;
	cd day$$n; cargo add --path ../harness; touch data.txt; touch example.txt; cp ../templates/main.rs ./src/main.rs

run:
	@echo "\n======\nDAY $$n\n======"
	@cd day$$n; cargo run -q;

run-all:
	ls -l | awk '/day/ { print $$9 }' | sed 's/day/n=/' | xargs -L 1 make run