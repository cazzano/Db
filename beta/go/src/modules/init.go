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

	// Optional: Check if the directory exists
	if _, err := os.Stat(location); os.IsNotExist(err) {
		return "", fmt.Errorf("location does not exist: %s", location)
	}

	return location, nil
}

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

func main() {
	// Prompt user for location
	fmt.Print("Enter a location (absolute path): ")
	reader := bufio.NewReader(os.Stdin)
	input, err := reader.ReadString('\n')
	if err != nil {
		fmt.Println("Error reading input:", err)
		return
	}

	// Parse and validate location
	location, err := parseLocation(input)
	if err != nil {
		fmt.Println("Invalid location:", err)
		return
	}

	// Save location
	err = saveLocation(location)
	if err != nil {
		fmt.Println("Error saving location:", err)
		return
	}

	fmt.Printf("Location '%s' saved successfully to ~/.config/database/data_base.json\n", location)
}
