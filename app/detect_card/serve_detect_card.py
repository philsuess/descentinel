from detect_card import OverlordCardsKeywordsMatcher
import pika
import json
import os
import logging

logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s %(message)s",
)

logging.debug("Starting detect_card_service...")

rabbitmq_server = os.environ["RABBITMQSERVER"]

logging.debug(f"Looking for rabbitmq at {rabbitmq_server}")
connection = pika.BlockingConnection(pika.ConnectionParameters(rabbitmq_server))
channel = connection.channel()

channel.exchange_declare(exchange="OVERLORD_CARDS_IMAGES", exchange_type="fanout")

result = channel.queue_declare(queue="", exclusive=True)
queue_name = result.method.queue

channel.queue_bind(exchange="OVERLORD_CARDS_IMAGES", queue=queue_name)

matcher = OverlordCardsKeywordsMatcher.from_file("./keywords_cards.json")
logging.debug("Overlord cards detector initialized.")


def callback(ch, method, properties, body):
    bytestream_from_channel = body
    cv2_image = convert_to_cv2_image(bytestream_from_channel)
    card = matcher.identify(cv2_image)
    logging.debug(f"identified {card}")
    channel.basic_publish(
        exchange="OVERLORD_CARDS_IMAGES", routing_key="", body=json.dumps(card)
    )


channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=True)

channel.start_consuming()
