# descentinel
Support app for the board game Descent v1 that keeps watch over a physical game in progress.

1. The app consists of various services:
    - [monitor](#monitor): always watching...
    - [detect_card](#detect_card): which card am I holding?
1. All services are connected to rabbitmq pub/sub channels
1. The entire app runs on a small device (Raspberry Pi)

## monitor: stream the game pieces as images {#monitor}

What?
- A continuous video feed from a small camera is chunked into still images for downstream services to digest.

Why?
- This is the eye of the app. It's the one source of information (for now).

How?
- Capture video signal and chop it into still images in pre-defined time intervals.
- rust?

## detect_card: recognize various cards {#detect_card}

What? 
- An image of a card is given to this service. Based on character recognition, the contents of this card are matched against a database of cards and the correct card type and card are identified.

Why?
- Other services may use the context information to display more infos about the card. Examples may be translations from other languages or information from http://www.descentinthedark.com.
- Ultimately we have a (ro)bot playing with us and using and referencing the physical components.

How?
- python
- cv2 feature mathching to detect card type (OL/equipment/...)
- tesseract for character recognition



Descent: Journeys in the Dark is Copyright © Fantasy Flight Games. All rights reserved.
