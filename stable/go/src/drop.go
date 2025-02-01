package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// Drop is the refactored main function from drop.go
func Drop() {
	// Read the configuration file
	configPath := filepath.Join(os.Getenv("HOME"), ".config", "database", "data_base.json")
	configData, err := os.ReadFile(configPath)
	if err != nil {
		fmt.Printf("Error reading config file: %v\n", err)
		return
	}

	// Parse the JSON config
	var config Config
	err = json.Unmarshal(configData, &config)
	if err != nil {
		fmt.Printf("Error parsing config file: %v\n", err)
		return
	}

	// Check if there are any folders
	if len(config.Folders) == 0 {
		fmt.Println("No folders found in the configuration. Please create a folder first.")
		return
	}

	// List available folders
	fmt.Println("Available folders:")
	folderList := make([]string, 0, len(config.Folders))
	for name := range config.Folders {
		folderList = append(folderList, name)
		fmt.Printf("- %s\n", name)
	}

	// Prompt user to select a folder
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter the name of the folder where you want to store the file/folder: ")
	folderName, err := reader.ReadString('\n')
	if err != nil {
		fmt.Printf("Error reading input: %v\n", err)
		return
	}
	folderName = strings.TrimSpace(folderName)

	// Validate folder name
	if _, exists := config.Folders[folderName]; !exists {
		fmt.Printf("Folder '%s' does not exist.\n", folderName)
		return
	}

	// Get the destination folder path
	destinationFolder := config.Folders[folderName].Path

	// Prompt user for the source file/folder path
	fmt.Print("Enter the full path of the file or folder you want to drop: ")
	sourcePath, err := reader.ReadString('\n')
	if err != nil {
		fmt.Printf("Error reading input: %v\n", err)
		return
	}
	sourcePath = strings.TrimSpace(sourcePath)

	// Sanitize the source path
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
	destPath := filepath.Join(destinationFolder, baseName)

	// Perform action based on user choice
	if choice == "1" || choice == "copy" {
		// Copy
		if fileInfo.IsDir() {
			err = copyDir(sourcePath, destPath)
			if err != nil {
				fmt.Println("Error copying directory:", err)
				return
			}
			fmt.Printf("Directory '%s' copied successfully to '%s'\n", baseName, destinationFolder)
		} else {
			err = copyFile(sourcePath, destPath)
			if err != nil {
				fmt.Println("Error copying file:", err)
				return
			}
			fmt.Printf("File '%s' copied successfully to '%s'\n", baseName, destinationFolder)
		}
	} else if choice == "2" || choice == "move" {
		// Move
		if fileInfo.IsDir() {
			err = moveDir(sourcePath, destPath)
			if err != nil {
				fmt.Println("Error moving directory:", err)
				return
			}
			fmt.Printf("Directory '%s' moved successfully to '%s'\n", baseName, destinationFolder)
		} else {
			err = moveFile(sourcePath, destPath)
			if err != nil {
				fmt.Println("Error moving file:", err)
				return
			}
			fmt.Printf("File '%s' moved successfully to '%s'\n", baseName, destinationFolder)
		}
	} else {
		fmt.Println("Invalid choice. Please choose 1 for copy or 2 for move.")
	}
}
