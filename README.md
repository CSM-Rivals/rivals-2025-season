# rivals-2025-season
Repository for the rival robotics 2025 season. This repository is also being used for updates and new features for the robot that can be easily generalized to future seasons if needed.

## Setup Local Environment

If you have just pulled this project, follow these steps to setup your local environment. 

To install Python dependencies, enter the follwoing terminal commands:

python -m venv venv

This will create the virtual environment (venv) for Python and any dependencies to run on. This file is too large to justify uploading on to Github, hence the necessity of this process.

.\venv\Scripts\Activate.ps1

This will activate the environment, switching the terminal to the private Python interperter instead of the global one.

pip install -r requirements.txt

Install dependencies specified in the requirements.txt file. This is how the list of dependencies are communicated through Github.

deactivate

Escapes the venv in the terminal, returning to the normal global settings. Running the program still uses the venv contents.

## Update Dependencies

If a new library is installed, it should be done through venv, and the requirements.txt file should be updated with these new instructions using the following commands in the order they are presented:

.\venv\Scripts\Activate.ps1

pip install mylibrary

pip freeze > requirements.txt

deactivate