from selenium import webdriver
from selenium.webdriver.common.keys import Keys
import time

def host_join():
    driver = webdriver.Chrome()
    driver.get("http://localhost:3000/")

    # Click "Host" button
    time.sleep(2)
    host_button = driver.find_element_by_xpath("//button[contains(text(), 'Host')]")
    host_button.click()

    # Click "Join" button
    time.sleep(2)
    join_button = driver.find_element_by_xpath("//button[contains(text(), 'Join')]")
    join_button.click()

    # Close the driver
    driver.close()

if __name__ == "__main__":
    host_join()
