# progress.py
import os
import shutil
import time
from tqdm import tqdm

class FileProgressTracker:
    @staticmethod
    def copy_with_progress(source, destination):
        """
        Copy file with detailed progress tracking
        
        Args:
            source (str): Source file path
            destination (str): Destination file path
        
        Returns:
            bool: Successful copy status
        """
        try:
            # Get file size
            file_size = os.path.getsize(source)
            
            # Open source and destination files
            with open(source, 'rb') as src_file, \
                 open(destination, 'wb') as dest_file, \
                 tqdm(total=file_size, 
                      unit='B', 
                      unit_scale=True, 
                      desc=f"Copying {os.path.basename(source)}",
                      ascii=True) as progress_bar:
                
                # Copy file with progress tracking
                while True:
                    buffer = src_file.read(1024 * 1024)  # 1MB chunks
                    if not buffer:
                        break
                    dest_file.write(buffer)
                    progress_bar.update(len(buffer))
            
            # Preserve file metadata
            shutil.copystat(source, destination)
            
            return True
        
        except Exception as e:
            print(f"Progress tracking error: {e}")
            return False

    @staticmethod
    def advanced_copy_with_details(source, destination):
        """
        Advanced file copy with comprehensive details
        
        Args:
            source (str): Source file path
            destination (str): Destination file path
        
        Returns:
            dict: Copy operation details
        """
        start_time = time.time()
        
        try:
            # Validate source file
            if not os.path.exists(source):
                raise FileNotFoundError(f"Source file not found: {source}")
            
            # Get file details
            file_size = os.path.getsize(source)
            file_name = os.path.basename(source)
            
            # Copy with progress
            copy_success = FileProgressTracker.copy_with_progress(source, destination)
            
            # Calculate metrics
            end_time = time.time()
            duration = end_time - start_time
            transfer_speed = file_size / duration if duration > 0 else 0
            
            return {
                'success': copy_success,
                'file_name': file_name,
                'file_size': file_size,
                'duration': duration,
                'transfer_speed': transfer_speed
            }
        
        except Exception as e:
            print(f"Advanced copy error: {e}")
            return {
                'success': False,
                'error': str(e)
            }

    @staticmethod
    def generate_copy_report(copy_details):
        """
        Generate detailed copy operation report
        
        Args:
            copy_details (dict): Copy operation details
        """
        if not copy_details['success']:
            print("File copy failed.")
            return
        
        print("\n--- File Copy Report ---")
        print(f"File: {copy_details['file_name']}")
        print(f"Size: {copy_details['file_size'] / 1024:.2f} KB")
        print(f"Duration: {copy_details['duration']:.2f} seconds")
        print(f"Transfer Speed: {copy_details['transfer_speed'] / 1024:.2f} KB/s")
        print("----------------------")
