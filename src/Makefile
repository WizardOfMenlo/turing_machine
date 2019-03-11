all: *.rs bin/main.rs
	cargo build --release
	cp ../target/release/main runtm
	cp ../data/binadd.tm binadd.tm
	cp ../data/repeat.tm repeat.tm
	cp ../data/paren.tm paren.tm

.PHONY: clean
clean:
	rm -f runtm	
	rm -f binadd.tm
	rm -f paren.tm
	rm -f repeat.tm
