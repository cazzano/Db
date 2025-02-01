package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
)

// Recent is the refactored main function from recent.go
func Recent() {
	// Expand the path to the config file
	configPath := filepath.Join(os.Getenv("HOME"), ".config", "database", "data_base.json")

	// Read the config file
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
		fmt.Println("No folders found in the configuration.")
		return
	}

	// Create a slice to store folders for sorting
	folderList := make([]struct {
		Name   string
		Folder Folder
	}, 0, len(config.Folders))

	// Convert map to slice for sorting
	for name, folder := range config.Folders {
		folderList = append(folderList, struct {
			Name   string
			Folder Folder
		}{Name: name, Folder: folder})
	}

	// Sort folders by creation time (most recent first)
	sort.Slice(folderList, func(i, j int) bool {
		return folderList[i].Folder.CreatedAt.After(folderList[j].Folder.CreatedAt)
	})

	// Print header
	fmt.Println("Recently Created Folders:")
	fmt.Println("------------------------")

	// Print folder details
	for i, folder := range folderList {
		fmt.Printf("%d. Folder Name: %s\n", i+1, folder.Name)
		fmt.Printf("   Path: %s\n", folder.Folder.Path)
		fmt.Printf("   Created At: %s\n", folder.Folder.CreatedAt.Format("2006-01-02 15:04:05"))

		if folder.Folder.Description != "" {
			fmt.Printf("   Description: %s\n", folder.Folder.Description)
		}
		fmt.Println()
	}

	// Print total folder count
	fmt.Printf("Total Folders: %d\n", len(config.Folders))
}
