all: *.rs bin/main.rs
	cargo build --release
	cp ../target/release/main runtm
	cp ../data/binadd.tm binadd.tm

.PHONY: clean
clean:
	rm -f runtm	
	rm -f binadd.tm