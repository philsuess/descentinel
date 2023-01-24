from detect_card import OverlordCardsKeywordsMatcher
import pika
import json

connection = pika.BlockingConnection(pika.ConnectionParameters(host="localhost"))
channel = connection.channel()

channel.exchange_declare(exchange="OVERLORD_CARDS_IMAGES", exchange_type="fanout")

result = channel.queue_declare(queue="", exclusive=True)
queue_name = result.method.queue

channel.queue_bind(exchange="OVERLORD_CARDS_IMAGES", queue=queue_name)

matcher = OverlordCardsKeywordsMatcher.from_file("./keywords_cards.json")

print(" [*] Waiting for logs. To exit press CTRL+C")


def callback(ch, method, properties, body):
    bytestream_from_channel = body
    cv2_image = convert_to_cv2_image(bytestream_from_channel)
    card = matcher.identify(cv2_image)
    print(f"identified {card}")
    channel.basic_publish(
        exchange="OVERLORD_CARDS_IMAGES", routing_key="", body=json.dumps(card)
    )


channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=True)

channel.start_consuming()
