from detect_card import encode_image
import cv2
import pika
import logging
import os

#adapted from https://www.architect.io/blog/2021-01-19/rabbitmq-docker-tutorial/

QUEUE=os.environ['QUEUE']
EXCHANGE=os.environ['EXCHANGE']

logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s %(message)s",
)

logging.info("Starting send an image to rabbitmq...")

amqp_url = os.environ['RABBITMQ_AMQP_URL']

logging.info(f"Looking for rabbitmq AMQP at {amqp_url}")
connection_params = pika.ConnectionParameters(host=amqp_url, connection_attempts=3, retry_delay=12.0)
connection = pika.BlockingConnection(connection_params)
channel = connection.channel()

channel.queue_declare(queue=QUEUE)

image = cv2.imread("tests/BaumSombre.jpg_detected.jpg")
encoded = encode_image(image)
send_properties = pika.BasicProperties(app_id='descentinel', content_type='image/jpg')
channel.basic_publish(exchange=EXCHANGE, routing_key='ol_card', body=str(encoded), properties=send_properties)
connection.close()
