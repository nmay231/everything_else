WGET=$(which wget)
CURL := $(shell which curl)

init-day%:
	$(CURL) --cookie '$(COOKIE)' https://adventofcode.com/2023/day/${@:init-day%=%}/input -o assets/day${@:init-day%=%}.txt
	cp src/bin/blank.rs src/bin/day${@:init-day%=%}.rs
	sed -i 's/DAY_NUMBER/${@:init-day%=%}/g' src/bin/day${@:init-day%=%}.rs

run-day%:
	cargo run --bin day${@:run-day%=%}
