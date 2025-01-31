package main

import (
	"fmt"
	"os"
)

func main() {
	// Check if no arguments are provided
	if len(os.Args) < 2 {
		fmt.Println("Error: No command specified")
		displayHelp()
		os.Exit(1)
	}

	// Get the command from arguments
	command := os.Args[1]

	// Handle different commands
	switch command {
	case "store":
		err := createFolder() // Direct call to createFolder from store.go
		if err != nil {
			fmt.Println("Error in store command:", err)
			os.Exit(1)
		}

	case "recent":
		err := listRecentFolders() // Direct call to listRecentFolders from recent.go
		if err != nil {
			fmt.Println("Error in recent command:", err)
			os.Exit(1)
		}

	case "help":
		displayHelp()

	default:
		fmt.Printf("Unknown command: %s\n", command)
		displayHelp()
		os.Exit(1)
	}
}

func displayHelp() {
	fmt.Println("DB CLI Tool - Manage Your Folders and Configurations")
	fmt.Println("\nUsage: go run main.go <command> [options]")
	fmt.Println("\nAvailable Commands:")
	fmt.Println("  store    - Create and manage folders")
	fmt.Println("  recent   - List recently created folders")
	fmt.Println("  help     - Show this help message")
	fmt.Println("\nExamples:")
	fmt.Println("  go run main.go store")
	fmt.Println("  go run main.go recent")
}
