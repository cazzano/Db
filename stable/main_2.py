# main_2.py
import os
import sys
import traceback

class CLIOperations:
    @staticmethod
    def handle_copy(source):
        """
        Handle file or folder copy operation
        """
        print(f"[COPY] Attempting to copy: {source}")
        
        try:
            from drop_error import FileDropHandler
            from folder_drop import FolderDropHandler

            # Determine if source is a file or folder
            if os.path.isdir(source):
                copy_result = FolderDropHandler.safe_folder_copy(source)
                success = copy_result['success']
            else:
                success = FileDropHandler.safe_file_copy(source)
            
            if success:
                print("[COPY] Operation completed successfully")
            else:
                print("[COPY] Operation failed")
        except Exception as e:
            print(f"Copy operation error: {e}")

    @staticmethod
    def handle_move(source):
        """
        Handle file or folder move operation
        """
        print(f"[MOVE] Attempting to move: {source}")
        
        try:
            from drop_error import FileDropHandler
            from folder_drop import FolderDropHandler

            # Determine if source is a file or folder
            if os.path.isdir(source):
                success = FolderDropHandler.move_folder(source)
            else:
                success = FileDropHandler.move_file(source)
            
            if success:
                print("[MOVE] Operation completed successfully")
            else:
                print("[MOVE] Operation failed")
        except Exception as e:
            print(f"Move operation error: {e}")

    @staticmethod
    def handle_file_drop():
        """
        Initiate file or folder drop mode
        """
        print("[DROP MODE] Entering file/folder drop mode")
        try:
            from data import DataManager
            
            drop_result = DataManager.handle_file_drop()
            
            if drop_result:
                print("[DROP MODE] File/Folder successfully processed")
            else:
                print("[DROP MODE] File/Folder processing failed")
        except Exception as e:
            print(f"File drop error: {e}")

    @staticmethod
    def handle_storage(storage_type):
        """
        Handle different storage operations
        """
        print(f"[STORAGE] Initiating {storage_type.upper()} storage management")
        
        try:
            from data import DataManager

            if storage_type == 'data':
                DataManager.process_storage_request()
            elif storage_type == 'ad':
                # Advanced storage options
                DataManager.advanced_storage_options()
            else:
                print(f"[WARNING] Storage type '{storage_type}' not fully implemented")
        except Exception as e:
            print(f"Storage management error: {e}")

    @staticmethod
    def show_default_message():
        """
        Display default welcome message
        """
        print("Welcome to the CLI File Management Application!")
        print("Use -h or --help to see available commands")
        print("Quick Start:")
        print("  - Use -d or --drop to enter file/folder drop mode")
        print("  - Use -s data for storage management")
        print("  - Use -s ad for advanced storage options")
        print("  - Use -c <source> to copy a file or folder")
        print("  - Use -m <source> to move a file or folder")
