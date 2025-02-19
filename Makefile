build_simple:
	rustc src/main.rs

clean:
	rm -f src/main

build_crates:
	cargo install --path . ; \
	cargo build > src/erors_and_warnings.txt 2>&1 ;  \

test:
	cargo install --path . ; \
	cargo build ;  \
	cargo build > src/erors_and_warnings.txt 2>&1 ;  \
	cd tests/ ; ./run_all.sh > ../src/rezultate.txt 2>&1 ; \

zip:
	rm -rf *.zip ; \
	zip -r rust-tema-1.zip *
