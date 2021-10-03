.PHONY: bless

bless:
	fd tests -e new --no-ignore-vcs --full-path --exec mv {} {.}
