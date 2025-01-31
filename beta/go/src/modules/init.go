package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"
)

// DatabaseConfig represents the structure of the database configuration
type DatabaseConfig struct {
	DatabasePath string `json:"database_path"`
}

// expandHomePath expands the ~ to the user's home directory
func expandHomePath(path string) string {
	if strings.HasPrefix(path, "~/") {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			fmt.Println("Error getting home directory:", err)
			return path
		}
		return filepath.Join(homeDir, path[2:])
	}
	return path
}

// createConfigDirectory creates the necessary config directory
func createConfigDirectory() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return err
	}

	configDir := filepath.Join(homeDir, ".config", "database")
	return os.MkdirAll(configDir, 0755)
}

// saveConfigFile saves the database path to config.json
func saveConfigFile(path string) error {
	// Expand and clean the path
	cleanPath := expandHomePath(filepath.Clean(path))

	// Verify path exists
	if _, err := os.Stat(cleanPath); os.IsNotExist(err) {
		fmt.Println("Warning: Specified path does not exist.")
		fmt.Print("Do you want to create the directory? (y/n): ")
		var response string
		fmt.Scanln(&response)
		if strings.ToLower(response) == "y" {
			err = os.MkdirAll(cleanPath, 0755)
			if err != nil {
				return fmt.Errorf("failed to create directory: %v", err)
			}
		} else {
			return fmt.Errorf("directory not created")
		}
	}

	// Create config struct
	config := DatabaseConfig{
		DatabasePath: cleanPath,
	}

	// Convert to JSON
	configJSON, err := json.MarshalIndent(config, "", "  ")
	if err != nil {
		return err
	}

	// Create config directory if it doesn't exist
	err = createConfigDirectory()
	if err != nil {
		return err
	}

	// Define config file path
	homeDir, _ := os.UserHomeDir()
	configFilePath := filepath.Join(homeDir, ".config", "database", "config.json")

	// Write JSON to file
	err = ioutil.WriteFile(configFilePath, configJSON, 0644)
	if err != nil {
		return err
	}

	return nil
}

// readConfigFile reads the existing database configuration
func readConfigFile() (*DatabaseConfig, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return nil, err
	}

	configFilePath := filepath.Join(homeDir, ".config", "database", "config.json")

	// Check if config file exists
	if _, err := os.Stat(configFilePath); os.IsNotExist(err) {
		return nil, fmt.Errorf("config file does not exist")
	}

	// Read config file
	configData, err := ioutil.ReadFile(configFilePath)
	if err != nil {
		return nil, err
	}

	// Unmarshal JSON
	var config DatabaseConfig
	err = json.Unmarshal(configData, &config)
	if err != nil {
		return nil, err
	}

	return &config, nil
}

// promptForPath interactively gets the database path from user
func promptForPath() string {
	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print("Enter the full path for your database: ")
		path, _ := reader.ReadString('\n')
		path = strings.TrimSpace(path)

		// Validate path
		if path == "" {
			fmt.Println("Path cannot be empty. Please try again.")
			continue
		}

		// Expand and clean path
		cleanPath := expandHomePath(filepath.Clean(path))

		// Confirm path
		fmt.Printf("You entered: %s\n", cleanPath)
		fmt.Print("Is this correct? (y/n): ")
		var confirm string
		fmt.Scanln(&confirm)

		if strings.ToLower(confirm) == "y" {
			return cleanPath
		}
	}
}

func main() {
	fmt.Println("Database Path Configuration")
	fmt.Println("-------------------------")

	// Check if config already exists
	existingConfig, err := readConfigFile()
	if err == nil {
		fmt.Println("Existing database path:", existingConfig.DatabasePath)
		fmt.Print("Do you want to update the path? (y/n): ")
		var response string
		fmt.Scanln(&response)
		if strings.ToLower(response) != "y" {
			fmt.Println("Configuration unchanged.")
			return
		}
	}

	// Get database path
	databasePath := promptForPath()

	// Save configuration
	err = saveConfigFile(databasePath)
	if err != nil {
		fmt.Println("Error saving configuration:", err)
		return
	}

	fmt.Println("Database path successfully configured!")

	// Verify and display the saved path
	savedConfig, err := readConfigFile()
	if err != nil {
		fmt.Println("Error reading saved configuration:", err)
		return
	}
	fmt.Println("Saved database path:", savedConfig.DatabasePath)
}
