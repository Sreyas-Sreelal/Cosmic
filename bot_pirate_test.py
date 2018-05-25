import logging
from bs4 import BeautifulSoup
import requests

logging.basicConfig(level=logging.DEBUG)

def start_test(name):
    try:
        name = name.replace(" ","+")
        res = requests.get("https://thepiratebay-proxylist.org")
        soup = BeautifulSoup(res.content,'html.parser')
        proxy = soup.find('td',{'class':'url'})['data-href']
        payload = proxy+'/s/?q='+name+'&page=0&orderby=99'
        res = requests.get(payload)
        soup = BeautifulSoup(res.content,'html.parser')
        payload = proxy + soup.find('a',{'class':'detLink'})['href']
        res = requests.get(payload)
        soup = BeautifulSoup(res.content,'html.parser')
        magnet = soup.find('a',{'style':"background-image: url('/static/img/icons/icon-magnet.gif');"})['href']
        about  = soup.find('div',{'class':'nfo'}).text
    
        if magnet is not None:
            logging.info("Magnet : " + magnet)    
            logging.info("***Description***\n" + about)
        else:
            logging.error("Failed to find magnet url")

    except Exception as e:
        logging.exception("Failed testing bot_pirate_test.py : " + str(e))

start_test("Witcher 3")


