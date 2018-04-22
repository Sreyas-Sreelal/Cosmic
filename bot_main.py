import bot_crash_detect
from bot_log import log
import asyncio
from discord.ext import commands
from bot_music import Music
from bot_mp3 import Youtube2Mp3
from bot_lyrics import Lyrics
from bot_chat import BotChatAi
from bot_pirate import Torrent

cosmic = commands.Bot(command_prefix='$')


@cosmic.event
async def on_ready():
    log.info('Started cosmic...')

cosmic.add_cog(Music(cosmic))
cosmic.add_cog(Youtube2Mp3(cosmic))
cosmic.add_cog(Lyrics(cosmic))
cosmic.add_cog(BotChatAi(cosmic))
cosmic.add_cog(Torrent(cosmic))

f = open('token.txt','r')
token=f.read().strip()
f.close()

try:
    cosmic.run(token)
except:
    log.error("make sure you provided valid token")
    exit()
    