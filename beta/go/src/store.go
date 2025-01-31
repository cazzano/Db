package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type Config struct {
	Location string            `json:"location"`
	Folders  map[string]Folder `json:"folders"`
}

type Folder struct {
	Path        string    `json:"path"`
	CreatedAt   time.Time `json:"created_at"`
	Description string    `json:"description,omitempty"`
}

func createFolder() error {
	// Expand the path to the config file
	configPath := filepath.Join(os.Getenv("HOME"), ".config", "database", "data_base.json")

	// Read the config file
	configData, err := os.ReadFile(configPath)
	if err != nil {
		fmt.Printf("Error reading config file: %v\n", err)
		// Initialize a new config if file doesn't exist
		configData = []byte("{\"location\":\"\", \"folders\":{}}")
	}

	// Parse the JSON config
	var config Config
	err = json.Unmarshal(configData, &config)
	if err != nil {
		return fmt.Errorf("error parsing config file: %v", err)
	}

	// Ensure folders map is initialized
	if config.Folders == nil {
		config.Folders = make(map[string]Folder)
	}

	// Print the base location
	fmt.Printf("Base location from config: %s\n", config.Location)

	// Create a reader for input
	reader := bufio.NewReader(os.Stdin)

	// Prompt for folder name
	fmt.Print("Enter the name of the folder you want to create: ")
	folderName, err := reader.ReadString('\n')
	if err != nil {
		return fmt.Errorf("error reading input: %v", err)
	}
	// Trim whitespace and newline
	folderName = strings.TrimSpace(folderName)

	// Validate folder name
	if folderName == "" {
		return fmt.Errorf("folder name cannot be empty")
	}

	// Prompt for folder description (optional)
	fmt.Print("Enter a description for the folder (optional): ")
	description, err := reader.ReadString('\n')
	if err != nil {
		return fmt.Errorf("error reading description: %v", err)
	}
	description = strings.TrimSpace(description)

	// Construct full path
	fullPath := filepath.Join(config.Location, folderName)

	// Confirm folder creation
	fmt.Printf("Do you want to create folder '%s' at '%s'? (yes/no): ", folderName, fullPath)
	var response string
	fmt.Scanln(&response)

	// Check user response
	if response == "yes" || response == "y" {
		// Check if folder already exists
		if _, err := os.Stat(fullPath); !os.IsNotExist(err) {
			fmt.Printf("Folder '%s' already exists. Overwrite? (yes/no): ", folderName)
			fmt.Scanln(&response)
			if response != "yes" && response != "y" {
				fmt.Println("Folder creation cancelled.")
				return nil
			}
		}

		// Create the directory
		err = os.MkdirAll(fullPath, 0755)
		if err != nil {
			return fmt.Errorf("error creating directory: %v", err)
		}

		// Create folder entry in config
		folderEntry := Folder{
			Path:        fullPath,
			CreatedAt:   time.Now(),
			Description: description,
		}
		config.Folders[folderName] = folderEntry

		// Write updated config back to file
		updatedConfigData, err := json.MarshalIndent(config, "", "  ")
		if err != nil {
			return fmt.Errorf("error preparing config data: %v", err)
		}

		err = os.WriteFile(configPath, updatedConfigData, 0644)
		if err != nil {
			return fmt.Errorf("error updating config file: %v", err)
		}

		fmt.Printf("Folder '%s' created successfully at: %s\n", folderName, fullPath)
		fmt.Println("Folder information saved to configuration.")
	} else {
		fmt.Println("Folder creation cancelled.")
	}

	return nil
}
