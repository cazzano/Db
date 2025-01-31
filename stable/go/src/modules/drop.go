package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"os/user"
	"path/filepath"
	"strings"
)

// expandUserPath expands the ~ to the user's home directory
func expandUserPath(path string) string {
	if !strings.HasPrefix(path, "~") {
		return path
	}

	currentUser, err := user.Current()
	if err != nil {
		return path
	}

	homeDir := currentUser.HomeDir
	return filepath.Join(homeDir, path[2:])
}

// sanitizePath cleans and resolves the path
func sanitizePath(path string) string {
	// Remove quotes if present
	path = strings.Trim(path, "\"'")

	// Expand user path (e.g., ~/Documents)
	expandedPath := expandUserPath(path)

	// Clean the path (remove redundant separators, resolve ..)
	cleanPath := filepath.Clean(expandedPath)

	return cleanPath
}

func copyFile(src, dst string) error {
	sourceFile, err := os.Open(src)
	if err != nil {
		return fmt.Errorf("error opening source file: %v", err)
	}
	defer sourceFile.Close()

	// Check if file already exists in destination
	if _, err := os.Stat(dst); err == nil {
		fmt.Printf("File '%s' already exists. Overwrite? (y/n): ", filepath.Base(dst))
		var response string
		fmt.Scanln(&response)
		if strings.ToLower(response) != "y" {
			return fmt.Errorf("file copy cancelled")
		}
	}

	destFile, err := os.Create(dst)
	if err != nil {
		return fmt.Errorf("error creating destination file: %v", err)
	}
	defer destFile.Close()

	_, err = io.Copy(destFile, sourceFile)
	if err != nil {
		return fmt.Errorf("error copying file: %v", err)
	}

	// Preserve file permissions
	sourceInfo, err := os.Stat(src)
	if err == nil {
		err = os.Chmod(dst, sourceInfo.Mode())
		if err != nil {
			fmt.Printf("Warning: Could not preserve file permissions: %v\n", err)
		}
	}

	return nil
}

func copyDir(src, dst string) error {
	// Check if directory already exists
	if _, err := os.Stat(dst); err == nil {
		fmt.Printf("Directory '%s' already exists. Overwrite? (y/n): ", filepath.Base(dst))
		var response string
		fmt.Scanln(&response)
		if strings.ToLower(response) != "y" {
			return fmt.Errorf("directory copy cancelled")
		}
		// Remove existing directory
		os.RemoveAll(dst)
	}

	// Create destination directory
	err := os.MkdirAll(dst, 0755)
	if err != nil {
		return fmt.Errorf("error creating destination directory: %v", err)
	}

	// Walk through the source directory
	return filepath.Walk(src, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		// Get the relative path
		relativePath, err := filepath.Rel(src, path)
		if err != nil {
			return err
		}

		// Construct destination path
		destPath := filepath.Join(dst, relativePath)

		// If it's a directory, create it
		if info.IsDir() {
			return os.MkdirAll(destPath, 0755)
		}

		// If it's a file, copy it
		return copyFile(path, destPath)
	})
}

func main() {
	// Support multiple ways of getting the path
	var sourcePath string

	// Check command-line arguments first
	if len(os.Args) > 1 {
		sourcePath = strings.Join(os.Args[1:], " ")
	} else {
		// Prompt user for file or folder path
		fmt.Println("Drop your file or folder here (enter full path):")

		// Use bufio to handle paths with spaces
		reader := bufio.NewReader(os.Stdin)
		sourcePath, _ = reader.ReadString('\n')
		sourcePath = strings.TrimSpace(sourcePath)
	}

	// Sanitize the path
	sourcePath = sanitizePath(sourcePath)

	// Get current working directory
	currentDir, err := os.Getwd()
	if err != nil {
		fmt.Println("Error getting current directory:", err)
		return
	}

	// Check if source path exists
	fileInfo, err := os.Stat(sourcePath)
	if err != nil {
		fmt.Printf("Error accessing file/folder: %v\n", err)
		fmt.Println("Please check the path and ensure it exists.")
		return
	}

	// Get the base name of the source
	baseName := filepath.Base(sourcePath)
	destPath := filepath.Join(currentDir, baseName)

	// Copy file or directory
	if fileInfo.IsDir() {
		err = copyDir(sourcePath, destPath)
		if err != nil {
			fmt.Println("Error copying directory:", err)
			return
		}
		fmt.Printf("Directory '%s' copied successfully to current directory\n", baseName)
	} else {
		err = copyFile(sourcePath, destPath)
		if err != nil {
			fmt.Println("Error copying file:", err)
			return
		}
		fmt.Printf("File '%s' copied successfully to current directory\n", baseName)
	}
}
