import sys
import traceback
import time
from bot_log import log

def OnExceptSignal(exc_type, exc_value, tb):
    exce = traceback.format_exception(exc_type,exc_value,tb)
    log.critical(str(exce))
    sys.exit(-1)

sys.excepthook = OnExceptSignal