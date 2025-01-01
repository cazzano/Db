#!/usr/bin/env bash

# Virtual Environment Management Script
# Version: 1.1.0

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

# Detect script and project directories
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$SCRIPT_DIR"

# Python Version Check
check_python_version() {
    local min_version="3.7"
    local python_cmd=""

    # Determine Python command
    if command -v python3 &> /dev/null; then
        python_cmd="python3"
    elif command -v python &> /dev/null; then
        python_cmd="python"
    else
        log_error "Python not found!"
        return 1
    fi

    # Check Python version
    local python_version
    python_version=$("$python_cmd" --version | awk '{print $2}')
    
    if [ "$(printf '%s\n' "$min_version" "$python_version" | sort -V | head -n1)" = "$min_version" ]; then
        log_info "Python version check passed: $python_version"
        return 0
    else
        log_error "Minimum Python version $min_version required. Current: $python_version"
        return 1
    fi
}

# Virtual Environment Detection
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

# Create Virtual Environment
create_virtual_env() {
    local venv_name="venv"
    
    log_info "Creating virtual environment..."
    
    if check_python_version; then
        if command -v python3 &> /dev/null; then
            python3 -m venv "$venv_name"
        else
            python -m venv "$venv_name"
        fi
        
        if [ $? -eq 0 ]; then
            log_info "Virtual environment created successfully in $venv_name"
            return 0
        else
            log_error "Failed to create virtual environment"
            return 1
        fi
    else
        log_error "Python version check failed"
        return 1
    fi
}

# Print Virtual Environment Information
print_venv_info() {
    local venv_path="$1"
    
    log_info "Virtual Environment Details:"
    echo "Path: $venv_path"
    echo "Python Executable: $venv_path/bin/python"
    "$venv_path/bin/python" --version
}

# Main Workflow
main() {
    cd "$PROJECT_DIR" || exit 1
    
    # Check for existing virtual environment
    local venv_path
    venv_path=$(detect_venv)
    
    if [ -z "$venv_path" ]; then
        log_warn "No virtual environment detected"
        
        # Create virtual environment
        if ! create_virtual_env; then
            log_error "Virtual environment creation failed"
            return 1
        fi
        
        venv_path="venv"
    else
        log_info "Existing virtual environment detected: $venv_path"
    fi
    
    # Print virtual environment information
    print_venv_info "$venv_path"

    # Provide instructions for activation
    log_info "To activate the virtual environment, use:"
    echo "source $venv_path/bin/activate"
}

# Execute main function
main "$@"
