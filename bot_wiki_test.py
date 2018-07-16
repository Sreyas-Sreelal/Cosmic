import logging
import wikipedia

logging.basicConfig(level=logging.DEBUG)

def start_test(name):
    try:
        logging.info(wikipedia.summary(name))       
    except wikipedia.DisambiguationError:
        logging.info("DisambiguationError Specify that more precisely cause")
    except Exception as e:
        logging.error("[Failed Operation] : " + str(e))

start_test("Python")
start_test("Go lang")
start_test("Einstein")