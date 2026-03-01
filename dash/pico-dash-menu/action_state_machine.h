#pragma once

#include "enums.h"
#include "hero.h"
#include "screens.h"

unsigned char current_action_state = 1;
const unsigned char left_half_action_next_to_state[] = {
    2,  3,  4,  5,  6,  7,  1,  14, 15, 16, 17, 18, 19, 20,
    29, 30, 31, 32, 21, 22, 23, 24, 25, 26, 27, 28, 33, 8,
    9,  10, 11, 12, 13, 35, 36, 37, 38, 39, 40, 34};
const unsigned char left_half_action_previous_to_state[] = {
    7,  1,  2,  3,  4,  5,  6,  28, 29, 30, 31, 32, 33, 8,
    9,  10, 11, 12, 13, 14, 19, 20, 21, 22, 23, 24, 25, 26,
    15, 16, 17, 18, 27, 40, 34, 35, 36, 37, 38, 39};
const unsigned char right_half_action_next_to_state[] = {
    8,  14, 20, 22, 24, 26, 28, 9,  10, 11, 12, 13, 1, 15,
    16, 17, 18, 19, 2,  21, 3,  23, 4,  25, 5,  27, 6, 29,
    30, 31, 32, 33, 7,  34, 35, 36, 37, 38, 39, 40};
const unsigned char right_half_action_previous_to_state[] = {
    13, 19, 21, 23, 25, 27, 33, 1,  8,  9,  10, 11, 12, 2,
    14, 15, 16, 17, 18, 3,  20, 4,  22, 5,  24, 6,  26, 7,
    28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39};
const unsigned char toggle_hero_stunned_to_state[] = {
    34, 35, 36, 37, 38, 39, 40, 34, 34, 34, 34, 34, 34, 35,
    35, 35, 35, 35, 35, 36, 36, 37, 37, 38, 38, 39, 39, 40,
    40, 40, 40, 40, 40, 1,  2,  3,  4,  5,  6,  7};

void toggle_hero_ready(struct hero_stats *player_stats) {
  if (player_stats->hero_ready == HERO_READY) {
    player_stats->hero_ready = HERO_DONE;
  } else {
    player_stats->hero_ready = HERO_READY;
  }
}

const uint16_t *current_left_half_action_image_data() {
  switch (current_action_state) {
  case 1:
  case 8:
  case 9:
  case 10:
  case 11:
  case 12:
  case 13:
  case 34:
    return Action_advance_50px_image_data;
  case 2:
  case 14:
  case 15:
  case 16:
  case 17:
  case 18:
  case 19:
  case 35:
    return Action_fight_50px_image_data;
  case 3:
  case 20:
  case 21:
  case 36:
    return Command_Guard_with_back_image_data;
  case 4:
  case 22:
  case 23:
  case 37:
    return Command_Rest_with_back_image_data;
  case 5:
  case 24:
  case 25:
  case 38:
    return Command_Evade_with_back_image_data;
  case 6:
  case 26:
  case 27:
  case 39:
    return Command_Aim_with_back_image_data;
  case 7:
  case 28:
  case 29:
  case 30:
  case 31:
  case 32:
  case 33:
  case 40:
    return Command_Prolonged_with_back_image_data;
  default:
    return Action_advance_50px_image_data;
  }
}

const uint16_t *current_right_half_action_image_data() {
  switch (current_action_state) {
  case 1:
  case 2:
  case 3:
  case 4:
  case 5:
  case 6:
  case 7:
    return Action_advance_50px_image_data;
  case 8:
  case 14:
  case 20:
  case 22:
  case 24:
  case 26:
  case 28:
    return Action_fight_50px_image_data;
  case 9:
  case 15:
  case 29:
    return Command_Guard_with_back_image_data;
  case 10:
  case 16:
  case 30:
    return Command_Rest_with_back_image_data;
  case 11:
  case 17:
  case 31:
    return Command_Evade_with_back_image_data;
  case 12:
  case 18:
  case 32:
    return Command_Aim_with_back_image_data;
  case 13:
  case 19:
  case 21:
  case 23:
  case 25:
  case 27:
  case 33:
    return Command_Prolonged_with_back_image_data;
  case 34:
  case 35:
  case 36:
  case 37:
  case 38:
  case 39:
  case 40:
    return second_half_action_unavailable_image_data;
  default:
    return Action_advance_50px_image_data;
  }
}

void reset_actions(struct hero_stats *player_stats) {
  player_stats->action_move = 0;
  player_stats->action_fight = 0;
  player_stats->command_evade = 0;
  player_stats->command_guard = 0;
  player_stats->command_prolonged = 0;
  player_stats->command_rest = 0;
  player_stats->command_aim = 0;
}

void switch_state_to(unsigned char new_state, struct hero_stats *player_stats) {
  current_action_state = new_state;
  reset_actions(player_stats);
  switch (current_action_state) {
  case 1:
    player_stats->action_move = 2;
    break;
  case 2:
    player_stats->action_move = 1;
    player_stats->action_fight = 1;
    break;
  case 3:
    player_stats->action_move = 1;
    player_stats->command_guard = 1;
    break;
  case 4:
    player_stats->action_move = 1;
    player_stats->command_rest = 1;
    break;
  case 5:
    player_stats->action_move = 1;
    player_stats->command_evade = 1;
    break;
  case 6:
    player_stats->action_move = 1;
    player_stats->command_aim = 1;
    break;
  case 7:
    player_stats->action_move = 1;
    player_stats->command_prolonged = 1;
    break;
  case 8:
    player_stats->action_fight = 1;
    player_stats->action_move = 1;
    break;
  case 9:
    player_stats->action_move = 1;
    player_stats->command_guard = 1;
    break;
  case 10:
    player_stats->action_move = 1;
    player_stats->command_rest = 1;
    break;
  case 11:
    player_stats->action_move = 1;
    player_stats->command_evade = 1;
    break;
  case 12:
    player_stats->action_move = 1;
    player_stats->command_aim = 1;
    break;
  case 13:
    player_stats->action_move = 1;
    player_stats->command_prolonged = 1;
    break;
  case 14:
    player_stats->action_fight = 2;
    break;
  case 15:
    player_stats->action_fight = 1;
    player_stats->command_guard = 1;
    break;
  case 16:
    player_stats->action_fight = 1;
    player_stats->command_rest = 1;
    break;
  case 17:
    player_stats->action_fight = 1;
    player_stats->command_evade = 1;
    break;
  case 18:
    player_stats->action_fight = 1;
    player_stats->command_aim = 1;
    break;
  case 19:
    player_stats->action_fight = 1;
    player_stats->command_prolonged = 1;
    break;
  case 20:
    player_stats->command_guard = 1;
    player_stats->action_fight = 1;
    break;
  case 21:
    player_stats->command_guard = 1;
    player_stats->command_prolonged = 1;
    break;
  case 22:
    player_stats->command_rest = 1;
    player_stats->action_fight = 1;
    break;
  case 23:
    player_stats->command_rest = 1;
    player_stats->command_prolonged = 1;
    break;
  case 24:
    player_stats->command_evade = 1;
    player_stats->action_fight = 1;
    break;
  case 25:
    player_stats->command_evade = 1;
    player_stats->command_prolonged = 1;
    break;
  case 26:
    player_stats->command_aim = 1;
    player_stats->action_fight = 1;
    break;
  case 27:
    player_stats->command_aim = 1;
    player_stats->command_prolonged = 1;
    break;
  case 28:
    player_stats->command_prolonged = 1;
    player_stats->action_fight = 1;
    break;
  case 29:
    player_stats->command_prolonged = 1;
    player_stats->command_guard = 1;
    break;
  case 30:
    player_stats->command_prolonged = 1;
    player_stats->command_rest = 1;
    break;
  case 31:
    player_stats->command_prolonged = 1;
    player_stats->command_evade = 1;
    break;
  case 32:
    player_stats->command_prolonged = 1;
    player_stats->command_aim = 1;
    break;
  case 33:
    player_stats->command_prolonged = 2;
    break;
  case 34:
    player_stats->action_move = 1;
    break;
  case 35:
    player_stats->action_fight = 1;
    break;
  case 36:
    player_stats->command_guard = 1;
    break;
  case 37:
    player_stats->command_rest = 1;
    break;
  case 38:
    player_stats->command_evade = 1;
    break;
  case 39:
    player_stats->command_aim = 1;
    break;
  case 40:
    player_stats->command_prolonged = 1;
    break;
  default:
    break;
  }
}

void toggle_left_half_action_next(struct hero_stats *player_stats) {
  switch_state_to(left_half_action_next_to_state[current_action_state - 1],
                  player_stats);
}

void toggle_left_half_action_previous(struct hero_stats *player_stats) {
  switch_state_to(left_half_action_previous_to_state[current_action_state - 1],
                  player_stats);
}

void toggle_right_half_action_next(struct hero_stats *player_stats) {
  switch_state_to(right_half_action_next_to_state[current_action_state - 1],
                  player_stats);
}

void toggle_right_half_action_previous(struct hero_stats *player_stats) {
  switch_state_to(right_half_action_previous_to_state[current_action_state - 1],
                  player_stats);
}

void toggle_hero_stunned(struct hero_stats *player_stats) {
  switch_state_to(toggle_hero_stunned_to_state[current_action_state - 1],
                  player_stats);
}