SRC ?= src
BUILD ?= build
RUSTC ?= rustc
ARCH ?= $(shell $(RUSTC) -v | grep host | cut -f 2 -d ' ')
SYSLIBDIR ?= /usr/local/lib/rustlib/$(ARCH)/lib
PROJNAME ?= $(shell basename $(CURDIR))
LIBSRC ?= $(SRC)/$(PROJNAME)/lib.rs
LIB ?= $(BUILD)/lib$(PROJNAME)-*.rlib
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

install:
	cp $(wildcard $(LIB)) $(SYSLIBDIR)

examples: $(BUILD) install $(EXAMPLESRCS)
	mkdir -p $(BUILD)/examples
	$(RUSTC) $(EXAMPLESRCS) --out-dir $(BUILD)/examples
