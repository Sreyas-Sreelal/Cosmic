import logging
import requests
import os
import random

logging.basicConfig(level=logging.DEBUG)

def download_file(url):
    local_filename = url.split('/')[-1]
    r = requests.get(url, stream=True)
    if str(r) != "<Response [200]>":
        return ""

    with open(local_filename, 'wb') as f:
        for chunk in r.iter_content(chunk_size=1024): 
            if chunk: 
                f.write(chunk)
            
    return local_filename

def search():
    try:
        response = requests.get('https://api.imgflip.com/get_memes')
        data = response.json()
        memes = data['data']['memes']
        random_meme = random.choice(memes)['url']
        logging.debug("Meme url is " + random_meme)
        meme_file = download_file(random_meme)
        os.remove(meme_file)

    except Exception as e:
        logging.exception(str(e))
        exit()
    
search()