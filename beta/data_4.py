#!/usr/bin/python3

import os
import inquirer
from drop_error import FileDropHandler
from data_2 import AdvancedDataOperations
from data_3 import AdvancedFileOperations

class DataDropManager:
    @staticmethod
    def handle_file_drop(max_attempts=3):
        """
        Interactively handle file/folder dropping with multiple attempts
        
        Args:
            max_attempts (int): Maximum number of file selection attempts
        
        Returns:
            bool: Whether file/folder was successfully processed
        """
        print("File/Folder Drop Mode")
        
        # Destination directory selection
        destination_prompt = inquirer.confirm(
            "Do you want to specify a destination directory?", 
            default=False
        )

        destination_dir = None
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

        attempts = 0
        items_dropped = []
        
        while attempts < max_attempts:
            # Prepare file path input questions
            questions = [
                inquirer.Text(
                    'file_path', 
                    message="Enter the full path of the file/folder to copy (drag and drop supported)",
                    validate=lambda answers, current: os.path.exists(
                        AdvancedDataOperations.sanitize_path(current)
                    )
                ),
                inquirer.Confirm(
                    'confirm', 
                    message="Do you want to copy this file/folder?", 
                    default=True
                )
            ]
            
            try:
                # Prompt for file input
                answers = inquirer.prompt(questions)
                
                # Handle user cancellation
                if not answers or not answers['confirm']:
                    print("File/Folder copy cancelled.")
                    break
                
                # Sanitize and validate path
                source_path = AdvancedDataOperations.sanitize_path(answers['file_path'])
                
                # Determine if it's a file or directory
                if os.path.isdir(source_path):
                    # Folder copy
                    try:
                        unique_dest = AdvancedFileOperations.get_unique_destination_path(
                            destination_dir, 
                            source_path
                        )
                        
                        shutil.copytree(source_path, unique_dest)
                        print(f"Successfully copied folder: {source_path}")
                        print(f"Destination: {unique_dest}")
                        items_dropped.append(source_path)
                    except Exception as e:
                        print(f"Error copying folder: {e}")
                        attempts += 1
                else:
                    # File copy
                    try:
                        unique_dest = AdvancedFileOperations.get_unique_destination_path(
                            destination_dir, 
                            source_path
                        )
                        
                        shutil.copy2(source_path, unique_dest)
                        print(f"Successfully copied file: {source_path}")
                        print(f"Destination: {unique_dest}")
                        items_dropped.append(source_path)
                    except Exception as e:
                        print(f"Error copying file: {e}")
                        attempts += 1
                
                # Ask if user wants to continue dropping files
                continue_prompt = inquirer.confirm(
                    "Do you want to copy another file or folder?", 
                    default=True
                )
                
                if not continue_prompt:
                    break
            
            except KeyboardInterrupt:
                print("\nFile/Folder selection interrupted.")
                break
            except Exception as e:
                print(f"Unexpected error during file/folder selection: {e}")
                attempts += 1
        
        # Summary of dropped items
        print("\nFile/Folder Drop Summary:")
        print(f"Total items copied: {len(items_dropped)}")
        print(f"Destination directory: {destination_dir}")
        
        return len(items_dropped) > 0

# Add a method to get unique destination path
def get_unique_destination_path(destination_dir, source_path):
    """
    Generate a unique destination path to prevent overwriting
    """
    import os
    
    base_name = os.path.basename(source_path)
    unique_path = os.path.join(destination_dir, base_name)
    counter = 1
    
    while os.path.exists(unique_path):
        # If file exists, add a counter
        name, ext = os.path.splitext(base_name)
        unique_path = os.path.join(destination_dir, f"{name}_{counter}{ext}")
        counter += 1
    
    return unique_path

# Main execution for testing
def main():
    print("DataDropManager Module")
    DataDropManager.handle_file_drop()

if __name__ == "__main__":
    main()
