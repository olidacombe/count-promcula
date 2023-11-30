.PHONY: init readme

init:
	cargo install cargo-rdme

readme:
	cargo rdme --force
