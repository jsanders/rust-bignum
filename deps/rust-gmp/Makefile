SRC ?= .
BUILD ?= build
RUSTC ?= rustc
ARCH ?= $(shell $(RUSTC) -v | grep host | cut -f 2 -d ' ')
SYSLIBDIR ?= /usr/local/lib/rustlib/$(ARCH)/lib
LIBSRC ?= $(SRC)/gmp.rs

all: clean test

clean:
	rm -rf $(BUILD)/* || true

$(BUILD):
	mkdir -p $(BUILD)

lib: $(BUILD) $(LIBSRC)
	$(RUSTC) --out-dir $(BUILD) $(LIBSRC)

test: lib
	$(RUSTC) --test -o $(BUILD)/test $(LIBSRC)
	$(BUILD)/test

install: $(wildcard $(BUILD)/libgmp-*.rlib)
	cp $? $(SYSLIBDIR)
