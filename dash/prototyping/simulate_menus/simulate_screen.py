from pathlib import Path
import pygame

# Initialize Pygame
pygame.init()

# Set up the screen with 160x128 resolution
screen = pygame.display.set_mode((160, 128))

# Set window title
pygame.display.set_caption("160x128 Pixel Screen Simulation")


def draw_value(center_x, center_y, value: str):
    value_image = pygame.image.load(Path.cwd() / "value_circle_18px.png")
    value_image_rect = value_image.get_rect(center=(center_x, center_y))
    screen.blit(value_image, value_image_rect)
    font = pygame.font.SysFont(None, 15)
    # Render the number inside the circle
    text = font.render(str(value), True, (255, 255, 255))
    text_rect = text.get_rect(center=(center_x, center_y))
    screen.blit(text, text_rect)


def draw_title(center_x=80, center_y=11, title: str = "Titel"):
    font = pygame.font.SysFont(None, 15)
    text = font.render(title, True, (0, 0, 0))
    text_rect = text.get_rect(center=(center_x, center_y))
    screen.blit(text, text_rect)


def action_to_value(used: bool) -> str:
    return "X" if used else "O"
    # return "\u2714" if used else "\u2718"


def draw_icon(
    center_x: int,
    center_y: int,
    value: str,
    value_offset_x: int,
    value_offset_y: int,
    image: Path,
):
    icon_image = pygame.image.load(image)
    image_rect = icon_image.get_rect(center=(center_x, center_y))
    screen.blit(icon_image, image_rect)
    draw_value(center_x + value_offset_x, center_y + value_offset_y, value)


def draw_two_symbols(
    title: str,
    left_image: Path,
    left_value: str,
    left_value_offset_x: int,
    left_value_offset_y: int,
    right_image: Path,
    right_value: str,
    right_value_offset_x: int,
    right_value_offset_y: int,
):
    draw_title(title=title)
    draw_icon(
        center_x=50,
        center_y=60,
        value=left_value,
        image=left_image,
        value_offset_x=left_value_offset_x,
        value_offset_y=left_value_offset_y,
    )
    draw_icon(
        center_x=110,
        center_y=60,
        value=right_value,
        image=right_image,
        value_offset_x=right_value_offset_x,
        value_offset_y=right_value_offset_y,
    )


def draw_one_symbol(
    title: str, image: Path, value: str, value_offset_x: int, value_offset_y: int
):
    draw_title(title=title)
    draw_icon(
        center_x=80,
        center_y=60,
        value=value,
        image=image,
        value_offset_x=value_offset_x,
        value_offset_y=value_offset_y,
    )


# Loop to keep the window open
running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    # Fill the screen with a color (RGB)
    # screen.fill((81, 68, 15))
    background_image = pygame.image.load(Path.cwd() / "background_160x128px.png")
    background_image_rect = background_image.get_rect()
    screen.blit(background_image, background_image_rect)

    # draw_two_symbols(
    #     title="Leben",
    #     left_image=Path.cwd() / "heart_40px.png",
    #     left_value=str(18),
    #     left_value_offset_x=12,
    #     left_value_offset_y=15,
    #     right_image=Path.cwd() / "tear_40px.png",
    #     right_value=str(8),
    #     right_value_offset_x=12,
    #     right_value_offset_y=15,
    # )
    # draw_two_symbols(
    #     title="Geld (1/2)",
    #     left_image=Path.cwd() / "Coin_25_50px.png",
    #     left_value=str(3),
    #     left_value_offset_x=12,
    #     left_value_offset_y=15,
    #     right_image=Path.cwd() / "Coin_100_50px.png",
    #     right_value=str(2),
    #     right_value_offset_x=12,
    #     right_value_offset_y=15,
    # )
    # draw_two_symbols(
    #     title="Geld (2/2)",
    #     left_image=Path.cwd() / "Coin_500_50px.png",
    #     left_value=str(3),
    #     left_value_offset_x=12,
    #     left_value_offset_y=15,
    #     right_image=Path.cwd() / "Coin_2500_50px.png",
    #     right_value=str(2),
    #     right_value_offset_x=12,
    #     right_value_offset_y=15,
    # )
    # draw_two_symbols(
    #     title="Tränke (1/3)",
    #     left_image=Path.cwd() / "Potion_life_50px.png",
    #     left_value=str(2),
    #     left_value_offset_x=18,
    #     left_value_offset_y=18,
    #     right_image=Path.cwd() / "Potion_endurance_50px.png",
    #     right_value=str(1),
    #     right_value_offset_x=18,
    #     right_value_offset_y=18,
    # )
    # draw_two_symbols(
    #     title="Tränke (2/3)",
    #     left_image=Path.cwd() / "Potion_power_50px.png",
    #     left_value=str(0),
    #     left_value_offset_x=18,
    #     left_value_offset_y=18,
    #     right_image=Path.cwd() / "Potion_invisibility_50px.png",
    #     right_value=str(2),
    #     right_value_offset_x=18,
    #     right_value_offset_y=18,
    # )
    # draw_one_symbol(
    #     title="Tränke (3/3)",
    #     image=Path.cwd() / "Potion_invulnerability_50px.png",
    #     value=str(1),
    #     value_offset_x=18,
    #     value_offset_y=18,
    # )
    # draw_two_symbols(
    #     title="Aktion (1/2)",
    #     left_image=Path.cwd() / "Action_advance_50px.png",
    #     left_value=action_to_value(True),
    #     left_value_offset_x=17,
    #     left_value_offset_y=22,
    #     right_image=Path.cwd() / "Action_fight_50px.png",
    #     right_value=action_to_value(False),
    #     right_value_offset_x=17,
    #     right_value_offset_y=22,
    # )
    # draw_two_symbols(
    #     title="Aktion (2/2)",
    #     left_image=Path.cwd() / "Action_run_50px.png",
    #     left_value=action_to_value(False),
    #     left_value_offset_x=17,
    #     left_value_offset_y=22,
    #     right_image=Path.cwd() / "Action_Command_50px.png",
    #     right_value=action_to_value(True),
    #     right_value_offset_x=17,
    #     right_value_offset_y=22,
    # )
    # draw_two_symbols(
    #     title="Befehl (1/3)",
    #     left_image=Path.cwd() / "Command_aim_38px.png",
    #     left_value=action_to_value(False),
    #     left_value_offset_x=17,
    #     left_value_offset_y=19,
    #     right_image=Path.cwd() / "Command_evade_38px.png",
    #     right_value=action_to_value(False),
    #     right_value_offset_x=17,
    #     right_value_offset_y=19,
    # )
    # draw_two_symbols(
    #     title="Befehl (2/3)",
    #     left_image=Path.cwd() / "Command_rest_38px.png",
    #     left_value=action_to_value(False),
    #     left_value_offset_x=17,
    #     left_value_offset_y=19,
    #     right_image=Path.cwd() / "Command_guard_38px.png",
    #     right_value=action_to_value(True),
    #     right_value_offset_x=17,
    #     right_value_offset_y=19,
    # )
    # draw_one_symbol(
    #     title="Befehl (3/3)",
    #     image=Path.cwd() / "Command_prolonged_38px.png",
    #     value=action_to_value(False),
    #     value_offset_x=17,
    #     value_offset_y=19,
    # )
    draw_two_symbols(
        title="Zustand (1/4)",
        left_image=Path.cwd() / "Status_poisoned_50px.png",
        left_value=str(5),
        left_value_offset_x=18,
        left_value_offset_y=18,
        right_image=Path.cwd() / "Status_stunned_50px.png",
        right_value=action_to_value(False),
        right_value_offset_x=18,
        right_value_offset_y=18,
    )
    # draw_two_symbols(
    #     title="Zustand (2/4)",
    #     left_image=Path.cwd() / "Status_webbed_50px.png",
    #     left_value=str(2),
    #     left_value_offset_x=18,
    #     left_value_offset_y=18,
    #     right_image=Path.cwd() / "Status_burned_50px.png",
    #     right_value=str(2),
    #     right_value_offset_x=18,
    #     right_value_offset_y=18,
    # )
    # draw_two_symbols(
    #     title="Zustand (3/4)",
    #     left_image=Path.cwd() / "Status_bleeding_50px.png",
    #     left_value=str(2),
    #     left_value_offset_x=18,
    #     left_value_offset_y=18,
    #     right_image=Path.cwd() / "Status_dazed_50px.png",
    #     right_value=str(2),
    #     right_value_offset_x=18,
    #     right_value_offset_y=18,
    # )
    # draw_two_symbols(
    #     title="Zustand (4/4)",
    #     left_image=Path.cwd() / "Status_frozen_50px.png",
    #     left_value=str(2),
    #     left_value_offset_x=18,
    #     left_value_offset_y=18,
    #     right_image=Path.cwd() / "Status_cursed_50px.png",
    #     right_value=str(2),
    #     right_value_offset_x=18,
    #     right_value_offset_y=18,
    # )
    # draw_two_symbols(
    #     title="Training (1/2)",
    #     left_image=Path.cwd() / "Training_melee_50px.png",
    #     left_value=str(0),
    #     left_value_offset_x=18,
    #     left_value_offset_y=18,
    #     right_image=Path.cwd() / "Training_ranged_50px.png",
    #     right_value=str(1),
    #     right_value_offset_x=18,
    #     right_value_offset_y=18,
    # )
    # draw_one_symbol(
    #     title="Training (2/2)",
    #     image=Path.cwd() / "Training_magic_50px.png",
    #     value=str(0),
    #     value_offset_x=18,
    #     value_offset_y=18,
    # )

    # Update the display
    pygame.display.flip()
    pygame.display.flip()

    pygame.display.flip()

# Quit Pygame
pygame.quit()
