package main

import (
	"fmt"
	"os"
)

func main() {
	// Check if a command is provided
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./main <command>")
		fmt.Println("Available commands:")
		fmt.Println("  init   - Initialize or update the base location")
		fmt.Println("  store  - Create and store a new folder")
		fmt.Println("  recent - List recently created folders")
		fmt.Println("  drop   - Drop files into a folder")
		return
	}

	// Handle commands
	switch os.Args[1] {
	case "init":
		// Call the init functionality
		Init()
	case "store":
		// Call the store functionality
		Store()
	case "recent":
		// Call the recent functionality
		Recent()
	case "drop":
		// Call the drop functionality
		Drop()
	default:
		fmt.Printf("Unknown command: %s\n", os.Args[1])
		fmt.Println("Available commands:")
		fmt.Println("  init   - Initialize or update the base location")
		fmt.Println("  store  - Create and store a new folder")
		fmt.Println("  recent - List recently created folders")
		fmt.Println("  drop   - Drop files into a folder")
	}
}
