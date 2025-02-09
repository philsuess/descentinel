from pathlib import Path
from PIL import Image
import numpy as np

image_name = "Menu_Leben"

# Open the image (assume it's 128x160 for the display)
image = Image.open((Path.cwd() / f"{image_name}.png").as_posix())
image = image.rotate(90, expand=True)

# Resize image to match the display size (if needed)
# image = image.resize((128, 160))
print(f"image is {image.width}x{image.height}")


# Convert image to RGB565 format (16-bit color)
def convert_to_rgb565(image):
    pixels = []
    test_pixels = np.zeros((image.height, image.width, 3), dtype=np.uint8)
    for y in range(image.height):
        for x in range(image.width):
            r, g, b, _ = image.getpixel((x, y))
            # Convert RGB888 to RGB565
            r5 = (r >> 3) & 0x1F
            g6 = (g >> 2) & 0x3F
            b5 = (b >> 3) & 0x1F
            rgb565 = (r5 << 11) | (g6 << 5) | b5
            pixels.append(rgb565)

            # Convert RGB565 back to RGB888 for debugging
            r_debug = (r5 << 3) | (r5 >> 2)  # Expand 5-bit to 8-bit
            g_debug = (g6 << 2) | (g6 >> 4)  # Expand 6-bit to 8-bit
            b_debug = (b5 << 3) | (b5 >> 2)  # Expand 5-bit to 8-bit

            test_pixels[y, x] = [r_debug, g_debug, b_debug]
    test_image = Image.fromarray(test_pixels, mode="RGB")
    test_image.show()
    return pixels


pixels = convert_to_rgb565(image)

# Save to a file or directly upload the data as a byte array
rgb565_file = Path.cwd() / f"{image_name}.rgb565"
with open(rgb565_file, "wb") as f:
    for pixel in pixels:
        f.write(pixel.to_bytes(2, "big"))

with open(rgb565_file, "rb") as f:
    data = f.read()

# Convert to C array format
c_array = ", ".join(f"0x{data[i]:02X}{data[i+1]:02X}" for i in range(0, len(data), 2))

# Save to a .h file
with open(f"{image_name}_image_data.h", "w") as f:
    f.write(
        f"#ifndef {image_name.upper()}_IMAGE_DATA_H\n#define {image_name.upper()}_IMAGE_DATA_H\n\n"
    )
    f.write(
        f"#define {image_name.upper()}_IMAGE_WIDTH {image.width}\n#define {image_name.upper()}_IMAGE_HEIGHT {image.height}\n\n"
    )
    f.write(
        f"const uint16_t {image_name}_image_data[{image.width * image.height}] = {{\n    {c_array}\n}};\n\n"
    )
    f.write(f"#endif // {image_name.upper()}_IMAGE_DATA_H\n")

print(f"Conversion complete! Check {image_name}_image_data.h")
