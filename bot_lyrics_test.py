from bs4 import BeautifulSoup
import requests
import logging

logging.basicConfig(level=logging.DEBUG)

def start_test(name,link,headers):
    name = name.replace(" ","+")
    get_request_search = requests.get(link + name+ "&w=songs&p=1" , headers=headers)
    soup_search = BeautifulSoup(get_request_search.content,'html.parser')
    
    try:
        link_from_search  = soup_search.find('td',{'class':'text-left visitedlyr'}).find('a')
        get_request_lyrics = requests.get(link_from_search['href'],headers=headers)
        soup_lyrics = BeautifulSoup(get_request_lyrics.content,'html.parser')
        
        lyrics_text = soup_lyrics.find('div',{'class':'col-xs-12 col-lg-8 text-center'}).find('div',None).text
        header_text = soup_lyrics.find('div',{'class':'lyricsh'}).text       
        logging.info('header : ' + header_text)
        logging.info('lryics : ' + lyrics_text)
    
    except Exception as e:
        logging.exception(str(e))

headers = { 'User-Agent': 'Mozilla/5.0 (Windows NT 6.0; WOW64; rv:24.0) Gecko/20100101 Firefox/24.0' }
link = "https://search.azlyrics.com/search.php?q="
start_test("stand by you",link,headers)

