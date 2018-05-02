import discord
from discord.ext import commands

class CMDS:
    def __init__(self,bot):
        self.bot =  bot    
    
    @commands.command()
    async def help(self):

        embed = discord.Embed(title = "Cosmic commands",description="\
$help - displays all commands\n\
$searchtor - search for torrent\n\
$play - plays a music\n\
$skip - skips the currently playing song\n\
$resume - resumes the paused song\n\
$volume - set volume\n\
$stop   - stops currently playing song\n\
$playing - tells what is playing now\n\
$summon - summons cosmic to a voice channel\n\
$join - call cosmic to join \n\
$convert - converts youtube videos to mp3\n\
$lyrics - finds lyric of songs",color=0xff0004)
        await self.bot.say(embed=embed)