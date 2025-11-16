import json
import os

class SettingsReader:

    def load_settings(self, filename="lvm_settings.json"):
        
        #build full file path
        config_path = os.path.join(os.getcwd(), filename)

        try:
            #read file
            with open(config_path, 'r') as config_file:
                #json.load() converts json variables to python dictionary
                settings = json.load(config_file)
            return settings
        except FileNotFoundError:
            print(f"Error: The file '{filename}' was not found at {config_path}")
            return None
        except json.JSONDecodeError as e:
            print(f"Error decoding JSON from file '{filename}': {e}")
            return None
