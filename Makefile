SRC ?= src
BUILD ?= build
RUSTC ?= rustc
ARCH ?= $(shell $(RUSTC) -v | grep host | cut -f 2 -d ' ')
SYSLIBDIR ?= /usr/local/lib/rustlib/$(ARCH)/lib
DEPS ?= $(wildcard deps/*)
PROJNAME ?= $(subst rust-,,$(shell basename $(CURDIR)))
LIBSRC ?= $(SRC)/$(PROJNAME)/lib.rs
LIB ?= $(BUILD)/lib$(PROJNAME)-*.rlib
EXAMPLESRCS ?= $(wildcard $(SRC)/examples/*.rs)

.PHONY: all clean cleandeps deps lib test install examples

all: test

$(BUILD):
	mkdir -p $(BUILD)

clean: cleandeps
	rm -rf $(BUILD)/* || true

cleandeps:
	@for dep in $(DEPS) ; do \
		$(MAKE) -w -C $$dep clean ; \
	done

deps:
	@for dep in $(DEPS) ; do \
		$(MAKE) -w -C $$dep && $(MAKE) -w -C $$dep install ; \
	done

lib: $(BUILD) deps $(LIBSRC)
	$(RUSTC) --out-dir $(BUILD) $(LIBSRC)

test: lib
	$(RUSTC) --test -o $(BUILD)/test $(LIBSRC)
	$(BUILD)/test

install:
	cp $(wildcard $(LIB)) $(SYSLIBDIR)

examples: $(BUILD) lib install $(EXAMPLESRCS)
	mkdir -p $(BUILD)/examples
	$(RUSTC) $(EXAMPLESRCS) --out-dir $(BUILD)/examples
