package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"time"
)

func listRecentFolders() error {
	// Read the config file
	configPath := filepath.Join(os.Getenv("HOME"), ".config", "database", "data_base.json")

	// Read the config file
	configData, err := os.ReadFile(configPath)
	if err != nil {
		return fmt.Errorf("error reading config file: %v", err)
	}

	// Parse the JSON config
	var config Config
	err = json.Unmarshal(configData, &config)
	if err != nil {
		return fmt.Errorf("error parsing config file: %v", err)
	}

	// Check if there are any folders
	if len(config.Folders) == 0 {
		fmt.Println("No folders found in the configuration.")
		return nil
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
		fmt.Printf("%d. Name: %s, Path: %s, Created At: %s, Description: %s\n",
			i+1, folder.Name, folder.Folder.Path, folder.Folder.CreatedAt.Format(time.RFC1123), folder.Folder.Description)
	}

	return nil
}
