# folder_drop.py
import os
import shutil
import time
from tqdm import tqdm
from drop_error import FileDropHandler

class FolderDropHandler:
    @staticmethod
    def validate_folder_path(folder_path):
        """
        Validate folder path with comprehensive checks
        
        Args:
            folder_path (str): Path to folder
        
        Returns:
            bool: Whether the folder path is valid
        """
        try:
            # Check if path exists
            if not os.path.exists(folder_path):
                print(f"Error: Folder path does not exist - {folder_path}")
                return False
            
            # Check if it's a directory
            if not os.path.isdir(folder_path):
                print(f"Error: Path is not a directory - {folder_path}")
                return False
            
            # Check folder readability
            if not os.access(folder_path, os.R_OK):
                print(f"Error: No read permissions for folder - {folder_path}")
                return False
            
            return True
        except Exception as e:
            print(f"Folder validation error: {e}")
            return False

    @staticmethod
    def count_files_and_size(source_folder):
        """
        Count total files and calculate total size in a folder
        
        Args:
            source_folder (str): Source folder path
        
        Returns:
            tuple: (total files, total size in bytes)
        """
        total_files = 0
        total_size = 0
        
        for root, _, files in os.walk(source_folder):
            for file in files:
                file_path = os.path.join(root, file)
                total_files += 1
                total_size += os.path.getsize(file_path)
        
        return total_files, total_size

    @staticmethod
    def safe_folder_copy(source_folder, destination_dir=None):
        """
        Safely copy entire folder with progress tracking
        
        Args:
            source_folder (str): Source folder path
            destination_dir (str, optional): Destination directory
        
        Returns:
            dict: Copy operation details
        """
        start_time = time.time()
        
        try:
            # Clean and validate source folder
            cleaned_source = FileDropHandler.clean_file_path(source_folder)
            
            if not FolderDropHandler.validate_folder_path(cleaned_source):
                return {'success': False, 'error': 'Invalid folder path'}
            
            # Use current directory if no destination specified
            if destination_dir is None:
                destination_dir = os.getcwd()
            
            # Ensure destination directory exists
            os.makedirs(destination_dir, exist_ok=True)
            
            # Get folder name and full destination path
            folder_name = os.path.basename(cleaned_source)
            destination_path = os.path.join(destination_dir, folder_name)
            
            # Count total files and size
            total_files, total_size = FolderDropHandler.count_files_and_size(cleaned_source)
            
            # Create progress bar
            with tqdm(total=total_files, 
                      unit='file', 
                      desc=f"Copying folder: {folder_name}",
                      ascii=True) as progress_bar:
                
                # Copy entire directory tree
                def copy_with_progress(src, dst):
                    os.makedirs(dst, exist_ok=True)
                    for item in os.listdir(src):
                        s = os.path.join(src, item)
                        d = os.path.join(dst, item)
                        
                        if os.path.isdir(s):
                            copy_with_progress(s, d)
                        else:
                            shutil.copy2(s, d)
                            progress_bar.update(1)
                
                copy_with_progress(cleaned_source, destination_path)
            
            # Calculate metrics
            end_time = time.time()
            duration = end_time - start_time
            
            return {
                'success': True,
                'folder_name': folder_name,
                'total_files': total_files,
                'total_size': total_size,
                'duration': duration,
                'destination_path': destination_path
            }
        
        except Exception as e:
            print(f"Folder copy error: {e}")
            return {
                'success': False,
                'error': str(e)
            }

    @staticmethod
    def generate_folder_copy_report(copy_details):
        """
        Generate detailed folder copy operation report
        
        Args:
            copy_details (dict): Folder copy operation details
        """
        if not copy_details['success']:
            print("Folder copy failed.")
            return
        
        print("\n--- Folder Copy Report ---")
        print(f"Folder: {copy_details['folder_name']}")
        print(f"Total Files: {copy_details['total_files']}")
        print(f"Total Size: {copy_details['total_size'] / (1024 * 1024):.2f} MB")
        print(f"Duration: {copy_details['duration']:.2f} seconds")
        print(f"Destination: {copy_details['destination_path']}")
        print("------------------------")

    @staticmethod
    def move_folder(source_folder, destination_dir=None):
        """
        Move entire folder with progress tracking
        
        Args:
            source_folder (str): Source folder path
            destination_dir (str, optional): Destination directory
        
        Returns:
            bool: Move success status
        """
        try:
            # Copy folder
            copy_details = FolderDropHandler.safe_folder_copy(source_folder, destination_dir)
            
            # Generate and display copy report
            FolderDropHandler.generate_folder_copy_report(copy_details)
            
            # If copy successful, remove source folder
            if copy_details['success']:
                shutil.rmtree(FileDropHandler.clean_file_path(source_folder))
                print(f"Folder moved successfully: {copy_details['folder_name']}")
                return True
            
            return False
        
        except Exception as e:
            print(f"Folder move error: {e}")
            return False
