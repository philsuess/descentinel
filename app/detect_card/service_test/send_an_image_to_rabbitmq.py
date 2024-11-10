import pika
import uuid
import logging
import os
import json

# adapted from https://www.architect.io/blog/2021-01-19/rabbitmq-docker-tutorial/

Q_GAME_ROOM_FEED = os.environ["Q_GAME_ROOM_FEED"]
EXCHANGE = ""

logging.basicConfig(
    filename="log.out",
    filemode="w",
    level=logging.INFO,
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

channel.queue_declare(queue=Q_GAME_ROOM_FEED)

with open("ol_doom.png", "rb") as image_file:
    image_bytes = list(image_file.read())
message = {"content": image_bytes}

message_json = json.dumps(message)

unique_id = str(uuid.uuid4())
send_properties = pika.BasicProperties(
    app_id="descentinel", content_type="application/json", correlation_id=unique_id
)
channel.basic_publish(
    exchange=EXCHANGE,
    routing_key=Q_GAME_ROOM_FEED,
    body=message_json,
    properties=send_properties,
)
connection.close()
