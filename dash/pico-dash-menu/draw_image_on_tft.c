#include "ST7735_TFT.h"
#include "hardware/spi.h"
#include "hw.h"
#include "pico/cyw43_arch.h"
#include "pico/stdlib.h"
#include <stdio.h>

#include "screens.h"
#include "ssid_secrets.h"

#define INC_LEFT_BUTTON_PIN 17
#define DEC_LEFT_BUTTON_PIN 16
#define INC_RIGHT_BUTTON_PIN 19
#define DEC_RIGHT_BUTTON_PIN 18
#define NEXT_SCREEN_BUTTON_PIN 20
#define PREVIOUS_SCREEN_BUTTON_PIN 21

#define CONNECETED_LED_PIN 5
#define HERO_READY_LED_PIN 4
#define HERO_DONE_LED_PIN 3
#define HERO_ORDER_LED_PIN 2

#define CONNECTION_CONNECTED 2
#define CONNECTION_DISCONNECTED 1
#define CONNECTION_OFFLINE 0

struct hero_stats {
  uint8_t life;
  uint8_t stamina;
  uint8_t coin25;
  uint8_t coin100;
  uint8_t coin500;
  uint8_t coin2500;
  uint8_t potion_life;
  uint8_t potion_stamina;
  uint8_t potion_power;
  uint8_t potion_invisibility;
  uint8_t potion_invulnerability;
  uint8_t status_poisoned;
  uint8_t status_stunned;
  uint8_t status_webbed;
  uint8_t status_burned;
  uint8_t status_bleeding;
  uint8_t status_dazed;
  uint8_t status_frozen;
  uint8_t status_cursed;
  uint8_t action_advance;
  uint8_t action_fight;
  uint8_t action_run;
  uint8_t command_evade;
  uint8_t command_aim;
  uint8_t command_prolonged;
  uint8_t command_rest;
  uint8_t command_guard;
  uint8_t training_melee;
  uint8_t training_ranged;
  uint8_t training_magic;

  uint8_t *current_left_value;
  uint8_t *current_right_value;
  int current_screen;

  uint8_t connected_state;
  uint8_t number_of_consecutive_unsuccessful_connection_attempts;
};

void switch_to_new_screen(struct hero_stats *player_stats) {
  switch (player_stats->current_screen) {
  case LIFE_SCREEN:
    player_stats->current_left_value = &player_stats->life;
    player_stats->current_right_value = &player_stats->stamina;
    break;
  case COIN_SMALL_SCREEN:
    player_stats->current_left_value = &player_stats->coin25;
    player_stats->current_right_value = &player_stats->coin100;
    break;
  case COIN_BIG_SCREEN:
    player_stats->current_left_value = &player_stats->coin500;
    player_stats->current_right_value = &player_stats->coin2500;
    break;
  case POTIONS_1_SCREEN:
    player_stats->current_left_value = &player_stats->potion_life;
    player_stats->current_right_value = &player_stats->potion_stamina;
    break;
  case POTIONS_2_SCREEN:
    player_stats->current_left_value = &player_stats->potion_power;
    player_stats->current_right_value = &player_stats->potion_invisibility;
    break;
  case POTIONS_3_SCREEN:
    player_stats->current_left_value = &player_stats->potion_invulnerability;
    player_stats->current_right_value = &player_stats->potion_invulnerability;
    break;
  case STATUS_1_SCREEN:
    player_stats->current_left_value = &player_stats->status_poisoned;
    player_stats->current_right_value = &player_stats->status_stunned;
    break;
  case STATUS_2_SCREEN:
    player_stats->current_left_value = &player_stats->status_webbed;
    player_stats->current_right_value = &player_stats->status_burned;
    break;
  case STATUS_3_SCREEN:
    player_stats->current_left_value = &player_stats->status_bleeding;
    player_stats->current_right_value = &player_stats->status_dazed;
    break;
  case STATUS_4_SCREEN:
    player_stats->current_left_value = &player_stats->status_frozen;
    player_stats->current_right_value = &player_stats->status_cursed;
    break;
  case ACTION_1_SCREEN:
    player_stats->current_left_value = &player_stats->action_advance;
    player_stats->current_right_value = &player_stats->action_fight;
    break;
  case ACTION_2_SCREEN:
    player_stats->current_left_value = &player_stats->action_run;
    player_stats->current_right_value = &player_stats->command_evade;
    break;
  case ACTION_3_SCREEN:
    player_stats->current_left_value = &player_stats->command_aim;
    player_stats->current_right_value = &player_stats->command_prolonged;
    break;
  case ACTION_4_SCREEN:
    player_stats->current_left_value = &player_stats->command_rest;
    player_stats->current_right_value = &player_stats->command_guard;
    break;
  case TRAINING_1_SCREEN:
    player_stats->current_left_value = &player_stats->training_melee;
    player_stats->current_right_value = &player_stats->training_ranged;
    break;
  case TRAINING_2_SCREEN:
    player_stats->current_left_value = &player_stats->training_magic;
    player_stats->current_right_value = &player_stats->training_magic;
    break;
  }
}

void initialize_hero_state(struct hero_stats *hero) {
  hero->life = 12;
  hero->stamina = 4;
  hero->coin25 = 0;
  hero->coin100 = 4;
  hero->coin500 = 0;
  hero->coin2500 = 0;
  hero->potion_life = 0;
  hero->potion_stamina = 0;
  hero->potion_power = 0;
  hero->potion_invisibility = 0;
  hero->potion_invulnerability = 0;
  hero->status_poisoned = 1;
  hero->status_stunned = 1;
  hero->status_webbed = 1;
  hero->status_burned = 1;
  hero->status_bleeding = 1;
  hero->status_dazed = 1;
  hero->status_frozen = 1;
  hero->status_cursed = 1;
  hero->action_advance = 0;
  hero->action_fight = 0;
  hero->action_run = 0;
  hero->command_evade = 0;
  hero->command_aim = 0;
  hero->command_prolonged = 0;
  hero->command_rest = 0;
  hero->command_guard = 0;
  hero->training_melee = 0;
  hero->training_ranged = 0;
  hero->training_magic = 0;

  hero->current_screen = LIFE_SCREEN;
  switch_to_new_screen(hero);

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
  case ACTION_1_SCREEN:
    draw_screen_with_two_items(
        Action_1_Header_image_data, Action_1_Navi_image_data,
        Action_Advance_with_value_image_data, player_stats->action_advance,
        Action_Fight_with_value_image_data, player_stats->action_fight);
    break;
  case ACTION_2_SCREEN:
    draw_screen_with_two_items(
        Action_2_Header_image_data, Action_2_Navi_image_data,
        Action_Run_with_value_image_data, player_stats->action_run,
        Command_Evade_with_value_image_data, player_stats->command_evade);
    break;
  case ACTION_3_SCREEN:
    draw_screen_with_two_items(
        Action_3_Header_image_data, Action_3_Navi_image_data,
        Command_Aim_with_value_image_data, player_stats->command_aim,
        Command_Prolonged_with_value_image_data,
        player_stats->command_prolonged);
    break;
  case ACTION_4_SCREEN:
    draw_screen_with_two_items(
        Action_4_Header_image_data, Action_4_Navi_image_data,
        Command_Rest_with_value_image_data, player_stats->command_rest,
        Command_Guard_with_value_image_data, player_stats->command_guard);
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

  handle_increment_left_pressed(player_stats, &changes_made);
  handle_decrement_left_pressed(player_stats, &changes_made);
  handle_increment_right_pressed(player_stats, &changes_made);
  handle_decrement_right_pressed(player_stats, &changes_made);
  handle_next_screen_pressed(player_stats, &changes_made);
  handle_previous_screen_pressed(player_stats, &changes_made);

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
