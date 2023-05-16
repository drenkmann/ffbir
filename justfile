# Show this list
default:
    @just --list --unsorted

# Build release version
build: check-cargo
    @echo "Building ffbir..."
    cargo build --release

# Install manually
install: build
    @echo "Installing..."
    @sudo cp target/release/ffbir /usr/bin/ > /dev/null
    @echo ""
    @echo "Successfully installed ffbir into /usr/bin/"

# Remove manual installation
uninstall:
    @echo "Uninstalling..."
    @sudo rm /usr/bin/ffbir > /dev/null
    @echo ""
    @echo "Successfully uninstalled."

# Check if cargo is installed
check-cargo:
    @which cargo > /dev/null
    @echo "Cargo found."

# Check if fpm is installed
check-fpm:
    @which fpm > /dev/null
    @echo "FPM found"

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