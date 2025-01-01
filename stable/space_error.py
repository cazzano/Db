# space_error.py
import os
import shutil
import re

class FilePathHandler:
    @staticmethod
    def clean_path(path):
        """
        Comprehensive path cleaning method
        """
        # Remove surrounding quotes (single or double)
        path = path.strip("'\"")
        
        # Expand user home directory
        path = os.path.expanduser(path)
        
        # Normalize path (resolve symlinks, remove redundant separators)
        path = os.path.realpath(path)
        
        return path

    @staticmethod
    def validate_file_path(path):
        """
        Comprehensive file path validation
        """
        try:
            # Clean the path
            cleaned_path = FilePathHandler.clean_path(path)
            
            # Detailed path checks
            if not os.path.exists(cleaned_path):
                print(f"Error: Path does not exist - {cleaned_path}")
                return None
            
            if not os.path.isfile(cleaned_path):
                print(f"Error: Not a file - {cleaned_path}")
                return None
            
            if not os.access(cleaned_path, os.R_OK):
                print(f"Error: No read permission - {cleaned_path}")
                return None
            
            return cleaned_path
        
        except Exception as e:
            print(f"Error processing path: {e}")
            return None

    @staticmethod
    def safe_copy_file(source_path, destination_dir=None):
        """
        Safe file copying with comprehensive error handling
        """
        try:
            # Validate source path
            validated_source = FilePathHandler.validate_file_path(source_path)
            
            if not validated_source:
                return False
            
            # Determine destination directory
            if destination_dir is None:
                destination_dir = os.getcwd()
            
            # Get filename and sanitize
            filename = FilePathHandler.sanitize_filename(
                os.path.basename(validated_source)
            )
            
            # Create full destination path
            destination_path = os.path.join(destination_dir, filename)
            
            # Handle file overwrite
            if os.path.exists(destination_path):
                import inquirer
                overwrite = inquirer.confirm(
                    f"File {filename} already exists. Overwrite?", 
                    default=False
                )
                if not overwrite:
                    print("File copy cancelled.")
                    return False
            
            # Perform file copy
            shutil.copy2(validated_source, destination_path)
            print(f"File '{filename}' copied successfully to {destination_dir}!")
            return True
        
        except Exception as e:
            print(f"Unexpected error during file copy: {e}")
            return False

    @staticmethod
    def sanitize_filename(filename):
        """
        Sanitize filename to handle special characters
        """
        # Remove or replace potentially problematic characters
        sanitized = re.sub(r'[<>:"/\\|?*]', '_', filename)
        
        # Trim leading/trailing spaces and dots
        sanitized = sanitized.strip('. ')
        
        # Limit filename length
        max_length = 255
        if len(sanitized) > max_length:
            sanitized = sanitized[:max_length]
        
        # Ensure non-empty filename
        if not sanitized:
            sanitized = 'unnamed_file'
        
        return sanitized
