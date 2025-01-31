package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type Config struct {
	Location string `json:"location"`
}

func main() {
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

	// Print the base location
	fmt.Printf("Base location from config: %s\n", config.Location)

	// Create a reader for input
	reader := bufio.NewReader(os.Stdin)

	// Prompt for folder name
	fmt.Print("Enter the name of the folder you want to create: ")
	folderName, err := reader.ReadString('\n')
	if err != nil {
		fmt.Printf("Error reading input: %v\n", err)
		return
	}
	// Trim whitespace and newline
	folderName = strings.TrimSpace(folderName)

	// Validate folder name
	if folderName == "" {
		fmt.Println("Folder name cannot be empty.")
		return
	}

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
				return
			}
		}

		// Create the directory
		err = os.MkdirAll(fullPath, 0755)
		if err != nil {
			fmt.Printf("Error creating directory: %v\n", err)
			return
		}
		fmt.Printf("Folder '%s' created successfully at: %s\n", folderName, fullPath)
	} else {
		fmt.Println("Folder creation cancelled.")
	}
}
