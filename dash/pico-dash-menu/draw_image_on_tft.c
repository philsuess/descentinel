#include "ST7735_TFT.h"
#include "hardware/spi.h"
#include "hw.h"
#include "pico/cyw43_arch.h"
#include "pico/stdlib.h"
#include <stdio.h>

#include "images/Menu_Leben_image_data.h"
#include "ssid_secrets.h"

#define INC_LEFT_BUTTON_PIN 17
#define DEC_LEFT_BUTTON_PIN 16
#define INC_RIGHT_BUTTON_PIN 19
#define DEC_RIGHT_BUTTON_PIN 18
#define NEXT_MENU_BUTTON_PIN 20
#define PREVIOUS_MENU_BUTTON_PIN 21

#define CONNECETED_LED_PIN 5
#define HERO_READY_LED_PIN 4
#define HERO_DONE_LED_PIN 3
#define HERO_ORDER_LED_PIN 2

#define FIRST_MENU 0
#define LIFE_MENU 0
#define LAST_MENU 0

#define CONNECTION_CONNECTED 2
#define CONNECTION_DISCONNECTED 1
#define CONNECTION_OFFLINE 0

struct hero_stats {
  uint8_t life;
  uint8_t stamina;

  uint8_t *current_left_value;
  uint8_t *current_right_value;
  int current_menu;

  uint8_t connected_state;
  uint8_t number_of_consecutive_unsuccessful_connection_attempts;
};

void initialize_hero_state(struct hero_stats *hero) {
  hero->life = 12;
  hero->stamina = 4;

  hero->current_left_value = &hero->life;
  hero->current_right_value = &hero->stamina;
  hero->current_menu = LIFE_MENU;

  hero->connected_state = CONNECTION_OFFLINE;
  hero->number_of_consecutive_unsuccessful_connection_attempts = 0;
}

struct timers {
  struct repeating_timer buttons_timer;
  struct repeating_timer server_health_timer;
};

void initialize_led_gpios() {
  gpio_init(CONNECETED_LED_PIN);
  gpio_set_dir(CONNECETED_LED_PIN, GPIO_OUT);

  gpio_init(HERO_READY_LED_PIN);
  gpio_set_dir(HERO_READY_LED_PIN, GPIO_OUT);

  gpio_init(HERO_DONE_LED_PIN);
  gpio_set_dir(HERO_DONE_LED_PIN, GPIO_OUT);

  gpio_init(HERO_ORDER_LED_PIN);
  gpio_set_dir(HERO_ORDER_LED_PIN, GPIO_OUT);
}

void set_led(int led, bool on) { gpio_put(led, on); }

void initialize_button_gpios() {
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

  gpio_init(NEXT_MENU_BUTTON_PIN);
  gpio_set_dir(NEXT_MENU_BUTTON_PIN, GPIO_IN);
  gpio_pull_up(NEXT_MENU_BUTTON_PIN);

  gpio_init(PREVIOUS_MENU_BUTTON_PIN);
  gpio_set_dir(PREVIOUS_MENU_BUTTON_PIN, GPIO_IN);
  gpio_pull_up(PREVIOUS_MENU_BUTTON_PIN);
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

void switch_to_next_menu(struct hero_stats *player_stats) {
  if (player_stats->current_menu == LAST_MENU) {
    player_stats->current_menu = FIRST_MENU;
  } else {
    player_stats->current_menu += 1;
  }
}

void switch_to_previous_menu(struct hero_stats *player_stats) {
  if (player_stats->current_menu == FIRST_MENU) {
    player_stats->current_menu = LAST_MENU;
  } else {
    player_stats->current_menu -= 1;
  }
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

void handle_next_menu_pressed(struct hero_stats *player_stats,
                              bool *changes_made) {
  if (is_button_pressed(NEXT_MENU_BUTTON_PIN)) {
    switch_to_next_menu(player_stats);
    *changes_made = true;
  }
}

void handle_previous_menu_pressed(struct hero_stats *player_stats,
                                  bool *changes_made) {
  if (is_button_pressed(PREVIOUS_MENU_BUTTON_PIN)) {
    switch_to_previous_menu(player_stats);
    *changes_made = true;
  }
}

void draw_current_screen(struct hero_stats *player_stats) {
  switch (player_stats->current_menu) {
  case LIFE_MENU:
    draw_life_screen(player_stats);
    break;

  default:
    break;
  }
}

void set_connection_state(struct hero_stats *player_stats,
                          uint8_t connection_state) {
  player_stats->connected_state = connection_state;
  switch (connection_state) {
  case CONNECTION_CONNECTED:
    player_stats->number_of_consecutive_unsuccessful_connection_attempts = 0;
    set_led(CONNECETED_LED_PIN, true);
    break;
  case CONNECTION_DISCONNECTED:
    player_stats->number_of_consecutive_unsuccessful_connection_attempts += 1;
    set_led(CONNECETED_LED_PIN, false);
    break;

  default:
    set_led(CONNECETED_LED_PIN, false);
    player_stats->number_of_consecutive_unsuccessful_connection_attempts += 1;
    break;
  }
  draw_current_screen(player_stats);
}

bool check_wlan_connection() {
  set_led(CONNECETED_LED_PIN, true);
  if (cyw43_wifi_link_status(&cyw43_state, CYW43_ITF_STA) != CYW43_LINK_DOWN) {
    return true;
  }

  bool connection_success = true;
  set_led(CONNECETED_LED_PIN, false);
  draw_status_message_on_screen("Connecting ...");

  if (cyw43_arch_wifi_connect_timeout_ms(SSID, WLAN_PASSWORD,
                                         CYW43_AUTH_WPA2_AES_PSK, 30000)) {
    connection_success = false;
  }

  return connection_success;
}

void draw_status_message_on_screen(const char *message) {
  setRotation(3);
  drawText(10, 10, message, ST7735_GREEN, ST7735_BLACK, 1);
  setRotation(0);
}
bool check_server_connection() { return false; }

bool check_connection_to_server(struct repeating_timer *t) {
  struct hero_stats *player_stats = (struct hero_stats *)(t->user_data);
  if (player_stats->number_of_consecutive_unsuccessful_connection_attempts >=
      MAXIMUM_NUMBER_OF_CONNECTION_ATTEMPTS) {
    return true;
  }

  if (!check_wlan_connection()) {
    set_connection_state(player_stats, CONNECTION_OFFLINE);
    return true;
  }

  if (!check_server_connection()) {
    set_connection_state(player_stats, CONNECTION_DISCONNECTED);
    return true;
  }

  set_connection_state(player_stats, CONNECTION_CONNECTED);
  return true;
}

void initialize_connection_to_server(struct hero_stats *hero,
                                     struct repeating_timer *timer) {
  add_repeating_timer_ms(CONNECTION_HEALTH_CHECK_FREQUENCY_IN_MS,
                         check_connection_to_server, hero, timer);
}

bool check_button_pressed_states(struct repeating_timer *t) {
  bool changes_made = false;
  struct hero_stats *player_stats = (struct hero_stats *)(t->user_data);

  handle_increment_left_pressed(player_stats, &changes_made);
  handle_decrement_left_pressed(player_stats, &changes_made);
  handle_increment_right_pressed(player_stats, &changes_made);
  handle_decrement_right_pressed(player_stats, &changes_made);
  handle_next_menu_pressed(player_stats, &changes_made);
  handle_previous_menu_pressed(player_stats, &changes_made);

  if (changes_made) {
    draw_current_screen(player_stats);
  }

  return true;
}

void initialize_buttons(struct hero_stats *hero,
                        struct repeating_timer *timer) {
  initialize_button_gpios();
  add_repeating_timer_ms(2, check_button_pressed_states, hero, timer);
}

void initialize_display() {
  spi_init(SPI_PORT, 16000000); // SPI with 1Mhz
  gpio_set_function(SPI_RX, GPIO_FUNC_SPI);
  gpio_set_function(SPI_SCK, GPIO_FUNC_SPI);
  gpio_set_function(SPI_TX, GPIO_FUNC_SPI);
  tft_spi_init();
  TFT_BlackTab_Initialize();
  fillScreen(ST7735_BLACK);
}

void initialize_dashboard(struct hero_stats *hero, struct timers *timers) {
  stdio_init_all();

  cyw43_arch_init_with_country(CYW43_COUNTRY_GERMANY);
  cyw43_arch_enable_sta_mode();

  initialize_display();
  initialize_led_gpios();
  initialize_buttons(hero, &timers->buttons_timer);
  initialize_connection_to_server(hero, &timers->server_health_timer);

  draw_current_screen(hero);
}

int main() {
  struct hero_stats hero;
  initialize_hero_state(&hero);
  struct timers timers;
  initialize_dashboard(&hero, &timers);

  while (1) {
  }

  return 0;
}
