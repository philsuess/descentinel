from overlord_card_match import OverlordCardsKeywordsMatcher, decode_image
from recognize_card import CardDetector, CardType
import pika
import cv2
import json
import os
import logging
import numpy as np

# adapted from https://www.architect.io/blog/2021-01-19/rabbitmq-docker-tutorial/

QUEUE_GAME_ROOM_FEED = os.environ["QUEUE_GAME_ROOM_FEED"]
QUEUE_DETECTED_OL_CARDS = os.environ["QUEUE_DETECTED_OL_CARDS"]
EXCHANGE = ""


def on_message(channel, delivery, properties, body):
    """Callback when a message arrives.
    :param channel: the AMQP channel object.
    :type channel: :class:`pika.channel.Channel`
    :param delivery: the AMQP protocol-level delivery object,
      which includes a tag, the exchange name, and the routing key.
      All of this should be information the sender has as well.
    :type delivery: :class:`pika.spec.Deliver`
    :param properties: AMQP per-message metadata.  This includes
      things like the body's content type, the correlation ID and
      reply-to queue for RPC-style messaging, a message ID, and so
      on.  It also includes an additional table of structured
      caller-provided headers.  Again, all of this is information
      the sender provided as part of the message.
    :type properties: :class:`pika.spec.BasicProperties`
    :param str body: Byte string of the message body.
    """
    # Just dump out the information we think is interesting.
    logging.info(f"Got an image from game room feed on Exchange: {delivery.exchange}")
    logging.info(f"\tRouting key: {delivery.routing_key}")
    logging.info(f"\tContent type: {properties.content_type}")
    # logging.info(body)

    bytestream_from_channel = np.frombuffer(body, dtype=np.uint8)
    cv2_image = decode_image(bytestream_from_channel)

    card_type = detector.detect(cv2_image)
    logging.info(f"Detected card type: {card_type}")

    if card_type != CardType.OVERLORD:
        return

    card = matcher.identify(cv2_image)
    logging.info(f"\tidentified {card}")
    if not card:
        return

    send_properties = pika.BasicProperties(
        app_id="descentinel",
        content_type="application/json",
        correlation_id=properties.correlation_id,
    )
    channel.basic_publish(
        exchange=EXCHANGE,
        routing_key=QUEUE_DETECTED_OL_CARDS,
        body=json.dumps(card),
        properties=send_properties,
    )
    logging.info(f"\tIdentified card pushed to queue {QUEUE_DETECTED_OL_CARDS}")


logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(message)s",
)

template_overlord_image = cv2.imread("OL_template.jpg", cv2.IMREAD_GRAYSCALE)
detector = CardDetector(template_overlord_card_cv2_image=template_overlord_image)
logging.info("Overlord cards detector initialized.")

matcher = OverlordCardsKeywordsMatcher.from_file("./keywords_cards.json")
logging.info("Overlord cards keyword matcher initialized.")
logging.info("Starting detect_card_service...")

amqp_url = os.environ["RABBITMQ_AMQP_URL"]

logging.info(f"Looking for rabbitmq AMQP at {amqp_url}")
connection_params = pika.ConnectionParameters(
    host=amqp_url, connection_attempts=3, retry_delay=12.0
)
connection = pika.BlockingConnection(connection_params)
channel = connection.channel()

channel.queue_declare(queue=QUEUE_GAME_ROOM_FEED)
channel.queue_declare(queue=QUEUE_DETECTED_OL_CARDS)

channel.basic_consume(
    queue=QUEUE_GAME_ROOM_FEED, on_message_callback=on_message, auto_ack=True
)
channel.start_consuming()
