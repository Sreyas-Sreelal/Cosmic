import discord
from discord.ext import commands
import requests
import asyncio
import random
from bot_log import log
import os 

class Memes:
    
    def __init__(self,bot):
        self.bot = bot
    
    def download_file(self,url):
        local_filename = url.split('/')[-1]
        r = requests.get(url, stream=True)
        if str(r) != "<Response [200]>":
            return ""

        with open(local_filename, 'wb') as f:
            for chunk in r.iter_content(chunk_size=1024): 
                if chunk: 
                    f.write(chunk)
                
        return local_filename

    async def search(self,channel):
        await self.bot.wait_until_ready()
        try:
            response = requests.get('https://api.imgflip.com/get_memes')
            data = response.json()
            memes = data['data']['memes']
            random_meme = random.choice(memes)['url']
            log.info("Meme url is " + random_meme)
            meme_file = self.download_file(random_meme)
            await self.bot.send_file(channel,meme_file)
            os.remove(meme_file)

        except Exception as e:
            await self.bot.send_message(channel,"OOps can't get the meme")
            log.exception(str(e))
    
    @commands.command(pass_context=True)
    async def meme(self,ctx):
        await self.bot.loop.create_task(self.search(ctx.message.channel))

        
       