package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// Drop is the refactored main function from drop.go
func Drop() {
	// Support multiple ways of getting the path
	var sourcePath string

	// Prompt user for file or folder path
	fmt.Println("Drop your file or folder here (enter full path):")

	// Use bufio to handle paths with spaces
	reader := bufio.NewReader(os.Stdin)
	sourcePath, _ = reader.ReadString('\n')
	sourcePath = strings.TrimSpace(sourcePath)

	// Sanitize the path
	sourcePath = sanitizePath(sourcePath)

	// Ensure the source path is absolute
	if !filepath.IsAbs(sourcePath) {
		fmt.Println("Please provide an absolute path.")
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
	destPath := filepath.Join(".", baseName) // Use current directory (destination folder)

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
