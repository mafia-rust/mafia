from dataclasses import dataclass
from selenium import webdriver
from sys import argv
import getopt
from selenium.webdriver.remote.webdriver import WebDriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

def button(text: str) -> any:
    return (By.XPATH, f"//button[contains(text(), '{text}')]")


def host_game(wait: WebDriverWait) -> None:
    """ Hosts a lobby using the current window """
    wait.until(EC.element_to_be_clickable(button("Host"))).click()

    # Make sure the lobby actually starts
    wait.until(EC.element_to_be_clickable(button("Start Game")))


def join_game(wait: WebDriverWait) -> None:
    """ Connects the current window to the game """
    wait.until(EC.element_to_be_clickable(button("Join"))).click()
    wait.until(EC.element_to_be_clickable(button("Join Lobby"))).click()


@dataclass
class Options:
    driver: WebDriver = None
    players: int = 16
    auto_start: bool = False


def get_options() -> Options:
    """ Parses options from the command line """

    options = Options()

    opts, _ = getopt.getopt(argv[1:], "ad:p:", ["autostart", "driver=", "players="])
    for opt, arg in opts:
        if opt in ("-a", "--autostart"):
            options.auto_start = True
        elif opt in ("-d", "--driver"):
            if arg in ("Firefox", "firefox", "ff", "Mozilla", "mozilla"):
                fo = webdriver.firefox.options.Options()
                fo.set_preference("dom.popup_maximum", 129)
                options.driver = webdriver.Firefox(options=fo)
            elif arg in ("Chrome", "chrome", "Chromium", "chromium", "Google", "google"):
                options.driver = webdriver.Chrome()
            else:
                print(f"Unknown web driver: {arg}")
                exit(1)
        elif opt in ("-p", "--players"):
            if arg.isnumeric():
                options.players = int(arg)
            else:
                print(f"Expected integer number of players. Got: {arg}")
                exit(1)
        else:
            print(f"Unknown flag: {opt}")
            exit(1)

    if options.driver is None:
        options.driver = webdriver.Chrome()

    return options


def main():
    options = get_options()

    driver = options.driver
    wait = WebDriverWait(driver, timeout=10)

    # TODO, make address part of Options
    driver.get("http://localhost:3000/")
    
    # Make the first tab the host
    host_game(wait)
    
    # Makes the rest of the players
    for i in range(options.players-1):
        driver.execute_script("window.open('http://localhost:3000/');")
        driver.switch_to.window(driver.window_handles[i+1])
        join_game(wait)

    # Switch back to host tab
    driver.switch_to.window(driver.window_handles[0])

    if options.auto_start:
        wait.until(EC.element_to_be_clickable(button("Start Game"))).click()


if __name__ == "__main__":
    main()
