from dataclasses import dataclass
from typing import NoReturn
from selenium import webdriver
from sys import argv
import getopt
from selenium.webdriver.remote.webdriver import WebDriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

def wait_for_button(driver: WebDriver, text: str) -> any:
    return WebDriverWait(driver, timeout=10).until(EC.element_to_be_clickable(
        (By.XPATH, f"//button[contains(text(), '{text}')]")
    ))


def exit_error(error: str = None) -> NoReturn:
    if error is not None: print(error)

    print(f"""Usage: python3 create_lobby.py [OPTIONS]

Options:
    -h --help            : Show this help menu
    -s --autostart       : Automatically start the game after all the clients have 
                           connected
    -a --address addr    : Connect to the specified address, rather than the 
                           default (https://localhost:3000/)
    -d --driver driver   : Specify the driver to use. Chrome and Firefox are 
                           supported
    -p --players players : Specify the number of players to connect to the game. 
                           The default is 16
""")
    exit(1)


def host_game(driver: WebDriver) -> None:
    """ Hosts a lobby using the current window """
    wait_for_button(driver, "Host").click()

    # Make sure the lobby actually starts
    wait_for_button(driver, "Start Game")


def join_game(driver: WebDriver) -> None:
    """ Connects the current window to the game """
    wait_for_button(driver, "Join").click()
    wait_for_button(driver, "Join Lobby").click()


@dataclass
class Options:
    driver: WebDriver = None
    players: int = 16
    auto_start: bool = False
    server_address: str = "http://localhost:3000/"


def get_options() -> Options:
    """ Parses options from the command line """

    options = Options()

    try:
        opts, _ = getopt.getopt(argv[1:], "hsa:d:p:", ["help", "autostart", "address=", "driver=", "players="])
    except getopt.GetoptError as error:
        exit_error("Error: " + error.msg)
    for opt, arg in opts:
        if opt in ("-s", "--autostart"):
            options.auto_start = True
        elif opt in ("-h", "--help"):
            exit_error()
        elif opt in ("-a", "--address"):
            options.server_address = arg
        elif opt in ("-d", "--driver"):
            if arg in ("Firefox", "firefox", "ff", "Mozilla", "mozilla"):
                fo = webdriver.firefox.options.Options()
                fo.set_preference("dom.popup_maximum", 129)
                options.driver = webdriver.Firefox(options=fo)
            elif arg in ("Chrome", "chrome", "Chromium", "chromium", "Google", "google"):
                options.driver = webdriver.Chrome()
            else:
                exit_error(f"Unknown web driver: {arg}")
        elif opt in ("-p", "--players"):
            if arg.isnumeric():
                options.players = int(arg)
            else:
                exit_error(f"Expected integer number of players. Got: {arg}")
        else:
            exit_error(f"Unknown flag: {opt}")

    if options.driver is None:
        options.driver = webdriver.Chrome()

    return options


def main():
    options = get_options()

    driver = options.driver

    driver.get(options.server_address)
    
    # Make the first tab the host
    host_game(driver)
    
    # Makes the rest of the players
    for i in range(options.players-1):
        driver.execute_script(f"window.open('{options.server_address}');")
        driver.switch_to.window(driver.window_handles[i+1])
        join_game(driver)

    # Switch back to host tab
    driver.switch_to.window(driver.window_handles[0])

    if options.auto_start:
        wait_for_button("Start Game").click()


if __name__ == "__main__":
    main()
