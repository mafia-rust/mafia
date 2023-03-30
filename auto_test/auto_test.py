from dataclasses import dataclass
from selenium import webdriver
import time
from sys import argv
import getopt
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC


def make_host(driver):
    # wait for the Host button to be clickable
    wait = WebDriverWait(driver, 10)
    host_button = wait.until(EC.element_to_be_clickable(
        (By.XPATH, "//button[contains(text(), 'Host')]")
    ))
    host_button.click()
    wait.until(EC.element_to_be_clickable(
        (By.XPATH, "//button[contains(text(), 'Start Game')]")
    ))


def make_reg_players(driver):
    # wait for the Host button to be clickable
    wait = WebDriverWait(driver, 10)
    host_button = wait.until(EC.element_to_be_clickable(
        (By.XPATH, "//button[contains(text(), 'Join')]")
    ))

    host_button.click()
    #wait till page contains changes 
    wait = WebDriverWait(driver, 10)
    host_button = wait.until(EC.element_to_be_clickable(
        (By.XPATH, "//button[contains(text(), 'Join Lobby')]")
    ))

    host_button.click()


@dataclass
class Options:
    driver = None
    players = 16
    auto_start = False


def get_options():
    """ Parses options from the command line """

    options = Options()

    opts, _ = getopt.getopt(argv[1:], "ad:p:", ["autostart", "driver=", "players="])
    for opt, arg in opts:
        if opt in ("-a", "--autostart"):
            options.auto_start = True
        elif opt in ("-d", "--driver"):
            if arg in ("Firefox", "firefox", "ff", "Mozilla", "mozilla"):
                options.driver = webdriver.Firefox()
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
    # navigate to the Mafia game page
    driver.get("http://localhost:3000/")
    # wait for the page to load
    
    make_host(driver)
    #makes a new tab and runs the make_reg_players function for each player
    for i in range(options.players-1):
        driver.execute_script("window.open('');")
        driver.switch_to.window(driver.window_handles[i+1])
        driver.get("http://localhost:3000/")
        make_reg_players(driver)
    driver.switch_to.window(driver.window_handles[0])
    
    while(True):
        time.sleep(5)


if __name__ == "__main__":
    main()
