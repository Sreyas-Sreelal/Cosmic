import logging

log = logging.Logger('cosmic')
handle_file = logging.FileHandler('cosmic_log.txt')
formatter = logging.Formatter('[%(asctime)s] [%(levelname)s] %(message)s')
handle_file.setFormatter(formatter)
log.addHandler(handle_file)
log.setLevel(logging.DEBUG)

