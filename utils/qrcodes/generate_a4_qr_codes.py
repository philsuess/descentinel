import json
from pathlib import Path
from typing import Dict, Any, Union
from collections.abc import Mapping

import qrcode
from reportlab.lib.pagesizes import A4
from reportlab.pdfgen import canvas
from reportlab.lib.units import cm


def read_overlord_cards_from_file(path: Union[str, Path]) -> Dict[str, Any]:
    """
    Reads overlord cards from a JSON file.

    Args:
        path (str or Path): Path to the JSON file.

    Returns:
        Dict[str, Any]: A dictionary mapping strings to OverlordCard data.

    Raises:
        FileNotFoundError: If the file cannot be found.
        json.JSONDecodeError: If the file is not valid JSON.
    """
    path = Path(path)  # Ensure the path is a Path object
    with path.open("r", encoding="utf-8") as file:
        cards = json.load(file)  # Deserialize JSON into a dictionary
    if not isinstance(cards, Mapping):
        raise ValueError("Expected a JSON object at the top level of the file.")
    return cards


def generate_qr_pdf(keywords, output_file="qrcodes.pdf"):
    # Page size and QR code settings
    page_width, page_height = A4
    qr_size = 2 * cm
    font_size = 8
    margin = 2 * cm  # Margin from page edges
    spacing = 0.5 * cm  # Space between QR codes

    # Calculate rows and columns
    cols = int((page_width - 2 * margin) // (qr_size + spacing))
    rows = int((page_height - 2 * margin) // (qr_size + spacing))

    # Create canvas for the PDF
    pdf = canvas.Canvas(output_file, pagesize=A4)
    pdf.setFont("Helvetica", font_size)

    x_start = margin
    y_start = page_height - margin - qr_size

    # Loop through keywords and generate QR codes
    for i, keyword in enumerate(keywords):
        # Calculate grid position relative to the current page
        col = i % cols
        row = (i // cols) % rows  # Ensure `row` resets correctly on a new page

        if i > 0 and i % (cols * rows) == 0:  # New page condition
            pdf.showPage()  # Start a new page
            y_start = page_height - margin - qr_size  # Reset y_start for the new page

        x = x_start + col * (qr_size + spacing)
        y = y_start - row * (qr_size + spacing)

        # Generate QR code
        qr_data = f"overlordcard/{keyword}"
        qr = qrcode.make(qr_data)
        qr_filename = f"{keyword}.png"
        qr.save(qr_filename)

        # Draw the QR code and keyword
        pdf.drawImage(qr_filename, x, y, width=qr_size, height=qr_size)
        pdf.drawString(x, y - font_size - 2, keyword)

    pdf.save()


try:
    cards = read_overlord_cards_from_file("overlord_cards.json")
    print(f"Got {len(cards.keys())} overlord card keywords")
    keywords = []
    for key in cards.keys():
        for i in range(cards[key]["occurences"]):
            keywords.append(key)
    print(f"Printing out qr codes for {len(keywords)} cards")
    generate_qr_pdf(keywords, "qrcodes.pdf")
except Exception as e:
    print(f"Error: {e}")
