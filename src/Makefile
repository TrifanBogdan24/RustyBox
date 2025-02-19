build_simple:
	rustc main.rs

clean:
	rm -f main

build_crates:
	cd ../ ; \
	cargo install --path . ; \
	cargo build > src/erors_and_warnings.txt 2>&1 ;  \
	cd src/

test:
	cd ../ ; \
	cargo install --path . ; \
	cargo build ;  \
	cargo build > src/erors_and_warnings.txt 2>&1 ;  \
	cd tests/ ; ./run_all.sh > ../src/rezultate.txt 2>&1 ; \
	cd ../src

zip:
	rm -rf *.zip ; \
	zip -r rust-tema-1-src.zip *
