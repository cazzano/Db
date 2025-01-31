package main

import (
	"bufio"
	"fmt"
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

	// Ask user whether to copy or move
	fmt.Println("Choose an action:")
	fmt.Println("1. Copy")
	fmt.Println("2. Move")
	fmt.Print("Enter your choice (1/2): ")
	var choice string
	fmt.Scanln(&choice)

	// Get the base name of the source
	baseName := filepath.Base(sourcePath)
	destPath := filepath.Join(currentDir, baseName)

	// Perform action based on user choice
	if choice == "1" || choice == "copy" {
		// Copy
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
	} else if choice == "2" || choice == "move" {
		// Move
		if fileInfo.IsDir() {
			err = moveDir(sourcePath, destPath)
			if err != nil {
				fmt.Println("Error moving directory:", err)
				return
			}
			fmt.Printf("Directory '%s' moved successfully to current directory\n", baseName)
		} else {
			err = moveFile(sourcePath, destPath)
			if err != nil {
				fmt.Println("Error moving file:", err)
				return
			}
			fmt.Printf("File '%s' moved successfully to current directory\n", baseName)
		}
	} else {
		fmt.Println("Invalid choice. Please choose 1 for copy or 2 for move.")
	}
}
