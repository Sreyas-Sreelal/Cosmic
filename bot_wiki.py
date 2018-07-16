from bot_log import log
import discord
from discord.ext import commands
import asyncio
from bs4 import BeautifulSoup
import requests
import wikipedia

class Wiki:
    def __init__(self,bot):
        self.bot = bot
    
    async def search(self,content,channel):
        await self.bot.wait_until_ready()
        try:
            summary = wikipedia.summary(content)
            embed = discord.Embed(title=content,description=summary,color=0xff4c00)
            await self.bot.send_message(channel,embed=embed)
        except wikipedia.DisambiguationError:
            await self.bot.send_message(channel,"Found more than one match specify that more precisely")
        except discord.errors.DiscordException:
            await self.bot.send_message(channel,"Too long information about " + content + " to post here.Refer here " + wikipedia.page(content).url)
        except Exception as e:
            log.error("[Failed Operation] : " + str(e))
            await self.bot.send_message(channel,"Sorry i can't process that right now. If this persists open an issue at https://github.com/sreyas-sreelal/cosmic")

    @commands.command(pass_context=True)
    async def wiki(self,ctx,*,content:str):
        await self.bot.loop.create_task(self.search(content,ctx.message.channel))
        
       
