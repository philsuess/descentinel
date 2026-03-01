#pragma once

#include "enums.h"

struct hero_stats {
  uint8_t hero_ready;

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
  uint8_t action_move;
  uint8_t action_fight;
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
  hero->status_poisoned = 0;
  hero->status_stunned = 0;
  hero->status_webbed = 0;
  hero->status_burned = 0;
  hero->status_bleeding = 0;
  hero->status_dazed = 0;
  hero->status_frozen = 0;
  hero->status_cursed = 0;
  hero->action_move = 0;
  hero->action_fight = 0;
  hero->command_evade = 0;
  hero->command_aim = 0;
  hero->command_prolonged = 0;
  hero->command_rest = 0;
  hero->command_guard = 0;
  hero->training_melee = 0;
  hero->training_ranged = 0;
  hero->training_magic = 0;

  hero->hero_ready = HERO_READY;

  hero->current_screen = LIFE_SCREEN;
  switch_to_new_screen(hero);

  hero->connected_state = CONNECTION_OFFLINE;
  hero->number_of_consecutive_unsuccessful_connection_attempts = 0;
}
