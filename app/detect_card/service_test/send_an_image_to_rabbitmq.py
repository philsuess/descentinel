import cv2
import pika
import uuid
import logging
import os

# adapted from https://www.architect.io/blog/2021-01-19/rabbitmq-docker-tutorial/


def encode_image(cv2_image):
    encode_param = [int(cv2.IMWRITE_JPEG_QUALITY), 100]
    _, buffered = cv2.imencode(".jpg", cv2_image, encode_param)
    return buffered


Q_CARD_IMAGE = os.environ["Q_CARD_IMAGE"]
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

channel.queue_declare(queue=Q_CARD_IMAGE)

image = cv2.imread("BaumeSombre_02.jpg_detected.jpg")
encoded = encode_image(image)
unique_id = str(uuid.uuid4())
send_properties = pika.BasicProperties(
    app_id="descentinel", content_type="image/jpg", correlation_id=unique_id
)
channel.basic_publish(
    exchange=EXCHANGE,
    routing_key=Q_CARD_IMAGE,
    body=encoded.tostring(),
    properties=send_properties,
)
connection.close()
