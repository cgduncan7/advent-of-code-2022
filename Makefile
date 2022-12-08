day:
	cargo new day$$n;
	cd day$$n; cargo add --path ../harness; touch data.txt; touch example.txt;

run:
	cd day$$n; cargo run;