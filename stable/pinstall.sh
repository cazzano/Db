#!/usr/bin/env bash

# Dependency Installation Script
# Version: 1.0.0

# Color Codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Logging Functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect project directory
PROJECT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Detect Virtual Environment
detect_venv() {
    local venv_paths=("venv" ".venv" "env")
    local path
    
    for path in "${venv_paths[@]}"; do
        if [ -d "$path" ] && [ -f "$path/bin/activate" ]; then
            echo "$path"
            return 0
        fi
    done
    
    return 1
}

# Upgrade pip and setuptools
upgrade_pip() {
    local venv_path="$1"
    
    log_info "Upgrading pip and setuptools..."
    "$venv_path/bin/pip" install --upgrade pip setuptools wheel
}

# Install Dependencies
install_dependencies() {
    local venv_path="$1"
    local requirements_files=(
        "requirements.txt" 
        "req.txt" 
        "requirements-dev.txt"
        "dev-requirements.txt"
    )
    local req_file
    local installed=false
    
    for req_file in "${requirements_files[@]}"; do
        if [ -f "$PROJECT_DIR/$req_file" ]; then
            log_info "Installing dependencies from $req_file"
            "$venv_path/bin/pip" install -r "$PROJECT_DIR/$req_file"
            
            if [ $? -eq 0 ]; then
                log_info "Dependencies from $req_file installed successfully"
                installed=true
            else
                log_error "Failed to install dependencies from $req_file"
                return 1
            fi
        fi
    done
    
    if [ "$installed" = false ]; then
        log_warn "No requirements file found"
        return 1
    fi
}

# Optional Development Dependencies
install_dev_dependencies() {
    local venv_path="$1"
    
    log_info "Checking for optional development dependencies..."
    
    # Example: Install development tools
    "$venv_path/bin/pip" install pytest pylint black
    
    log_info "Optional development tools installed"
}

# Main Workflow
main() {
    cd "$PROJECT_DIR" || exit 1
    
    # Detect virtual environment
    local venv_path
    venv_path=$(detect_venv)
    
    if [ -z "$venv_path" ]; then
        log_error "No virtual environment found. Please run psrc.sh first."
        exit 1
    fi
    
    # Full dependency installation workflow
    upgrade_pip "$venv_path"
    install_dependencies "$venv_path"
    
    # Optional: Install dev dependencies
    read -p "Install development dependencies? (y/N): " dev_choice
    if [[ "$dev_choice" =~ ^[Yy]$ ]]; then
        install_dev_dependencies "$venv_path"
    fi
}

# Execute main function
main "$@"
