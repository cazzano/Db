#!/usr/bin/python3
# Update import statement
from data import DataManager, DataDropManager  # Add DataDropManager
# Standard library imports
import sys
import os

# Import from main_3
from main_3 import CLIModuleHandler, CLIOperationsHandler

# Import default location manager
from default import DefaultLocationManager

# Import required modules
try:
    # Import version module
    version_module = CLIModuleHandler.safe_import('version')
    get_version = getattr(version_module, 'get_version', lambda: 'Unknown')

    # Import data modules
    data_module = CLIModuleHandler.safe_import('data')
    DataManager = getattr(data_module, 'DataManager')

    # Import advanced data operations
    data_2_module = CLIModuleHandler.safe_import('data_2')
    AdvancedDataOperations = getattr(data_2_module, 'AdvancedDataOperations')

    # Import advanced file operations from data_3
    data_3_module = CLIModuleHandler.safe_import('data_3')
    AdvancedFileOperations = getattr(data_3_module, 'AdvancedFileOperations')

except Exception as e:
    print(f"[FATAL] Module initialization failed: {e}")
    sys.exit(1)

class CLIApplication:
    def __init__(self):
        """
        Initialize CLI Application
        """
        self.parser = CLIModuleHandler.create_argument_parser()
        
        # Add new arguments
        self.parser.add_argument(
            '-l', '--location', 
            action='store_true',
            help='Manage default data management location'
        )
        
        # New argument for advanced file operations
        self.parser.add_argument(
            '--file-ops', 
            choices=['view-contents', 'backup', 'select-files'],
            help='Perform advanced file operations'
        )

    @CLIModuleHandler.handle_exceptions
    def run(self):
        """
        Main application logic
        """
        # Parse arguments
        args = self.parser.parse_args()

        # Default location management
        if args.location:
            DefaultLocationManager.interactive_default_location_management()
            return

        # Advanced file operations
        if args.file_ops:
            default_location = DefaultLocationManager.get_default_location()
            
            if args.file_ops == 'view-contents':
                # View directory contents
                AdvancedFileOperations.view_directory_contents(default_location)
            
            elif args.file_ops == 'backup':
                # Backup directory
                AdvancedFileOperations.backup_directory(default_location)
            
            elif args.file_ops == 'select-files':
                # Interactive file selection
                selected_files = AdvancedFileOperations.interactive_file_selection()
                print("Selected Files/Folders:")
                for file in selected_files:
                    print(f"- {file}")
            
            return

        # Verbose mode logging
        if args.verbose:
            print("[VERBOSE] Running in verbose mode")
            CLIOperationsHandler.enable_verbose_logging()

        # Version check
        if args.version:
            CLIOperationsHandler.show_version(get_version)
            return

        # Get default location if set
        default_location = DefaultLocationManager.get_default_location()

        # Copy operation
        if args.copy:
            destination = default_location if default_location else os.getcwd()
            CLIOperationsHandler.handle_copy(args.copy, AdvancedDataOperations, destination)
            return

        # Move operation
        if args.move:
            destination = default_location if default_location else os.getcwd()
            CLIOperationsHandler.handle_move(args.move, AdvancedDataOperations, destination)
            return

        # Storage management
        if args.storage:
            # Use default location if set
            if default_location:
                os.chdir(default_location)
            
            CLIOperationsHandler.handle_storage(args.storage, DataManager)
            return

        # File/Folder drop mode
        if args.drop:
            # Use default location if set
            if default_location:
                os.chdir(default_location)
            
            CLIOperationsHandler.handle_file_drop(DataManager)
            return

        # Show default message if no arguments are provided
        CLIOperationsHandler.show_default_message()

def main():
    """
    Entry point for the CLI application
    """
    try:
        app = CLIApplication()
        app.run()
    except Exception as e:
        print(f"[FATAL] Application encountered an error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
