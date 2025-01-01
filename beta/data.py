#!/usr/bin/python3

import os
import inquirer
from drop_error import FileDropHandler
from data_2 import AdvancedDataOperations
from data_3 import AdvancedFileOperations
from data_4 import DataDropManager

class DataManager:
    @staticmethod
    def create_folder():
        """
        Interactively create a folder with advanced validation
        """
        while True:
            # Interactive folder name input
            questions = [
                inquirer.Text(
                    'folder_name', 
                    message="Enter folder name to create",
                    validate=lambda answers, current: len(current.strip()) > 0
                ),
                inquirer.Confirm(
                    'confirm', 
                    message="Do you want to create this folder?", 
                    default=True
                )
            ]
            
            try:
                answers = inquirer.prompt(questions)
                
                # Handle user cancellation
                if not answers or not answers['confirm']:
                    print("Folder creation cancelled.")
                    return None
                
                # Sanitize folder name
                folder_name = AdvancedDataOperations.sanitize_path(answers['folder_name'])
                
                # Prevent creating folders in restricted locations
                current_dir = os.getcwd()
                full_path = os.path.join(current_dir, folder_name)
                
                # Create folder with advanced error handling
                os.makedirs(full_path, exist_ok=True)
                
                # Verify folder creation
                if os.path.isdir(full_path):
                    print(f"Folder '{folder_name}' created successfully!")
                    return full_path
                else:
                    print("Error: Unable to create folder.")
                    return None
            
            except PermissionError:
                print("Error: Insufficient permissions to create folder.")
            except OSError as e:
                print(f"Folder creation error: {e}")
            except Exception as e:
                print(f"Unexpected error: {e}")

    @staticmethod
    def process_storage_request():
        """
        Comprehensive storage management process
        """
        print("Storage Management Initiated")
        
        # Folder creation workflow
        create_folder_prompt = inquirer.confirm(
            "Do you want to create a new folder?", 
            default=False
        )
        
        if create_folder_prompt:
            folder = DataManager.create_folder()
            
            if folder:
                # Change to the newly created directory
                try:
                    os.chdir(folder)
                    print(f"Working directory changed to: {os.getcwd()}")
                except Exception as e:
                    print(f"Error changing directory: {e}")
        
        # File/Folder drop workflow
        drop_item_prompt = inquirer.confirm(
            "Do you want to drop a file or folder?", 
            default=False
        )
        
        if drop_item_prompt:
            file_drop_result = DataDropManager.handle_file_drop()
            
            if file_drop_result:
                print("File/Folder successfully processed.")
            else:
                print("File/Folder processing unsuccessful.")

    @staticmethod
    def handle_file_drop(max_attempts=3):
        """
        Delegate to DataDropManager
        """
        return DataDropManager.handle_file_drop(max_attempts)

    @staticmethod
    def advanced_storage_options():
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
                AdvancedDataOperations.copy_multiple_items()
            elif action == 'Move Files/Folders':
                AdvancedDataOperations.move_items()
            elif action == 'Backup Current Directory':
                AdvancedFileOperations.backup_directory()
            elif action == 'View Directory Contents':
                AdvancedFileOperations.view_directory_contents()
            elif action == 'Return to Main Menu':
                print("Returning to main menu...")
                break
            else:
                print("Invalid selection. Please try again.")

def main():
    """
    Demonstration of DataManager capabilities
    """
    print("DataManager Module")
    
    # Example usage
    print("\nStorage Options:")
    DataManager.advanced_storage_options()

if __name__ == "__main__":
    main()
