import discord
from discord.ext import commands
import asyncio
import youtube_dl
import os

class Youtube2Mp3:
    def __init__(self,bot):
        self.bot = bot
    
    async def start_conversion(self,user,link,output):
        await self.bot.wait_until_ready()
        ydl_opts = {
            'outtmpl':output,
            'format': 'bestaudio/best',
            'postprocessors': [{
                'key': 'FFmpegExtractAudio',
                'preferredcodec': 'mp3',
                'preferredquality': '192',
            }],
        }
        
        with youtube_dl.YoutubeDL(ydl_opts) as ydl:
                ydl.download([link])
        
        await self.bot.send_file(user,output)
        os.remove(output)

    @commands.command(pass_context=True)
    async def convert(self,ctx,link:str):
        try:
            conv_idx = link.index('?v=')+3
        
        except ValueError:
            await self.bot.say("Stop trolling kid provide valid youtube link!!")
            pass
        
        converted_name = link[conv_idx:]+'.mp3'
        await self.bot.say("Please wait while till i convert your video")
        
        try:
            self.bot.loop.create_task(self.start_conversion(ctx.message.author,link,converted_name))
            
        except Exception as e:
            await self.bot.say("Make sure your provided a valid link buddy " + str(e))