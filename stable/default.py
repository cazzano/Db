#!/usr/bin/python3

import os
import json
import inquirer
import sys
from datetime import datetime

class DefaultLocationManager:
    """
    Manages default locations for data management
    """
    DEFAULT_CONFIG_PATH = os.path.expanduser('~/.config/data_manager/config.json')
    RECENT_LOCATIONS_PATH = os.path.expanduser('~/.config/data_manager/recent_locations.json')

    @classmethod
    def ensure_config_directory(cls):
        """
        Ensure configuration directory exists
        """
        config_dir = os.path.dirname(cls.DEFAULT_CONFIG_PATH)
        os.makedirs(config_dir, exist_ok=True)

    @classmethod
    def save_recent_location(cls, location):
        """
        Save recently used locations
        """
        try:
            # Ensure config directory exists
            cls.ensure_config_directory()

            # Read existing recent locations
            recent_locations = []
            if os.path.exists(cls.RECENT_LOCATIONS_PATH):
                with open(cls.RECENT_LOCATIONS_PATH, 'r') as f:
                    recent_locations = json.load(f)

            # Remove duplicates and add new location
            if location in recent_locations:
                recent_locations.remove(location)
            recent_locations.insert(0, location)

            # Keep only last 10 unique locations
            recent_locations = recent_locations[:10]

            # Save updated recent locations
            with open(cls.RECENT_LOCATIONS_PATH, 'w') as f:
                json.dump(recent_locations, f, indent=4)

        except Exception as e:
            print(f"Error saving recent location: {e}")

    @classmethod
    def get_recent_locations(cls):
        """
        Retrieve recently used locations
        """
        try:
            if os.path.exists(cls.RECENT_LOCATIONS_PATH):
                with open(cls.RECENT_LOCATIONS_PATH, 'r') as f:
                    return json.load(f)
            return []
        except Exception as e:
            print(f"Error reading recent locations: {e}")
            return []

    @classmethod
    def set_default_location(cls):
        """
        Interactively set default location for data management
        """
        cls.ensure_config_directory()

        # Get recent locations
        recent_locations = cls.get_recent_locations()

        # Prepare location choices
        location_choices = [
            'Current Directory',
            'Home Directory',
            'Custom Directory',
            'Project Directory'
        ]

        # Add recent locations if available
        if recent_locations:
            location_choices.append('Recent Locations')

        # Interactive location selection
        location_questions = [
            inquirer.List(
                'location_type',
                message="Select default location type",
                choices=location_choices
            )
        ]

        # Prompt for location type
        location_result = inquirer.prompt(location_questions)
        location_type = location_result['location_type']

        # Determine the actual path based on selection
        if location_type == 'Current Directory':
            default_path = os.getcwd()
        elif location_type == 'Home Directory':
            default_path = os.path.expanduser('~')
        elif location_type == 'Project Directory':
            default_path = os.path.dirname(os.path.abspath(__file__))
        elif location_type == 'Recent Locations':
            # Select from recent locations
            recent_location_question = [
                inquirer.List(
                    'recent_path',
                    message="Select a recent location",
                    choices=recent_locations
                )
            ]
            recent_result = inquirer.prompt(recent_location_question)
            default_path = recent_result['recent_path']
        else:
            # Custom Directory
            custom_questions = [
                inquirer.Text(
                    'custom_path',
                    message="Enter full path to custom directory",
                    validate=lambda answers, current: (
                        os.path.isdir(current) and len(current) > 0
                    )
                )
            ]
            custom_result = inquirer.prompt(custom_questions)
            default_path = custom_result['custom_path']

        # Validate the selected path
        if not os.path.isdir(default_path):
            print(f"Error: {default_path} is not a valid directory.")
            return None

        # Save configuration
        try:
            config = {
                'default_location': default_path,
                'last_updated': str(datetime.now())
            }

            with open(cls.DEFAULT_CONFIG_PATH, 'w') as config_file:
                json.dump(config, config_file, indent=4)

            # Save as recent location
            cls.save_recent_location(default_path)

            print(f"Default location set to: {default_path}")
            return default_path

        except Exception as e:
            print(f"Error saving configuration: {e}")
            return None

    @classmethod
    def get_default_location(cls):
        """
        Retrieve the default location
        """
        try:
            if not os.path.exists(cls.DEFAULT_CONFIG_PATH):
                print("No default location set. Please set a default location first.")
                return None

            with open(cls.DEFAULT_CONFIG_PATH, 'r') as config_file:
                config = json.load(config_file)
                default_location = config.get('default_location')

                if default_location and os.path.isdir(default_location):
                    return default_location
                else:
                    print("Configured default location is invalid.")
                    return None

        except Exception as e:
            print(f"Error reading configuration: {e}")
            return None

    @classmethod
    def interactive_default_location_management(cls):
        """
        Interactive menu for default location management
        """
        while True:
            questions = [
                inquirer.List(
                    'action',
                    message="Default Location Management",
                    choices=[
                        'Set Default Location',
                        'View Current Default Location',
                        'View Recent Locations',
                        'Exit'
                    ]
                )
            ]

            result = inquirer.prompt(questions)
            action = result['action']

            if action == 'Set Default Location':
                cls.set_default_location()
            elif action == 'View Current Default Location':
                current_location = cls.get_default_location()
                if current_location:
                    print(f"Current Default Location: {current_location}")
            elif action == 'View Recent Locations':
                recent_locations = cls.get_recent_locations()
                if recent_locations:
                    print("Recent Locations:")
                    for idx, location in enumerate(recent_locations, 1):
                        print(f"{idx}. {location}")
                else:
                    print("No recent locations found.")
            else:
                break

def main():
    """
    Entry point for default location management
    """
    DefaultLocationManager.interactive_default_location_management()

if __name__ == "__main__":
    main()
