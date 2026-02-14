# Digital K-Hole justfile

# default recipe: list available commands
default:
    @just --list

# install deps for dev
setup:
    echo "https://rustup.rs/"
    echo "If on arch, just 'sudo -Syu rustup'. it will remove standalone rust."
    rustup default stable
    rustup target add wasm32-unknown-unknown

# run dev server with hot reload
serve: setup build
    ~/.cargo/bin/trunk serve

# run dev server on specific port
serve-port port="8080":
    ~/.cargo/bin/trunk serve --port {{port}}

# build for development
build:
    ~/.cargo/bin/trunk build

# build for release (optimized, smaller wasm)
release:
    ~/.cargo/bin/trunk build --release

# clean build artifacts
clean:
    cargo clean
    rm -rf dist/

# check code without building
check:
    cargo check --target wasm32-unknown-unknown

# format code
fmt:
    cargo fmt

# run clippy lints
lint:
    cargo clippy --target wasm32-unknown-unknown

# watch for changes and rebuild
watch:
    ~/.cargo/bin/trunk watch

# validate all tanka yaml files against schema
validate:
    cargo run --bin validate

# ============================================================================
# systemd service management
# ============================================================================

# Install as systemd service running on port 3004
# Run with sudo
systemd-install:
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ $EUID -ne 0 ]]; then
        echo "Error: This recipe must be run as root (use sudo)."
        exit 1
    fi

    # Use current directory as REPO_DIR (user should run from repo root)
    REPO_DIR="$(pwd)"
    SERVICE_NAME="digitalkhole-book"
    PORT="${PORT:-3004}"
    USER="${SUDO_USER:-root}"

    echo "Installing systemd service: ${SERVICE_NAME}"

    # Build release first (optional, trunk serve builds on the fly)
    echo "Note: trunk serve will build on-demand"

    # Copy and template service file
    sed -e "s|USER_PLACEHOLDER|${USER}|g" \
        -e "s|REPO_DIR_PLACEHOLDER|${REPO_DIR}|g" \
        "${REPO_DIR}/systemd/${SERVICE_NAME}.service" \
        > /etc/systemd/system/${SERVICE_NAME}.service

    # Reload systemd and enable service
    systemctl daemon-reload
    systemctl enable ${SERVICE_NAME}
    systemctl restart ${SERVICE_NAME}

    echo "Service installed and started!"
    echo ""
    echo "Commands:"
    echo "  sudo systemctl status ${SERVICE_NAME}"
    echo "  sudo systemctl restart ${SERVICE_NAME}"
    echo "  sudo journalctl -u ${SERVICE_NAME} -f"

# Uninstall systemd service
# Run with sudo
systemd-uninstall:
    #!/usr/bin/env bash
    SERVICE_NAME="digitalkhole-book"

    if [[ $EUID -ne 0 ]]; then
        echo "Error: This recipe must be run as root (use sudo)."
        exit 1
    fi

    echo "Stopping and disabling ${SERVICE_NAME}..."
    systemctl stop ${SERVICE_NAME} 2>/dev/null || true
    systemctl disable ${SERVICE_NAME} 2>/dev/null || true
    rm -f /etc/systemd/system/${SERVICE_NAME}.service
    systemctl daemon-reload
    echo "Service uninstalled."

# Show service status
systemd-status:
    #!/usr/bin/env bash
    SERVICE_NAME="${SERVICE_NAME:-digitalkhole-book}"
    systemctl status ${SERVICE_NAME}

# View service logs
systemd-logs:
    #!/usr/bin/env bash
    SERVICE_NAME="${SERVICE_NAME:-digitalkhole-book}"
    journalctl -u ${SERVICE_NAME} -f

# Restart the service
systemd-restart:
    #!/usr/bin/env bash
    SERVICE_NAME="${SERVICE_NAME:-digitalkhole-book}"
    systemctl restart ${SERVICE_NAME}
    systemctl status ${SERVICE_NAME}
