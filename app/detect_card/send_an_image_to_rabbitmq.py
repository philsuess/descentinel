from overlord_card_match import encode_image
import cv2
import pika
import uuid
import logging
import os

# adapted from https://www.architect.io/blog/2021-01-19/rabbitmq-docker-tutorial/

QUEUE_OL_IMAGES = os.environ["QUEUE_OL_IMAGES"]
EXCHANGE = ""

logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s %(message)s",
)

logging.info("Starting send an image to rabbitmq...")

amqp_url = os.environ["RABBITMQ_AMQP_URL"]

logging.info(f"Looking for rabbitmq AMQP at {amqp_url}")
connection_params = pika.ConnectionParameters(
    host=amqp_url, connection_attempts=3, retry_delay=12.0
)
connection = pika.BlockingConnection(connection_params)
channel = connection.channel()

channel.queue_declare(queue=QUEUE_OL_IMAGES)

image = cv2.imread("tests/BaumSombre.jpg_detected.jpg")
encoded = encode_image(image)
unique_id = str(uuid.uuid4())
send_properties = pika.BasicProperties(
    app_id="descentinel", content_type="image/jpg", correlation_id=unique_id
)
channel.basic_publish(
    exchange=EXCHANGE,
    routing_key=QUEUE_OL_IMAGES,
    body=encoded.tostring(),
    properties=send_properties,
)
connection.close()
