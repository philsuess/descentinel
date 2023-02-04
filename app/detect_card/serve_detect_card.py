from detect_card import OverlordCardsKeywordsMatcher
from functools import partial
import pika
import json
import os
import logging

#adapted from https://www.architect.io/blog/2021-01-19/rabbitmq-docker-tutorial/


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
    logging.info(f'Exchange: {delivery.exchange}')
    logging.info(f'Routing key: {delivery.routing_key}')
    logging.info(f'Content type: {properties.content_type}')
    logging.info(body)

    bytestream_from_channel = body
    cv2_image = convert_to_cv2_image(bytestream_from_channel)
    card = matcher.identify(cv2_image)
    logging.info(f"identified {card}")
    channel.basic_publish(
        exchange=EXCHANGE, routing_key="", body=json.dumps(card)
    )

    # Important!!! You MUST acknowledge the delivery.  If you don't,
    # then the broker will believe it is still outstanding, and
    # because we set the QoS limit above to 1 outstanding message,
    # we'll never get more.
    #
    # If something went wrong but retrying is a valid option, you
    # could also basic_reject() the message.
    channel.basic_ack(delivery.delivery_tag)


logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s %(message)s",
)

matcher = OverlordCardsKeywordsMatcher.from_file("./keywords_cards.json")
logging.info("Overlord cards detector initialized.")

QUEUE=""
EXCHANGE="OVERLORD_CARDS_IMAGES"

logging.info("Starting detect_card_service...")

amqp_url = os.environ['RABBITMQ_AMQP_URL']

logging.info(f"Looking for rabbitmq AMQP at {amqp_url}")
connection_params = pika.ConnectionParameters(host=amqp_url, connection_attempts=3, retry_delay=12.0)
connection = pika.BlockingConnection(connection_params)
channel = connection.channel()

channel.queue_declare(queue=QUEUE)
channel.basic_consume(queue=QUEUE, on_message_callback=on_message, auto_ack=True)
channel.start_consuming()
 
