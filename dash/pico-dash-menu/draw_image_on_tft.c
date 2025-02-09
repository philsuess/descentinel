#include <stdio.h>
#include "pico/stdlib.h"
#include "hardware/spi.h"
#include "hw.h"
#include "ST7735_TFT.h"

#include "images/Menu_Leben_image_data.h"

void init_hw()
{
    stdio_init_all();
    spi_init(SPI_PORT, 16000000); // SPI with 1Mhz
    gpio_set_function(SPI_RX, GPIO_FUNC_SPI);
    gpio_set_function(SPI_SCK, GPIO_FUNC_SPI);
    gpio_set_function(SPI_TX, GPIO_FUNC_SPI);
    tft_spi_init();
}

void draw_image(uint8_t width, uint8_t height, const uint16_t *pixel_data)
{
    const uint16_t *pixel = pixel_data;
    for (uint8_t j = 0; j < height; j++)
    {
        for (uint8_t i = 0; i < width; i++, pixel++)
        {
            drawPixel(i, j, *pixel);
        }
    }
}

void draw_value_left(const char *value)
{
    setRotation(3);
    drawText(62, 70, value, ST7735_WHITE, ST7735_BLACK, 1);
    setRotation(0);
}

void draw_value_right(const char *value)
{
    setRotation(3);
    drawText(124, 70, value, ST7735_WHITE, ST7735_BLACK, 1);
    setRotation(0);
}

void draw_life_screen(int life, int stamina)
{
    draw_image(MENU_LEBEN_IMAGE_WIDTH, MENU_LEBEN_IMAGE_HEIGHT, Menu_Leben_image_data);
    char buffer[12];
    sprintf(buffer, "%d", life);
    draw_value_left(buffer);
    sprintf(buffer, "%d", stamina);
    draw_value_right(buffer);
}

void draw_rectangle_for_testing()
{
    drawRectWH(5, 5, 125, 155, ST7735_CYAN);
}

void run_test()
{
    for (size_t i = 0; i < 4; i++)
    {
        setRotation(i);
        fillScreen(ST7735_BLACK);
        drawText(10, 10, "Test over!", ST7735_WHITE, ST7735_BLACK, 1);
        drawFastHLine(0, 0, 80, ST7735_CYAN);
        drawFastHLine(0, 25, 80, ST7735_CYAN);
        drawFastVLine(0, 0, 25, ST7735_CYAN);
        drawFastVLine(80, 0, 25, ST7735_CYAN);
    }
}

int main()
{
    init_hw();
#ifdef TFT_ENABLE_BLACK
    TFT_BlackTab_Initialize();
#elif defined(TFT_ENABLE_GREEN)
    TFT_GreenTab_Initialize();
#elif defined(TFT_ENABLE_RED)
    TFT_RedTab_Initialize();
#elif defined(TFT_ENABLE_GENERIC)
    TFT_ST7735B_Initialize();
#endif
    setTextWrap(true);
    fillScreen(ST7735_BLACK);
    setRotation(0);

    draw_life_screen(12, 4);
    // draw_rectangle_for_testing();
    //   run_test();

    while (1)
    {
        sleep_ms(1000);
    }

    return 0;
}
