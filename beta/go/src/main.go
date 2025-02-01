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
		fmt.Println("  init - Initialize or update the base location")
		return
	}

	// Handle commands
	switch os.Args[1] {
	case "init":
		// Call the init functionality
		initCommand()
	default:
		fmt.Printf("Unknown command: %s\n", os.Args[1])
		fmt.Println("Available commands:")
		fmt.Println("  init - Initialize or update the base location")
	}
}

// initCommand calls the functionality from init.go
func initCommand() {
	// Call the main function from init.go
	// Note: This assumes that the main function in init.go is refactored into a reusable function.
	// For example, you can rename the main function in init.go to Init() and call it here.
	Init()
}
