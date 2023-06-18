UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
ARCH := elf64
endif
ifeq ($(UNAME), Darwin)
ARCH := macho64
TARGET := --target x86_64-apple-darwin
endif



.PRECIOUS: test/%.s

test/%.s: test/%.snek src/main.rs
	cargo run -- $< test/$*.s

test/%.run: test/%.s runtime/start.rs
	nasm -f $(ARCH) test/$*.s -o test/$*.o
	ar rcs test/lib$*.a test/$*.o
	rustc $(TARGET) -L test/ -lour_code:$* runtime/start.rs -o test/$*.run

