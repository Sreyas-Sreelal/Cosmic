import bot_crash_detect
from bot_log import log
import asyncio
import discord
from discord.ext import commands
from bot_music import Music
from bot_mp3 import Youtube2Mp3
from bot_lyrics import Lyrics
from bot_chat import BotChatAi
from bot_pirate import Torrent
from bot_cmds import CMDS
from bot_memes import Memes
import os

cosmic = commands.Bot(command_prefix='$')
cosmic.remove_command("help")

@cosmic.event
async def on_ready():
    log.info('Cosmic started successfully')

cosmic.add_cog(Music(cosmic))
cosmic.add_cog(Youtube2Mp3(cosmic))
cosmic.add_cog(Lyrics(cosmic))
cosmic.add_cog(BotChatAi(cosmic))
cosmic.add_cog(Torrent(cosmic))
cosmic.add_cog(CMDS(cosmic))
cosmic.add_cog(Memes(cosmic))

token = os.getenv('COSMIC_TOKEN')

try:
    cosmic.run(token)
    
except discord.errors.LoginFailure:
    log.error("Invalid token provided")
    exit()
    