import sys
import traceback
import time
def OnExceptSignal(exc_type, exc_value, tb):
     
    exce = traceback.format_exception(exc_type,exc_value,tb)
    time_of_log = time.gmtime()
    
    log = open('cosmic_crash_report.txt','a')
    log.write(
        '['+str(time_of_log[0])+
        ':'+str(time_of_log[1])+
        ':'+str(time_of_log[2])+
        ':'+str(time_of_log[3])+
        ':'+str(time_of_log[4])+
        ':'+str(time_of_log[5])+
        ']\n'+str(exce)+'\n')
    log.close()
    
    sys.exit(-1)

sys.excepthook = OnExceptSignal