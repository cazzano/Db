package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type LocationData struct {
	Location string `json:"location"`
}

// Init is the refactored main function from init.go
func Init() {
	// Check for existing location
	currentLocation, err := getCurrentLocation()
	if err != nil {
		fmt.Println("Error checking existing location:", err)
		return
	}

	// If location exists, provide options
	if currentLocation != "" {
		fmt.Printf("Current location: %s\n", currentLocation)

		reader := bufio.NewReader(os.Stdin)
		fmt.Println("Choose an option:")
		fmt.Println("1. Update location")
		fmt.Println("2. Keep current location")
		fmt.Println("3. Exit")
		fmt.Print("Enter your choice (1-3): ")

		choice, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading input:", err)
			return
		}

		// Trim whitespace and convert to lowercase
		choice = strings.TrimSpace(choice)

		switch choice {
		case "1":
			// Proceed with updating location
			fmt.Print("Enter a new location (absolute path): ")
			input, err := reader.ReadString('\n')
			if err != nil {
				fmt.Println("Error reading input:", err)
				return
			}

			// Parse new location
			location, err := parseLocation(input)
			if err != nil {
				fmt.Println("Invalid location:", err)
				return
			}

			// Create folder if it doesn't exist
			location, err = createFolder(location)
			if err != nil {
				fmt.Println("Folder creation error:", err)
				return
			}

			// Save new location
			err = saveLocation(location)
			if err != nil {
				fmt.Println("Error saving location:", err)
				return
			}

			fmt.Printf("Location updated to '%s'\n", location)

		case "2":
			fmt.Println("Keeping current location.")
			return

		case "3":
			fmt.Println("Exiting.")
			return

		default:
			fmt.Println("Invalid choice. Exiting.")
			return
		}
	} else {
		// No existing location, prompt to add a new one
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("No existing location found. Enter a new location (absolute path): ")
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading input:", err)
			return
		}

		// Parse location
		location, err := parseLocation(input)
		if err != nil {
			fmt.Println("Invalid location:", err)
			return
		}

		// Create folder if it doesn't exist
		location, err = createFolder(location)
		if err != nil {
			fmt.Println("Folder creation error:", err)
			return
		}

		// Save location
		err = saveLocation(location)
		if err != nil {
			fmt.Println("Error saving location:", err)
			return
		}

		fmt.Printf("Location '%s' saved successfully\n", location)
	}
}

// getCurrentLocation retrieves the current location from the JSON config file
func getCurrentLocation() (string, error) {
	// Construct the path to the database file
	configDir := filepath.Join(os.Getenv("HOME"), ".config", "database")
	filePath := filepath.Join(configDir, "data_base.json")

	// Check if the file exists
	_, err := os.Stat(filePath)
	if os.IsNotExist(err) {
		return "", nil // No existing location
	} else if err != nil {
		return "", fmt.Errorf("error checking database file: %v", err)
	}

	// Read the existing JSON file
	file, err := os.Open(filePath)
	if err != nil {
		return "", fmt.Errorf("error opening database file: %v", err)
	}
	defer file.Close()

	var existingData LocationData
	decoder := json.NewDecoder(file)
	err = decoder.Decode(&existingData)
	if err != nil {
		return "", fmt.Errorf("error reading database file: %v", err)
	}

	return existingData.Location, nil
}

// parseLocation validates and processes the user-provided location
func parseLocation(input string) (string, error) {
	// Trim whitespace
	input = strings.TrimSpace(input)

	// Check if input is empty
	if input == "" {
		return "", fmt.Errorf("location cannot be empty")
	}

	// Remove surrounding quotes if present
	input = strings.Trim(input, "\"'")

	// Expand home directory if needed
	if strings.HasPrefix(input, "~/") {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			return "", fmt.Errorf("error getting home directory: %v", err)
		}
		input = filepath.Join(homeDir, input[2:])
	}

	// Clean and validate path
	location := filepath.Clean(input)

	// Check if the location is an absolute path
	if !filepath.IsAbs(location) {
		return "", fmt.Errorf("please provide an absolute path")
	}

	return location, nil
}

// createFolder creates the folder if it doesn't exist
func createFolder(location string) (string, error) {
	// Check if the directory exists
	if _, err := os.Stat(location); os.IsNotExist(err) {
		// Prompt user to create the folder
		reader := bufio.NewReader(os.Stdin)
		fmt.Printf("The folder '%s' does not exist. Do you want to create it? (y/n): ", location)
		response, err := reader.ReadString('\n')
		if err != nil {
			return "", fmt.Errorf("error reading input: %v", err)
		}

		// Trim whitespace and convert to lowercase
		response = strings.TrimSpace(strings.ToLower(response))

		if response == "y" || response == "yes" {
			// Create the directory with full permissions
			err = os.MkdirAll(location, 0755)
			if err != nil {
				return "", fmt.Errorf("failed to create directory: %v", err)
			}
			fmt.Printf("Folder '%s' created successfully.\n", location)
		} else {
			return "", fmt.Errorf("folder creation cancelled")
		}
	}

	return location, nil
}

// saveLocation saves the location to the JSON config file
func saveLocation(location string) error {
	// Create config directory if it doesn't exist
	configDir := filepath.Join(os.Getenv("HOME"), ".config", "database")
	err := os.MkdirAll(configDir, 0755)
	if err != nil {
		return fmt.Errorf("error creating config directory: %v", err)
	}

	// Prepare data to be saved
	data := LocationData{
		Location: location,
	}

	// Create JSON file
	filePath := filepath.Join(configDir, "data_base.json")
	file, err := os.Create(filePath)
	if err != nil {
		return fmt.Errorf("error creating file: %v", err)
	}
	defer file.Close()

	// Encode and write JSON
	encoder := json.NewEncoder(file)
	encoder.SetIndent("", "  ")
	err = encoder.Encode(data)
	if err != nil {
		return fmt.Errorf("error writing JSON: %v", err)
	}

	return nil
}
