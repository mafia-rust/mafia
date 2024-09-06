
import json
import re
import sys
import os
sys.path.append(os.path.abspath('..'))

from lang import CLIENT_LANG_ROOT, ENGLISH_LANG_FILE_PATH

BROKEN_KEYBOARD_LANG_FILE_PATH = f"{CLIENT_LANG_ROOT}/broken_keyboard.json"

def break_word(word: str) -> str:
    letters = list(word)
    filtered = "".join(filter(lambda l: l not in "aeiouAEIOU", letters))
    
    return filtered


def break_sentence(key: str, value: str) -> str:
    if key == "language":
        return "Broken Keyboard"
    
    return "".join([
        break_word(word) 
        for word in re.findall(r"[\w'-]+|\W", value)
        ])


def main():
    with open(ENGLISH_LANG_FILE_PATH, "r") as english_lang_file:
        lang = json.loads(english_lang_file.read())

        lang_dict: dict[str, str] = dict(lang)

        broken_keyboard_dict: dict[str, str] = {}
        for (key, value) in lang_dict.items():
            broken_keyboard_dict[key] = break_sentence(key, value)
        
        with open(BROKEN_KEYBOARD_LANG_FILE_PATH, "w+") as broken_keyboard_lang_file:
            broken_keyboard_lang_file.write(json.dumps(broken_keyboard_dict, indent=4))



if __name__ == "__main__":
    main()