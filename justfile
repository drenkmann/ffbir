# Show this list
default:
    @just --list --unsorted

alias b := build
alias i := install
alias u := uninstall

# Build release version
build: check-cargo
    cargo build --release

# Install manually
install: build
    sudo cp target/release/ffbir /usr/bin/

# Remove manual installation
uninstall:
    sudo rm /usr/bin/ffbir

# Check if cargo is installed
check-cargo:
    which cargo

# Check if fpm is installed
check-fpm:
    which fpm

# Package for all
package-all:
    -fpm -t rpm
    -fpm -t deb
    -fpm -t pacman
    -fpm -t sh

# Package rpm binary
package-rpm: check-fpm build
    fpm -t rpm

# Package deb binary
package-deb: check-fpm build
    fpm -t deb

# Package pacman binary
package-pacman: check-fpm build
    fpm -t pacman

# Package self-extracting install script
package-sh: check-fpm build
    fpm -t sh