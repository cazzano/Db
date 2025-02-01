package main

import "time"

// Config represents the structure of the JSON configuration file
type Config struct {
	Location string            `json:"location"`
	Folders  map[string]Folder `json:"folders"`
}

// Folder represents metadata about a folder
type Folder struct {
	Path        string    `json:"path"`
	CreatedAt   time.Time `json:"created_at"`
	Description string    `json:"description,omitempty"`
}
