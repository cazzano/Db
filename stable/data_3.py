#!/usr/bin/python3

import os
import shutil
import inquirer
from datetime import datetime

class AdvancedFileOperations:
    @staticmethod
    def sanitize_path(path):
        """
        Sanitize and clean file/folder paths
        """
        return path.strip().strip("'\"")

    @staticmethod
    def get_directory_size(directory):
        """
        Calculate total size of a directory
        """
        total_size = 0
        for dirpath, dirnames, filenames in os.walk(directory):
            for f in filenames:
                fp = os.path.join(dirpath, f)
                if not os.path.islink(fp):
                    total_size += os.path.getsize(fp)
        return total_size

    @staticmethod
    def backup_directory(source_dir=None):
        """
        Backup directory with comprehensive timestamp and error handling
        
        Args:
            source_dir (str, optional): Directory to backup. Defaults to current directory.
        
        Returns:
            bool: Backup success status
        """
        # Use current directory if no source specified
        current_dir = source_dir if source_dir else os.getcwd()
        
        # Generate unique backup directory name with timestamp
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        backup_dir = f"{current_dir}_backup_{timestamp}"
        
        try:
            # Ensure backup directory doesn't already exist
            counter = 1
            original_backup_dir = backup_dir
            while os.path.exists(backup_dir):
                backup_dir = f"{original_backup_dir}_{counter}"
                counter += 1
            
            # Perform backup
            shutil.copytree(current_dir, backup_dir)
            
            # Verify backup
            if os.path.exists(backup_dir):
                backup_size = AdvancedFileOperations.get_directory_size(backup_dir)
                print(f"Directory successfully backed up to: {backup_dir}")
                print(f"Backup size: {backup_size} bytes")
                return True
            else:
                print("Backup verification failed.")
                return False
        
        except PermissionError:
            print("Error: Insufficient permissions to create backup.")
            return False
        except Exception as e:
            print(f"Backup failed: {e}")
            return False

    @staticmethod
    def view_directory_contents(directory=None):
        """
        View contents of a directory with comprehensive details
        
        Args:
            directory (str, optional): Directory to view. Defaults to current directory.
        """
        # Use current directory if no directory specified
        current_dir = directory if directory else os.getcwd()
        
        print(f"Contents of {current_dir}:")
        
        try:
            for item in os.listdir(current_dir):
                item_path = os.path.join(current_dir, item)
                
                # Determine item type
                if os.path.isdir(item_path):
                    item_type = "Directory"
                    item_size = "N/A"
                else:
                    item_type = "File"
                    item_size = os.path.getsize(item_path)
                
                # Print item details
                print(f"{item} - Type: {item_type}, Size: {item_size} bytes")
        
        except Exception as e:
            print(f"Error viewing directory contents: {e}")

    @staticmethod
    def interactive_file_selection(message="Select files/folders"):
        """
        Interactive file and folder selection
        
        Args:
            message (str, optional): Prompt message for selection
        
        Returns:
            list: Selected file/folder paths
        """
        selected_items = []
        
        while True:
            continue_prompt = inquirer.confirm(
                f"{message}. Do you want to add an item?", 
                default=True
            )
            
            if not continue_prompt:
                break
            
            # Path selection
            path_questions = [
                inquirer.Text(
                    'path', 
                    message="Enter the full path of the file or folder",
                    validate=lambda answers, current: os.path.exists(
                        AdvancedFileOperations.sanitize_path(current)
                    )
                )
            ]
            path_answers = inquirer.prompt(path_questions)
            source_path = AdvancedFileOperations.sanitize_path(path_answers['path'])
            
            selected_items.append(source_path)
        
        return selected_items

def main():
    """
    Demonstration of advanced file operations
    """
    print("Advanced File Operations Module")
    
    # Example usage
    print("\nDirectory Contents:")
    AdvancedFileOperations.view_directory_contents()
    
    print("\nInteractive File Selection:")
    selected_items = AdvancedFileOperations.interactive_file_selection()
    print("Selected Items:", selected_items)

if __name__ == "__main__":
    main()
