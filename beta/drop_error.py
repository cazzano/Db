# drop_error.py
import os
import shutil
import re
from progress import FileProgressTracker

class FileDropHandler:
    @staticmethod
    def clean_file_path(file_path):
        """
        Clean and sanitize file path input
        
        Args:
            file_path (str): Raw file path input
        
        Returns:
            str: Cleaned and normalized file path
        """
        # Remove leading/trailing whitespaces
        cleaned_path = file_path.strip()
        
        # Handle paths with quotes
        cleaned_path = cleaned_path.strip("'\"")
        
        # Normalize path separators
        cleaned_path = cleaned_path.replace('\\', '/')
        
        # Expand user home directory
        cleaned_path = os.path.expanduser(cleaned_path)
        
        # Resolve any relative path references
        cleaned_path = os.path.abspath(cleaned_path)
        
        return cleaned_path

    @staticmethod
    def validate_file_path(file_path):
        """
        Validate file path with comprehensive checks
        
        Args:
            file_path (str): Cleaned file path
        
        Returns:
            bool: Whether the file path is valid
        """
        try:
            # Check if path exists
            if not os.path.exists(file_path):
                print(f"Error: File path does not exist - {file_path}")
                return False
            
            # Check if it's a file (not directory)
            if not os.path.isfile(file_path):
                print(f"Error: Path is not a file - {file_path}")
                return False
            
            # Check file readability
            if not os.access(file_path, os.R_OK):
                print(f"Error: No read permissions for file - {file_path}")
                return False
            
            return True
        except Exception as e:
            print(f"Validation error: {e}")
            return False

    @staticmethod
    def safe_file_copy(source_path, destination_dir=None):
        """
        Safely copy file with advanced handling and progress tracking
        
        Args:
            source_path (str): Source file path
            destination_dir (str, optional): Destination directory
        
        Returns:
            bool: Copy success status
        """
        try:
            # Clean source path
            cleaned_source = FileDropHandler.clean_file_path(source_path)
            
            # Validate source file
            if not FileDropHandler.validate_file_path(cleaned_source):
                return False
            
            # Use current directory if no destination specified
            if destination_dir is None:
                destination_dir = os.getcwd()
            
            # Ensure destination directory exists
            os.makedirs(destination_dir, exist_ok=True)
            
            # Generate destination filename
            filename = os.path.basename(cleaned_source)
            destination_path = os.path.join(destination_dir, filename)
            
            # Use progress tracker for advanced copying
            copy_details = FileProgressTracker.advanced_copy_with_details(
                cleaned_source, 
                destination_path
            )
            
            # Generate and display copy report
            FileProgressTracker.generate_copy_report(copy_details)
            
            return copy_details['success']
        
        except PermissionError:
            print("Error: Insufficient permissions to copy file")
        except shutil.SameFileError:
            print("Error: Source and destination are the same")
        except Exception as e:
            print(f"File copy error: {e}")
        
        return False

    @staticmethod
    def extract_file_paths(input_text):
        """
        Extract multiple file paths from complex input
        
        Args:
            input_text (str): Raw input text
        
        Returns:
            list: Cleaned file paths
        """
        # Regular expression to match file paths
        path_pattern = r'(?:\/|[a-zA-Z]:\\)[\w\-\. /\\]+(?:\.\w+)?'
        
        # Find all potential file paths
        paths = re.findall(path_pattern, input_text)
        
        # Clean and validate paths
        valid_paths = [
            path for path in paths 
            if FileDropHandler.validate_file_path(
                FileDropHandler.clean_file_path(path)
            )
        ]
        
        return valid_paths

    @staticmethod
    def move_file(source_path, destination_dir=None):
        """
        Move file with progress tracking
        
        Args:
            source_path (str): Source file path
            destination_dir (str, optional): Destination directory
        
        Returns:
            bool: Move success status
        """
        try:
            # Clean and validate source path
            cleaned_source = FileDropHandler.clean_file_path(source_path)
            
            if not FileDropHandler.validate_file_path(cleaned_source):
                return False
            
            # Use current directory if no destination specified
            if destination_dir is None:
                destination_dir = os.getcwd()
            
            # Ensure destination directory exists
            os.makedirs(destination_dir, exist_ok=True)
            
            # Generate destination filename
            filename = os.path.basename(cleaned_source)
            destination_path = os.path.join(destination_dir, filename)
            
            # Copy file with progress
            copy_details = FileProgressTracker.advanced_copy_with_details(
                cleaned_source, 
                destination_path
            )
            
            # Generate and display copy report
            FileProgressTracker.generate_copy_report(copy_details)
            
            # If copy successful, remove source file
            if copy_details['success']:
                os.remove(cleaned_source)
                print(f"File moved successfully: {filename}")
                return True
            
            return False
        
        except Exception as e:
            print(f"File move error: {e}")
            return False
