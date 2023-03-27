from selenium import webdriver
from selenium.webdriver.common.keys import Keys
import time
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

def make_host(driver):
    # wait for the Host button to be clickable
    wait = WebDriverWait(driver, 10)
    host_button = wait.until(EC.element_to_be_clickable((By.XPATH, "//button[contains(text(), 'Host')]")))
    host_button.click()
    wait.until(EC.element_to_be_clickable((By.XPATH, "//button[contains(text(), 'Start Game')]")))
def make_reg_players(driver):
    # wait for the Host button to be clickable
    wait = WebDriverWait(driver, 10)
    host_button = wait.until(EC.element_to_be_clickable((By.XPATH, "//button[contains(text(), 'Join')]")))
    host_button.click()
    #wait till page contains changes 
    wait = WebDriverWait(driver, 10)
    host_button = wait.until(EC.element_to_be_clickable((By.XPATH, "//button[contains(text(), 'Join Lobby')]")))
    host_button.click()
def main(players,auto_start):
    driver = webdriver.Chrome()
    # navigate to the Mafia game page
    driver.get("http://localhost:3000/")
    # wait for the page to load
    
    make_host(driver)
    #makes a new tab and runs the make_reg_players function for each player
    for i in range(players-1):
        driver.execute_script("window.open('');")
        driver.switch_to.window(driver.window_handles[i+1])
        driver.get("http://localhost:3000/")
        make_reg_players(driver)
    driver.switch_to.window(driver.window_handles[0])
    
    while(True):
        time.sleep(5)

    

if __name__ == "__main__":
    main(16,False)
