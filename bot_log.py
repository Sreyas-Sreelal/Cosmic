import logging

log = logging.Logger('cosmic')
hndl = logging.FileHandler('cosmic-log.log')
formatter = logging.Formatter('[%(asctime)s] [%(levelname)s] %(message)s')
hndl.setFormatter(formatter)
log.addHandler(hndl)
log.setLevel(logging.DEBUG)

