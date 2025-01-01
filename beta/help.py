# help.py

def display_help():
    """
    Display comprehensive help information for the CLI application.
    """
    help_text = """
CLI Application Help

Usage: python3 main.py [OPTIONS]

Options:
  -v, --version     Show the application version
  -h, --help        Display this help message

Examples:
  python3 main.py -v       # Display version
  python3 main.py --help   # Show help information
"""
    print(help_text)

def get_help_description():
    """
    Return a short description for argparse help.
    """
    return "CLI Application with version and help support"
