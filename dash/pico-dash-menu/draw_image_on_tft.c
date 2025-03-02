#include "ST7735_TFT.h"
#include "hardware/spi.h"
#include "hw.h"
#include "pico/cyw43_arch.h"
#include "pico/stdlib.h"
#include <stdio.h>

#include "action_state_machine.h"
#include "enums.h"
#include "hero.h"
#include "screens.h"
#include "ssid_secrets.h"

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

  gpio_init(NEXT_SCREEN_BUTTON_PIN);
  gpio_set_dir(NEXT_SCREEN_BUTTON_PIN, GPIO_IN);
  gpio_pull_up(NEXT_SCREEN_BUTTON_PIN);

  gpio_init(PREVIOUS_SCREEN_BUTTON_PIN);
  gpio_set_dir(PREVIOUS_SCREEN_BUTTON_PIN, GPIO_IN);
  gpio_pull_up(PREVIOUS_SCREEN_BUTTON_PIN);
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

void switch_to_next_screen(struct hero_stats *player_stats) {
  if (player_stats->current_screen == LAST_SCREEN) {
    player_stats->current_screen = FIRST_SCREEN;
  } else {
    player_stats->current_screen += 1;
  }
  switch_to_new_screen(player_stats);
}

void switch_to_previous_screen(struct hero_stats *player_stats) {
  if (player_stats->current_screen == FIRST_SCREEN) {
    player_stats->current_screen = LAST_SCREEN;
  } else {
    player_stats->current_screen -= 1;
  }
  switch_to_new_screen(player_stats);
}

void draw_image_at(uint8_t x, uint8_t y, uint8_t image_width,
                   uint8_t image_height, const uint16_t *image_data) {
  const uint16_t *pixel = image_data;
  for (uint8_t j = y; j < image_height + y; j++) {
    for (uint8_t i = x; i < image_width + x; i++, pixel++) {
      drawPixel(i, j, *pixel);
    }
  }
}

void draw_image(uint8_t width, uint8_t height, const uint16_t *pixel_data) {
  draw_image_at(0, 0, width, height, pixel_data);
}

void draw_background() {
  draw_image(BACKGROUND_160X128PX_IMAGE_WIDTH,
             BACKGROUND_160X128PX_IMAGE_HEIGHT,
             background_160x128px_image_data);
}

void draw_background_with_header_and_navi(const uint16_t *header_image_data,
                                          const uint16_t *navi_image_data) {
  draw_background();
  draw_image_at(HEADER_X, HEADER_Y, HEADER_IMAGE_WIDTH, HEADER_IMAGE_HEIGHT,
                header_image_data);
  draw_image_at(NAVI_X, NAVI_Y, NAVI_IMAGE_WIDTH, NAVI_IMAGE_HEIGHT,
                navi_image_data);
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

void draw_value_center(const char *value) {
  setRotation(3);
  drawText(102, 70, value, ST7735_WHITE, ST7735_BLACK, 1);
  setRotation(0);
}

void draw_screen_with_two_items(const uint16_t *header_image_data,
                                const uint16_t *navi_image_data,
                                const uint16_t *left_item_image_data,
                                uint8_t left_value,
                                const uint16_t *right_item_image_data,
                                uint8_t right_value) {
  draw_background_with_header_and_navi(header_image_data, navi_image_data);
  draw_image_at(LEFT_ICON_X, LEFT_ICON_Y, LEFT_ICON_WIDTH, LEFT_ICON_HEIGHT,
                left_item_image_data);
  draw_image_at(RIGHT_ICON_X, RIGHT_ICON_Y, RIGHT_ICON_WIDTH, RIGHT_ICON_HEIGHT,
                right_item_image_data);
  char buffer[12];
  sprintf(buffer, "%d", left_value);
  draw_value_left(buffer);
  sprintf(buffer, "%d", right_value);
  draw_value_right(buffer);
}

void draw_screen_with_center_item(const uint16_t *header_image_data,
                                  const uint16_t *navi_image_data,
                                  const uint16_t *item_image_data,
                                  uint8_t value) {
  draw_background_with_header_and_navi(header_image_data, navi_image_data);
  draw_image_at(CENTER_ICON_X, CENTER_ICON_Y, CENTER_ICON_WIDTH,
                CENTER_ICON_HEIGHT, item_image_data);
  char buffer[12];
  sprintf(buffer, "%d", value);
  draw_value_center(buffer);
}

void draw_action_screen() {
  draw_background_with_header_and_navi(Action_Header_image_data,
                                       Action_1_Navi_image_data);
  draw_image_at(LEFT_ICON_X, LEFT_ICON_Y, LEFT_ICON_WIDTH, LEFT_ICON_HEIGHT,
                current_left_half_action_image_data());
  draw_image_at(RIGHT_ICON_X, RIGHT_ICON_Y, LEFT_ICON_WIDTH, LEFT_ICON_HEIGHT,
                current_right_half_action_image_data());
}

void draw_active_effects(struct hero_stats *player_stats) {
  const uint8_t y_coordinate_of_effect_thumbnails[] = {80,  64, 96,  48,
                                                       112, 32, 128, 16};
  uint8_t effects_drawn = 0;
  if (player_stats->status_bleeding > 0) {
    draw_image_at(90, y_coordinate_of_effect_thumbnails[effects_drawn],
                  STATUS_BLEEDING_15PX_IMAGE_WIDTH,
                  STATUS_BLEEDING_15PX_IMAGE_HEIGHT,
                  Status_bleeding_15px_image_data);
    effects_drawn += 1;
  }
  if (player_stats->status_burned > 0) {
    draw_image_at(90, y_coordinate_of_effect_thumbnails[effects_drawn],
                  STATUS_BURNED_15PX_IMAGE_WIDTH,
                  STATUS_BURNED_15PX_IMAGE_HEIGHT,
                  Status_burned_15px_image_data);
    effects_drawn += 1;
  }
  if (player_stats->status_cursed > 0) {
    draw_image_at(90, y_coordinate_of_effect_thumbnails[effects_drawn],
                  STATUS_CURSED_15PX_IMAGE_WIDTH,
                  STATUS_CURSED_15PX_IMAGE_HEIGHT,
                  Status_cursed_15px_image_data);
    effects_drawn += 1;
  }
  if (player_stats->status_dazed > 0) {
    draw_image_at(90, y_coordinate_of_effect_thumbnails[effects_drawn],
                  STATUS_DAZED_15PX_IMAGE_WIDTH, STATUS_DAZED_15PX_IMAGE_HEIGHT,
                  Status_dazed_15px_image_data);
    effects_drawn += 1;
  }
  if (player_stats->status_frozen > 0) {
    draw_image_at(90, y_coordinate_of_effect_thumbnails[effects_drawn],
                  STATUS_FROZEN_15PX_IMAGE_WIDTH,
                  STATUS_FROZEN_15PX_IMAGE_HEIGHT,
                  Status_frozen_15px_image_data);
    effects_drawn += 1;
  }
  if (player_stats->status_poisoned > 0) {
    draw_image_at(90, y_coordinate_of_effect_thumbnails[effects_drawn],
                  STATUS_POISONED_15PX_IMAGE_WIDTH,
                  STATUS_POISONED_15PX_IMAGE_HEIGHT,
                  Status_poisoned_15px_image_data);
    effects_drawn += 1;
  }
  if (player_stats->status_stunned > 0) {
    draw_image_at(90, y_coordinate_of_effect_thumbnails[effects_drawn],
                  STATUS_STUNNED_15PX_IMAGE_WIDTH,
                  STATUS_STUNNED_15PX_IMAGE_HEIGHT,
                  Status_stunned_15px_image_data);
    effects_drawn += 1;
  }
  if (player_stats->status_webbed > 0) {
    draw_image_at(90, y_coordinate_of_effect_thumbnails[effects_drawn],
                  STATUS_WEBBED_15PX_IMAGE_WIDTH,
                  STATUS_WEBBED_15PX_IMAGE_HEIGHT,
                  Status_webbed_15px_image_data);
    effects_drawn += 1;
  }
}

void handle_increment_left_pressed(struct hero_stats *player_stats,
                                   bool *changes_made) {
  if (is_button_pressed(INC_LEFT_BUTTON_PIN)) {
    if (player_stats->current_screen == ACTION_SCREEN) {
      toggle_left_half_action_next(player_stats);
    } else {
      increment_left_value(player_stats);
    }
    *changes_made = true;
  }
}

void handle_decrement_left_pressed(struct hero_stats *player_stats,
                                   bool *changes_made) {
  if (is_button_pressed(DEC_LEFT_BUTTON_PIN)) {
    if (player_stats->current_screen == ACTION_SCREEN) {
      toggle_left_half_action_previous(player_stats);
    } else {
      decrement_left_value(player_stats);
    }
    *changes_made = true;
  }
}

void handle_increment_right_pressed(struct hero_stats *player_stats,
                                    bool *changes_made) {
  if (is_button_pressed(INC_RIGHT_BUTTON_PIN)) {
    if (player_stats->current_screen == ACTION_SCREEN) {
      toggle_right_half_action_next(player_stats);
    } else {
      increment_right_value(player_stats);
    }
    *changes_made = true;
  }
}

void handle_decrement_right_pressed(struct hero_stats *player_stats,
                                    bool *changes_made) {
  if (is_button_pressed(DEC_RIGHT_BUTTON_PIN)) {
    if (player_stats->current_screen == ACTION_SCREEN) {
      toggle_right_half_action_previous(player_stats);
    } else {
      decrement_right_value(player_stats);
    }
    *changes_made = true;
  }
}

void handle_next_screen_pressed(struct hero_stats *player_stats,
                                bool *changes_made) {
  if (is_button_pressed(NEXT_SCREEN_BUTTON_PIN)) {
    switch_to_next_screen(player_stats);
    *changes_made = true;
  }
}

void handle_previous_screen_pressed(struct hero_stats *player_stats,
                                    bool *changes_made) {
  if (is_button_pressed(PREVIOUS_SCREEN_BUTTON_PIN)) {
    switch_to_previous_screen(player_stats);
    *changes_made = true;
  }
}

void draw_current_screen(struct hero_stats *player_stats) {
  switch (player_stats->current_screen) {
  case LIFE_SCREEN:
    draw_screen_with_two_items(Health_Header_image_data, Health_Navi_image_data,
                               Heart_with_value_image_data, player_stats->life,
                               Tear_with_value_image_data,
                               player_stats->stamina);
    break;
  case COIN_SMALL_SCREEN:
    draw_screen_with_two_items(
        Coin_Small_Header_image_data, Coin_Small_Navi_image_data,
        Coin_25_with_value_image_data, player_stats->coin25,
        Coin_100_with_value_image_data, player_stats->coin100);
    break;
  case COIN_BIG_SCREEN:
    draw_screen_with_two_items(
        Coin_Big_Header_image_data, Coin_Big_Navi_image_data,
        Coin_500_with_value_image_data, player_stats->coin500,
        Coin_2500_with_value_image_data, player_stats->coin2500);
    break;
  case POTIONS_1_SCREEN:
    draw_screen_with_two_items(
        Potion_1_Header_image_data, Potion_1_Navi_image_data,
        Potion_Health_with_value_image_data, player_stats->potion_life,
        Potion_Stamina_with_value_image_data, player_stats->potion_stamina);
    break;
  case POTIONS_2_SCREEN:
    draw_screen_with_two_items(
        Potion_2_Header_image_data, Potion_2_Navi_image_data,
        Potion_Power_with_value_image_data, player_stats->potion_power,
        Potion_Invisibility_with_value_image_data,
        player_stats->potion_invisibility);
    break;
  case POTIONS_3_SCREEN:
    draw_screen_with_center_item(Potion_3_Header_image_data,
                                 Potion_3_Navi_image_data,
                                 Potion_Invulnerability_with_value_image_data,
                                 player_stats->potion_invulnerability);
    break;
  case STATUS_1_SCREEN:
    draw_screen_with_two_items(
        Status_1_Header_image_data, Status_1_Navi_image_data,
        Status_Poisoned_with_value_image_data, player_stats->status_poisoned,
        Status_Stunned_with_value_image_data, player_stats->status_stunned);
    break;
  case STATUS_2_SCREEN:
    draw_screen_with_two_items(
        Status_2_Header_image_data, Status_2_Navi_image_data,
        Status_Webbed_with_value_image_data, player_stats->status_webbed,
        Status_Burned_with_value_image_data, player_stats->status_burned);
    break;
  case STATUS_3_SCREEN:
    draw_screen_with_two_items(
        Status_3_Header_image_data, Status_3_Navi_image_data,
        Status_Bleeding_with_value_image_data, player_stats->status_bleeding,
        Status_Dazed_with_value_image_data, player_stats->status_dazed);
    break;
  case STATUS_4_SCREEN:
    draw_screen_with_two_items(
        Status_4_Header_image_data, Status_4_Navi_image_data,
        Status_Frozen_with_value_image_data, player_stats->status_frozen,
        Status_Cursed_with_value_image_data, player_stats->status_cursed);
    break;
  case ACTION_SCREEN:
    draw_action_screen();
    break;
  case TRAINING_1_SCREEN:
    draw_screen_with_two_items(
        Training_1_Header_image_data, Training_1_Navi_image_data,
        Training_Melee_with_value_image_data, player_stats->training_melee,
        Training_Ranged_with_value_image_data, player_stats->training_ranged);
    break;
  case TRAINING_2_SCREEN:
    draw_screen_with_center_item(
        Training_2_Header_image_data, Training_2_Navi_image_data,
        Training_Magic_with_value_image_data, player_stats->training_magic);
    break;
  default:
    break;
  }
  draw_active_effects(player_stats);
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

void draw_status_message_on_screen(const char *message) {
  setRotation(3);
  drawText(10, 10, message, ST7735_GREEN, ST7735_BLACK, 1);
  setRotation(0);
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

bool check_server_connection() { return false; }

void set_all_connections_off() { cyw43_arch_disable_sta_mode(); }

bool should_not_connect(struct hero_stats *player_stats) {
  if (player_stats->number_of_consecutive_unsuccessful_connection_attempts >=
      MAXIMUM_NUMBER_OF_CONNECTION_ATTEMPTS) {
    set_all_connections_off();
    return true;
  }
  return false;
}

bool check_connection_to_server(struct repeating_timer *t) {
  struct hero_stats *player_stats = (struct hero_stats *)(t->user_data);

  if (should_not_connect(player_stats)) {
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
  bool hero_stunned_before_change = player_stats->status_stunned > 0;

  handle_increment_left_pressed(player_stats, &changes_made);
  handle_decrement_left_pressed(player_stats, &changes_made);
  handle_increment_right_pressed(player_stats, &changes_made);
  handle_decrement_right_pressed(player_stats, &changes_made);
  handle_next_screen_pressed(player_stats, &changes_made);
  handle_previous_screen_pressed(player_stats, &changes_made);

  if (changes_made) {
    draw_current_screen(player_stats);
  }

  if (hero_stunned_before_change && player_stats->status_stunned == 0) {
    toggle_hero_stunned(player_stats);
  } else if (!hero_stunned_before_change && player_stats->status_stunned > 0) {
    toggle_hero_stunned(player_stats);
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
