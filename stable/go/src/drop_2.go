package main

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
)

func copyFile(src, dst string) error {
	sourceFile, err := os.Open(src)
	if err != nil {
		return fmt.Errorf("error opening source file: %v", err)
	}
	defer sourceFile.Close()

	// Check if file already exists in destination
	if _, err := os.Stat(dst); err == nil {
		fmt.Printf("File '%s' already exists. Overwrite? (y/n): ", filepath.Base(dst))
		var response string
		fmt.Scanln(&response)
		if strings.ToLower(response) != "y" {
			return fmt.Errorf("file copy cancelled")
		}
	}

	destFile, err := os.Create(dst)
	if err != nil {
		return fmt.Errorf("error creating destination file: %v", err)
	}
	defer destFile.Close()

	_, err = io.Copy(destFile, sourceFile)
	if err != nil {
		return fmt.Errorf("error copying file: %v", err)
	}

	// Preserve file permissions
	sourceInfo, err := os.Stat(src)
	if err == nil {
		err = os.Chmod(dst, sourceInfo.Mode())
		if err != nil {
			fmt.Printf("Warning: Could not preserve file permissions: %v\n", err)
		}
	}

	return nil
}

func copyDir(src, dst string) error {
	// Check if directory already exists
	if _, err := os.Stat(dst); err == nil {
		fmt.Printf("Directory '%s' already exists. Overwrite? (y/n): ", filepath.Base(dst))
		var response string
		fmt.Scanln(&response)
		if strings.ToLower(response) != "y" {
			return fmt.Errorf("directory copy cancelled")
		}
		// Remove existing directory
		os.RemoveAll(dst)
	}

	// Create destination directory
	err := os.MkdirAll(dst, 0755)
	if err != nil {
		return fmt.Errorf("error creating destination directory: %v", err)
	}

	// Walk through the source directory
	return filepath.Walk(src, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		// Get the relative path
		relativePath, err := filepath.Rel(src, path)
		if err != nil {
			return err
		}

		// Construct destination path
		destPath := filepath.Join(dst, relativePath)

		// If it's a directory, create it
		if info.IsDir() {
			return os.MkdirAll(destPath, 0755)
		}

		// If it's a file, copy it
		return copyFile(path, destPath)
	})
}

func moveFile(src, dst string) error {
	// First copy the file
	err := copyFile(src, dst)
	if err != nil {
		return err
	}

	// Then remove the source file
	err = os.Remove(src)
	if err != nil {
		return fmt.Errorf("error removing source file: %v", err)
	}

	return nil
}

func moveDir(src, dst string) error {
	// First copy the directory
	err := copyDir(src, dst)
	if err != nil {
		return err
	}

	// Then remove the source directory
	err = os.RemoveAll(src)
	if err != nil {
		return fmt.Errorf("error removing source directory: %v", err)
	}

	return nil
}
