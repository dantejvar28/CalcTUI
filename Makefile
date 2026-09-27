CARGO ?= cargo
PKGDIR = packaging

.PHONY: all build run test install uninstall package clean

all: build

build:
	$(CARGO) build --release

run:
	$(CARGO) run

test:
	$(CARGO) test

# instala el comando calctui en ~/.cargo/bin
install:
	$(CARGO) install --path . --force

uninstall:
	$(CARGO) uninstall

# genera el paquete de Arch, queda en packaging/*.pkg.tar.zst
package:
	cd $(PKGDIR) && makepkg -f

clean:
	$(CARGO) clean
	rm -rf $(PKGDIR)/pkg $(PKGDIR)/src $(PKGDIR)/*.pkg.tar.*
