#!/usr/bin/python3

import os
import shutil
import inquirer
from datetime import datetime
from drop_error import FileDropHandler
from folder_drop import FolderDropHandler
from default import DefaultLocationManager
from data_3 import AdvancedFileOperations

class AdvancedDataOperations:
    @staticmethod
    def sanitize_path(path):
        """
        Delegate to AdvancedFileOperations
        """
        return AdvancedFileOperations.sanitize_path(path)

    @staticmethod
    def copy_multiple_items(default_location=None):
        """
        Copy multiple files and folders interactively with enhanced validation
        """
        print("Multiple File/Folder Copy Mode")
        items_to_copy = []
        
        # Use default location if provided, otherwise prompt
        if not default_location:
            destination_prompt = inquirer.confirm(
                "Do you want to specify a destination directory?", 
                default=False
            )

            if destination_prompt:
                destination_questions = [
                    inquirer.Text(
                        'destination', 
                        message="Enter the destination directory path",
                        validate=lambda answers, current: (
                            os.path.isdir(AdvancedDataOperations.sanitize_path(current)) and 
                            len(AdvancedDataOperations.sanitize_path(current)) > 0
                        )
                    )
                ]
                destination_answers = inquirer.prompt(destination_questions)
                destination_dir = AdvancedDataOperations.sanitize_path(destination_answers['destination'])
            else:
                # Use current directory as default destination
                destination_dir = os.getcwd()
        else:
            # Use the provided default location
            destination_dir = default_location
            print(f"Using default location: {destination_dir}")

        # Use interactive file selection from data_3
        source_paths = AdvancedFileOperations.interactive_file_selection()

        for source_path in source_paths:
            try:
                # Determine if it's a file or folder
                if os.path.isdir(source_path):
                    # Folder copy
                    copy_result = FolderDropHandler.safe_folder_copy(
                        source_path, 
                        destination_dir
                    )
                    success = copy_result['success']
                else:
                    # File copy
                    success = FileDropHandler.safe_file_copy(
                        source_path, 
                        destination_dir
                    )
                
                if success:
                    items_to_copy.append(source_path)
                    print(f"Successfully copied: {source_path}")
                else:
                    print(f"Failed to copy: {source_path}")
            
            except Exception as e:
                print(f"Error copying {source_path}: {e}")

        # Copy operation summary
        print(f"\nCopy operation summary:")
        print(f"Total items copied: {len(items_to_copy)}")
        print(f"Destination directory: {destination_dir}")

        return len(items_to_copy) > 0

    @staticmethod
    def move_items(default_location=None):
        """
        Move files and folders between directories with enhanced validation
        """
        print("File/Folder Move Functionality")
        items_to_move = []
        
        # Use default location if provided, otherwise prompt
        if not default_location:
            destination_prompt = inquirer.confirm(
                "Do you want to specify a destination directory?", 
                default=False
            )

            if destination_prompt:
                destination_questions = [
                    inquirer.Text(
                        'destination', 
                        message="Enter the destination directory path",
                        validate=lambda answers, current: (
                            os.path.isdir(AdvancedDataOperations.sanitize_path(current)) and 
                            len(AdvancedDataOperations.sanitize_path(current)) > 0
                        )
                    )
                ]
                destination_answers = inquirer.prompt(destination_questions)
                destination_dir = AdvancedDataOperations.sanitize_path(destination_answers['destination'])
            else:
                # Use current directory as default destination
                destination_dir = os.getcwd()
        else:
            # Use the provided default location
            destination_dir = default_location
            print(f"Using default location: {destination_dir}")

        # Use interactive file selection from data_3
        source_paths = AdvancedFileOperations.interactive_file_selection("Select files/folders to move")

        for source_path in source_paths:
            try:
                # Determine if it's a file or folder
                if os.path.isdir(source_path):
                    # Folder move
                    success = FolderDropHandler.move_folder(
                        source_path, 
                        destination_dir
                    )
                else:
                    # File move
                    success = FileDropHandler.move_file(
                        source_path, 
                        destination_dir
                    )
                
                if success:
                    items_to_move.append(source_path)
                    print(f"Successfully moved: {source_path}")
                else:
                    print(f"Failed to move: {source_path}")
            
            except Exception as e:
                print(f"Error moving {source_path}: {e}")

        # Move operation summary
        print(f"\nMove operation summary:")
        print(f"Total items moved: {len(items_to_move)}")
        print(f"Destination directory: {destination_dir}")

        return len(items_to_move) > 0

    @staticmethod
    def advanced_storage_options(default_location=None):
        """
        Advanced storage management with multiple options
        """
        while True:
            options = [
                inquirer.List(
                    'storage_action',
                    message="Select advanced storage action",
                    choices=[
                        'Copy Multiple Files/Folders',
                        'Move Files/Folders', 
                        'Backup Current Directory',
                        'View Directory Contents',
                        'Return to Main Menu'
                    ]
                )
            ]
            
            # Prompt for storage action
            result = inquirer.prompt(options)
            action = result['storage_action']
            
            # Handle different storage actions
            if action == 'Copy Multiple Files/Folders':
                AdvancedDataOperations.copy_multiple_items(default_location)
            elif action == 'Move Files/Folders':
                AdvancedDataOperations.move_items(default_location)
            elif action == 'Backup Current Directory':
                AdvancedFileOperations.backup_directory()
            elif action == 'View Directory Contents':
                AdvancedFileOperations.view_directory_contents()
            elif action == 'Return to Main Menu':
                print("Returning to main menu...")
                break
            else:
                print("Invalid selection. Please try again")

# Add this to ensure the module can be imported
if __name__ == "__main__":
    pass
