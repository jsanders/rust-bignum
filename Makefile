SRC ?= src
BUILD ?= build
RUSTC ?= rustc
ARCH ?= $(shell $(RUSTC) -v | grep host | cut -f 2 -d ' ')
SYSLIBDIR ?= /usr/local/lib/rustlib/$(ARCH)/lib
LIBSRC ?= $(SRC)/bignum/lib.rs
EXAMPLESRCS ?= $(wildcard $(SRC)/examples/*.rs)

all: clean test lib

clean:
	rm -rf $(BUILD)/* || true

$(BUILD):
	mkdir -p $(BUILD)

test: $(BUILD) $(LIBSRC)
	$(RUSTC) --test -o $(BUILD)/test $(LIBSRC)
	$(BUILD)/test

lib: $(BUILD) $(LIBSRC)
	$(RUSTC) --out-dir $(BUILD) $(LIBSRC)

install: lib
	cp $(wildcard $(BUILD)/libbignum-*.rlib) $(SYSLIBDIR)

examples: $(BUILD) install $(EXAMPLESRCS)
	mkdir $(BUILD)/examples
	$(RUSTC) $(EXAMPLESRCS) --out-dir $(BUILD)/examples
