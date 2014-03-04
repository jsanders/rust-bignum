SRC ?= src
BUILD ?= build
RUSTC ?= rustc
ARCH ?= $(shell $(RUSTC) -v | grep host | cut -f 2 -d ' ')
SYSLIBDIR ?= /usr/local/lib/rustlib/$(ARCH)/lib
LIBSRC ?= $(SRC)/bignum/lib.rs
LIB ?= 
EXAMPLESRCS ?= $(wildcard $(SRC)/examples/*.rs)

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

install: $(wildcard $(BUILD)/libbignum-*.rlib)
	cp $? $(SYSLIBDIR)

examples: $(BUILD) install $(EXAMPLESRCS)
	mkdir $(BUILD)/examples
	$(RUSTC) $(EXAMPLESRCS) --out-dir $(BUILD)/examples
