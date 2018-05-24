from bot_log import log
import discord
from discord.ext import commands
import asyncio
from bs4 import BeautifulSoup
import requests

class Torrent:
    def __init__(self,bot):
        self.bot = bot

    async def search(self,name,channel,user):
        await self.bot.wait_until_ready()
        try:
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
                await self.bot.send_message(channel,"Got your torrent's magnet link")
                await self.bot.send_message(channel,"```"+magnet+"```")
                if len(about)>10:
                    embed = discord.Embed(title=name.replace("+"," "),description=about[:6000],color=0xff4c00)
                    await self.bot.send_message(channel,embed=embed)
                   
            else:
                await self.bot.send_message(channel,"Ehh can't find anything with that name")
        
        except Exception as e:
            await self.bot.send_message(channel,"Eww i think something got me there or what i'm searchin is not right")
            log.exception(str(e))

    @commands.command(pass_context=True)
    async def searchtor(self,ctx,*,torrent:str):
        await self.bot.say("Lemme look for " + torrent + " torrent ....")
        torrent = torrent.replace(" ","+")
        await self.bot.loop.create_task(self.search(torrent,ctx.message.channel,ctx.message.author))
        
       
