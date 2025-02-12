#include "ST7735_TFT.h"
#include "hardware/spi.h"
#include "hw.h"
#include "pico/stdlib.h"
#include <stdio.h>

#include "images/Menu_Leben_image_data.h"

#define INC_LEFT_BUTTON_PIN 17
#define DEC_LEFT_BUTTON_PIN 16
#define INC_RIGHT_BUTTON_PIN 19
#define DEC_RIGHT_BUTTON_PIN 18

struct hero_stats {
  uint8_t life;
  uint8_t stamina;

  uint8_t *current_left_value;
  uint8_t *current_right_value;
};

void init_tft() {
  spi_init(SPI_PORT, 16000000); // SPI with 1Mhz
  gpio_set_function(SPI_RX, GPIO_FUNC_SPI);
  gpio_set_function(SPI_SCK, GPIO_FUNC_SPI);
  gpio_set_function(SPI_TX, GPIO_FUNC_SPI);
  tft_spi_init();
}

void init_buttons() {
  gpio_init(INC_LEFT_BUTTON_PIN);
  gpio_set_dir(INC_LEFT_BUTTON_PIN, GPIO_IN);
  gpio_pull_up(INC_LEFT_BUTTON_PIN);

  gpio_init(DEC_LEFT_BUTTON_PIN);
  gpio_set_dir(DEC_LEFT_BUTTON_PIN, GPIO_IN);
  gpio_pull_up(DEC_LEFT_BUTTON_PIN);

  gpio_init(INC_RIGHT_BUTTON_PIN);
  gpio_set_dir(INC_RIGHT_BUTTON_PIN, GPIO_IN);
  gpio_pull_up(INC_RIGHT_BUTTON_PIN);

  gpio_init(DEC_RIGHT_BUTTON_PIN);
  gpio_set_dir(DEC_RIGHT_BUTTON_PIN, GPIO_IN);
  gpio_pull_up(DEC_RIGHT_BUTTON_PIN);
}

void init_hw() {
  stdio_init_all();
  init_tft();
  init_buttons();
}

bool is_button_pressed(int button_pin) { return !gpio_get(button_pin); }

void increment_left_value(struct hero_stats *player_stats) {
  *(player_stats->current_left_value) += 1;
}

void decrement_left_value(struct hero_stats *player_stats) {
  if (*(player_stats->current_left_value) == 0) {
    return;
  }
  *(player_stats->current_left_value) -= 1;
}

void increment_right_value(struct hero_stats *player_stats) {
  *(player_stats->current_right_value) += 1;
}

void decrement_right_value(struct hero_stats *player_stats) {
  if (*(player_stats->current_right_value) == 0) {
    return;
  }
  *(player_stats->current_right_value) -= 1;
}

void draw_image(uint8_t width, uint8_t height, const uint16_t *pixel_data) {
  const uint16_t *pixel = pixel_data;
  for (uint8_t j = 0; j < height; j++) {
    for (uint8_t i = 0; i < width; i++, pixel++) {
      drawPixel(i, j, *pixel);
    }
  }
}

void draw_value_left(const char *value) {
  setRotation(3);
  drawText(62, 70, value, ST7735_WHITE, ST7735_BLACK, 1);
  setRotation(0);
}

void draw_value_right(const char *value) {
  setRotation(3);
  drawText(124, 70, value, ST7735_WHITE, ST7735_BLACK, 1);
  setRotation(0);
}

void draw_life_screen(struct hero_stats *player_stats) {
  draw_image(MENU_LEBEN_IMAGE_WIDTH, MENU_LEBEN_IMAGE_HEIGHT,
             Menu_Leben_image_data);
  char buffer[12];
  sprintf(buffer, "%d", player_stats->life);
  draw_value_left(buffer);
  sprintf(buffer, "%d", player_stats->stamina);
  draw_value_right(buffer);
}

void handle_increment_left_pressed(struct hero_stats *player_stats,
                                   bool *changes_made) {
  if (is_button_pressed(INC_LEFT_BUTTON_PIN)) {
    increment_left_value(player_stats);
    *changes_made = true;
  }
}

void handle_decrement_left_pressed(struct hero_stats *player_stats,
                                   bool *changes_made) {
  if (is_button_pressed(DEC_LEFT_BUTTON_PIN)) {
    decrement_left_value(player_stats);
    *changes_made = true;
  }
}

void handle_increment_right_pressed(struct hero_stats *player_stats,
                                    bool *changes_made) {
  if (is_button_pressed(INC_RIGHT_BUTTON_PIN)) {
    increment_right_value(player_stats);
    *changes_made = true;
  }
}

void handle_decrement_right_pressed(struct hero_stats *player_stats,
                                    bool *changes_made) {
  if (is_button_pressed(DEC_RIGHT_BUTTON_PIN)) {
    decrement_right_value(player_stats);
    *changes_made = true;
  }
}

void redraw_current_screen(struct hero_stats *player_stats) {
  draw_life_screen(player_stats);
}

bool check_button_pressed_states(struct repeating_timer *t) {
  bool changes_made = false;
  struct hero_stats *player_stats = (struct hero_stats *)(t->user_data);

  handle_increment_left_pressed(player_stats, &changes_made);
  handle_decrement_left_pressed(player_stats, &changes_made);
  handle_increment_right_pressed(player_stats, &changes_made);
  handle_decrement_right_pressed(player_stats, &changes_made);

  if (changes_made) {
    redraw_current_screen(player_stats);
  }

  return true;
}

int main() {
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

  struct hero_stats player;
  player.life = 12;
  player.stamina = 4;
  player.current_left_value = &player.life;
  player.current_right_value = &player.stamina;

  draw_life_screen(&player);

  struct repeating_timer timer;
  add_repeating_timer_ms(200, check_button_pressed_states, &player, &timer);

  while (1) {
  }

  return 0;
}
