import logging

log = logging.Logger('cosmic')
formatter = logging.Formatter('[%(asctime)s] [%(levelname)s] %(message)s')

handle_file = logging.FileHandler('cosmic_log.txt')
handle_file.setFormatter(formatter)
log.addHandler(handle_file)

handle_terminal = logging.StreamHandler()
handle_terminal.setFormatter(formatter)
log.addHandler(handle_terminal)

log.setLevel(logging.DEBUG)

