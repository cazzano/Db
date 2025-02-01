package main

import (
	"os/user"
	"path/filepath"
	"strings"
)

// sanitizePath cleans and resolves the path
func sanitizePath(path string) string {
	// Remove quotes if present
	path = strings.Trim(path, "\"'")

	// Expand user path (e.g., ~/Documents)
	expandedPath := expandUserPath(path)

	// Clean the path (remove redundant separators, resolve ..)
	cleanPath := filepath.Clean(expandedPath)

	return cleanPath
}

// expandUserPath expands the ~ to the user's home directory
func expandUserPath(path string) string {
	if !strings.HasPrefix(path, "~") {
		return path
	}

	currentUser, err := user.Current()
	if err != nil {
		return path
	}

	homeDir := currentUser.HomeDir
	return filepath.Join(homeDir, path[2:])
}
