#pragma once

#include "enums.h"
#include "hero.h"
#include "screens.h"

unsigned char current_action_state = 1;

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
  switch (current_action_state) {
  case 1:
    switch_state_to(2, player_stats);
    break;
  case 2:
    switch_state_to(3, player_stats);
    break;
  case 3:
    switch_state_to(4, player_stats);
    break;
  case 4:
    switch_state_to(5, player_stats);
    break;
  case 5:
    switch_state_to(6, player_stats);
    break;
  case 6:
    switch_state_to(7, player_stats);
    break;
  case 7:
    switch_state_to(1, player_stats);
    break;
  case 8:
    switch_state_to(14, player_stats);
    break;
  case 9:
    switch_state_to(15, player_stats);
    break;
  case 10:
    switch_state_to(16, player_stats);
    break;
  case 11:
    switch_state_to(17, player_stats);
    break;
  case 12:
    switch_state_to(18, player_stats);
    break;
  case 13:
    switch_state_to(19, player_stats);
    break;
  case 14:
    switch_state_to(20, player_stats);
    break;
  case 15:
    switch_state_to(29, player_stats);
    break;
  case 16:
    switch_state_to(30, player_stats);
    break;
  case 17:
    switch_state_to(31, player_stats);
    break;
  case 18:
    switch_state_to(32, player_stats);
    break;
  case 19:
    switch_state_to(21, player_stats);
    break;
  case 20:
    switch_state_to(22, player_stats);
    break;
  case 21:
    switch_state_to(23, player_stats);
    break;
  case 22:
    switch_state_to(24, player_stats);
    break;
  case 23:
    switch_state_to(25, player_stats);
    break;
  case 24:
    switch_state_to(26, player_stats);
    break;
  case 25:
    switch_state_to(27, player_stats);
    break;
  case 26:
    switch_state_to(28, player_stats);
    break;
  case 27:
    switch_state_to(33, player_stats);
    break;
  case 28:
    switch_state_to(8, player_stats);
    break;
  case 29:
    switch_state_to(9, player_stats);
    break;
  case 30:
    switch_state_to(10, player_stats);
    break;
  case 31:
    switch_state_to(11, player_stats);
    break;
  case 32:
    switch_state_to(12, player_stats);
    break;
  case 33:
    switch_state_to(13, player_stats);
    break;
  case 34:
    switch_state_to(35, player_stats);
    break;
  case 35:
    switch_state_to(36, player_stats);
    break;
  case 36:
    switch_state_to(37, player_stats);
    break;
  case 37:
    switch_state_to(38, player_stats);
    break;
  case 38:
    switch_state_to(39, player_stats);
    break;
  case 39:
    switch_state_to(40, player_stats);
    break;
  case 40:
    switch_state_to(34, player_stats);
    break;
  default:
    break;
  }
}

void toggle_left_half_action_previous(struct hero_stats *player_stats) {
  switch (current_action_state) {
  case 1:
    switch_state_to(7, player_stats);
    break;
  case 2:
    switch_state_to(1, player_stats);
    break;
  case 3:
    switch_state_to(2, player_stats);
    break;
  case 4:
    switch_state_to(3, player_stats);
    break;
  case 5:
    switch_state_to(4, player_stats);
    break;
  case 6:
    switch_state_to(5, player_stats);
    break;
  case 7:
    switch_state_to(6, player_stats);
    break;
  case 8:
    switch_state_to(28, player_stats);
    break;
  case 9:
    switch_state_to(29, player_stats);
    break;
  case 10:
    switch_state_to(30, player_stats);
    break;
  case 11:
    switch_state_to(31, player_stats);
    break;
  case 12:
    switch_state_to(32, player_stats);
    break;
  case 13:
    switch_state_to(33, player_stats);
    break;
  case 14:
    switch_state_to(8, player_stats);
    break;
  case 15:
    switch_state_to(9, player_stats);
    break;
  case 16:
    switch_state_to(10, player_stats);
    break;
  case 17:
    switch_state_to(11, player_stats);
    break;
  case 18:
    switch_state_to(12, player_stats);
    break;
  case 19:
    switch_state_to(13, player_stats);
    break;
  case 20:
    switch_state_to(14, player_stats);
    break;
  case 21:
    switch_state_to(19, player_stats);
    break;
  case 22:
    switch_state_to(20, player_stats);
    break;
  case 23:
    switch_state_to(21, player_stats);
    break;
  case 24:
    switch_state_to(22, player_stats);
    break;
  case 25:
    switch_state_to(23, player_stats);
    break;
  case 26:
    switch_state_to(24, player_stats);
    break;
  case 27:
    switch_state_to(25, player_stats);
    break;
  case 28:
    switch_state_to(26, player_stats);
    break;
  case 29:
    switch_state_to(15, player_stats);
    break;
  case 30:
    switch_state_to(16, player_stats);
    break;
  case 31:
    switch_state_to(17, player_stats);
    break;
  case 32:
    switch_state_to(18, player_stats);
    break;
  case 33:
    switch_state_to(27, player_stats);
    break;
  case 34:
    switch_state_to(40, player_stats);
    break;
  case 35:
    switch_state_to(34, player_stats);
    break;
  case 36:
    switch_state_to(35, player_stats);
    break;
  case 37:
    switch_state_to(36, player_stats);
    break;
  case 38:
    switch_state_to(37, player_stats);
    break;
  case 39:
    switch_state_to(38, player_stats);
    break;
  case 40:
    switch_state_to(39, player_stats);
    break;
  default:
    break;
  }
}

void toggle_right_half_action_next(struct hero_stats *player_stats) {
  switch (current_action_state) {
  case 1:
    switch_state_to(8, player_stats);
    break;
  case 2:
    switch_state_to(14, player_stats);
    break;
  case 3:
    switch_state_to(20, player_stats);
    break;
  case 4:
    switch_state_to(22, player_stats);
    break;
  case 5:
    switch_state_to(24, player_stats);
    break;
  case 6:
    switch_state_to(26, player_stats);
    break;
  case 7:
    switch_state_to(28, player_stats);
    break;
  case 8:
    switch_state_to(9, player_stats);
    break;
  case 9:
    switch_state_to(10, player_stats);
    break;
  case 10:
    switch_state_to(11, player_stats);
    break;
  case 11:
    switch_state_to(12, player_stats);
    break;
  case 12:
    switch_state_to(13, player_stats);
    break;
  case 13:
    switch_state_to(1, player_stats);
    break;
  case 14:
    switch_state_to(15, player_stats);
    break;
  case 15:
    switch_state_to(16, player_stats);
    break;
  case 16:
    switch_state_to(17, player_stats);
    break;
  case 17:
    switch_state_to(18, player_stats);
    break;
  case 18:
    switch_state_to(19, player_stats);
    break;
  case 19:
    switch_state_to(2, player_stats);
    break;
  case 20:
    switch_state_to(21, player_stats);
    break;
  case 21:
    switch_state_to(3, player_stats);
    break;
  case 22:
    switch_state_to(23, player_stats);
    break;
  case 23:
    switch_state_to(4, player_stats);
    break;
  case 24:
    switch_state_to(25, player_stats);
    break;
  case 25:
    switch_state_to(5, player_stats);
    break;
  case 26:
    switch_state_to(27, player_stats);
    break;
  case 27:
    switch_state_to(6, player_stats);
    break;
  case 28:
    switch_state_to(29, player_stats);
    break;
  case 29:
    switch_state_to(30, player_stats);
    break;
  case 30:
    switch_state_to(31, player_stats);
    break;
  case 31:
    switch_state_to(32, player_stats);
    break;
  case 32:
    switch_state_to(33, player_stats);
    break;
  case 33:
    switch_state_to(7, player_stats);
    break;
  case 34:
  case 35:
  case 36:
  case 37:
  case 38:
  case 39:
  case 40:
  default:
    break;
  }
}

void toggle_right_half_action_previous(struct hero_stats *player_stats) {
  switch (current_action_state) {
  case 1:
    switch_state_to(13, player_stats);
    break;
  case 2:
    switch_state_to(19, player_stats);
    break;
  case 3:
    switch_state_to(21, player_stats);
    break;
  case 4:
    switch_state_to(23, player_stats);
    break;
  case 5:
    switch_state_to(25, player_stats);
    break;
  case 6:
    switch_state_to(27, player_stats);
    break;
  case 7:
    switch_state_to(33, player_stats);
    break;
  case 8:
    switch_state_to(1, player_stats);
    break;
  case 9:
    switch_state_to(8, player_stats);
    break;
  case 10:
    switch_state_to(9, player_stats);
    break;
  case 11:
    switch_state_to(10, player_stats);
    break;
  case 12:
    switch_state_to(11, player_stats);
    break;
  case 13:
    switch_state_to(12, player_stats);
    break;
  case 14:
    switch_state_to(2, player_stats);
    break;
  case 15:
    switch_state_to(14, player_stats);
    break;
  case 16:
    switch_state_to(15, player_stats);
    break;
  case 17:
    switch_state_to(16, player_stats);
    break;
  case 18:
    switch_state_to(17, player_stats);
    break;
  case 19:
    switch_state_to(18, player_stats);
    break;
  case 20:
    switch_state_to(3, player_stats);
  case 21:
    switch_state_to(20, player_stats);
    break;
  case 22:
    switch_state_to(4, player_stats);
    break;
  case 23:
    switch_state_to(22, player_stats);
    break;
  case 24:
    switch_state_to(5, player_stats);
    break;
  case 25:
    switch_state_to(24, player_stats);
    break;
  case 26:
    switch_state_to(6, player_stats);
    break;
  case 27:
    switch_state_to(26, player_stats);
    break;
  case 28:
    switch_state_to(7, player_stats);
    break;
  case 29:
    switch_state_to(28, player_stats);
    break;
  case 30:
    switch_state_to(29, player_stats);
    break;
  case 31:
    switch_state_to(30, player_stats);
    break;
  case 32:
    switch_state_to(31, player_stats);
    break;
  case 33:
    switch_state_to(32, player_stats);
    break;
  case 34:
  case 35:
  case 36:
  case 37:
  case 38:
  case 39:
  case 40:
    break;
  default:
    break;
  }
}

void toggle_hero_stunned(struct hero_stats *player_stats) {
  switch (current_action_state) {
  case 1:
    switch_state_to(34, player_stats);
    break;
  case 2:
    switch_state_to(35, player_stats);
    break;
  case 3:
    switch_state_to(36, player_stats);
    break;
  case 4:
    switch_state_to(37, player_stats);
    break;
  case 5:
    switch_state_to(38, player_stats);
    break;
  case 6:
    switch_state_to(39, player_stats);
    break;
  case 7:
    switch_state_to(40, player_stats);
    break;
  case 8:
    switch_state_to(34, player_stats);
    break;
  case 9:
    switch_state_to(34, player_stats);
    break;
  case 10:
    switch_state_to(34, player_stats);
    break;
  case 11:
    switch_state_to(34, player_stats);
    break;
  case 12:
    switch_state_to(34, player_stats);
    break;
  case 13:
    switch_state_to(34, player_stats);
    break;
  case 14:
    switch_state_to(35, player_stats);
    break;
  case 15:
    switch_state_to(35, player_stats);
    break;
  case 16:
    switch_state_to(35, player_stats);
    break;
  case 17:
    switch_state_to(35, player_stats);
    break;
  case 18:
    switch_state_to(35, player_stats);
    break;
  case 19:
    switch_state_to(35, player_stats);
    break;
  case 20:
    switch_state_to(36, player_stats);
    break;
  case 21:
    switch_state_to(36, player_stats);
    break;
  case 22:
    switch_state_to(37, player_stats);
    break;
  case 23:
    switch_state_to(37, player_stats);
    break;
  case 24:
    switch_state_to(38, player_stats);
    break;
  case 25:
    switch_state_to(38, player_stats);
    break;
  case 26:
    switch_state_to(39, player_stats);
    break;
  case 27:
    switch_state_to(39, player_stats);
    break;
  case 28:
    switch_state_to(40, player_stats);
    break;
  case 29:
    switch_state_to(40, player_stats);
    break;
  case 30:
    switch_state_to(40, player_stats);
    break;
  case 31:
    switch_state_to(40, player_stats);
    break;
  case 32:
    switch_state_to(40, player_stats);
    break;
  case 33:
    switch_state_to(40, player_stats);
    break;
  case 34:
    switch_state_to(1, player_stats);
    break;
  case 35:
    switch_state_to(2, player_stats);
    break;
  case 36:
    switch_state_to(3, player_stats);
    break;
  case 37:
    switch_state_to(4, player_stats);
    break;
  case 38:
    switch_state_to(5, player_stats);
    break;
  case 39:
    switch_state_to(6, player_stats);
    break;
  case 40:
    switch_state_to(7, player_stats);
    break;

  default:
    break;
  }
}