import json
import csv
from typing import List
import sys
import os
sys.path.append(os.path.abspath('../..'))

from generate import PROJECT_ROOT


CLIENT_DUMMY_NAMES_FILE_PATH = f"{PROJECT_ROOT}/client/src/resources/dummyNames.json"
SERVER_DEFAULT_DUMMY_NAMES_FILE_PATH = f"{PROJECT_ROOT}/server/resources/random_names/default_names.csv"
SERVER_EXTRA_DUMMY_NAMES_FILE_PATH = f"{PROJECT_ROOT}/server/resources/random_names/extra_names.csv"

def main():
    with open("./dummyNames.json") as dummy_names:
        names = json.loads(dummy_names.read())
        default_names: List[str] = names["defaultNames"]
        extra_names: List[str] = names["extraNames"]

        with open(CLIENT_DUMMY_NAMES_FILE_PATH, "w+") as client_dummy_names_file:
            combined_dummy_names = default_names + extra_names
            client_dummy_names_file.write(json.dumps(combined_dummy_names, indent=4))
        
        with open(SERVER_DEFAULT_DUMMY_NAMES_FILE_PATH, "w+") as server_default_dummy_names_file:
            writer = csv.writer(server_default_dummy_names_file)
            writer.writerows(map(lambda s: [s], default_names))

        with open(SERVER_EXTRA_DUMMY_NAMES_FILE_PATH, "w+") as server_extra_dummy_names_file:
            writer = csv.writer(server_extra_dummy_names_file)
            writer.writerows(map(lambda s: [s], extra_names))


if __name__ == "__main__":
    main()
