#!/bin/bash

# Color codes for better output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if PyInstaller is installed
check_pyinstaller() {
    if ! command -v pyinstaller &> /dev/null; then
        echo -e "${YELLOW}PyInstaller not found. Installing...${NC}"
        pip install pyinstaller
    fi
}

# Function to get all Python modules
get_python_modules() {
    local modules=()
    for file in *.py; do
        # Exclude main.py and build.sh
        if [[ "$file" != "main.py" ]] && [[ "$file" != "build.sh" ]]; then
            modules+=("$file")
        fi
    done
    echo "${modules[*]}"
}

# Function to build executable
build_executable() {
    # Check PyInstaller
    check_pyinstaller

    # Get additional modules
    local modules
    modules=$(get_python_modules)

    # Prepare module arguments for PyInstaller
    local module_args=()
    for module in $modules; do
        module_args+=("--add-data" "$module:.")
    done

    # Build command
    local build_cmd=(
        pyinstaller
        --onefile
        "${module_args[@]}"
        main.py
    )

    # Print build information
    echo -e "${GREEN}Building executable with modules:${NC}"
    echo -e "${YELLOW}Detected Modules:${NC}"
    echo "$modules"

    # Execute build
    "${build_cmd[@]}"

    # Check build status
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}? Build successful!${NC}"
        echo -e "${YELLOW}Executable location:${NC} dist/main"
    else
        echo -e "${RED}? Build failed!${NC}"
        exit 1
    fi
}

# Function to clean previous builds
clean_previous_build() {
    echo -e "${YELLOW}Cleaning previous builds...${NC}"
    rm -rf build dist *.spec
}

# Main script execution
main() {
    # Welcome message
    echo -e "${GREEN}? Automatic Python CLI Executable Builder${NC}"
    
    # Clean previous builds
    clean_previous_build

    # Build executable
    build_executable

    # Optional: Create checksum
    generate_checksum
}

# Function to generate checksum
generate_checksum() {
    if command -v sha256sum &> /dev/null; then
        echo -e "${YELLOW}Generating Checksum:${NC}"
        sha256sum dist/main
    fi
}

# Error handling
set -e
trap 'echo -e "${RED}? An error occurred during build${NC}"' ERR

# Execute main function
main
