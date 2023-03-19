This is a warp server that broadcasts all backend content, specifically the last item in the rabbitmq queues is provided.

# Routes
- `health`: am I dead?
- `descentinel/`
  - `log`: last log entry
  - `game_room_image`: last image from game room
  - `detected_ol_card`: top secret last detected OL card

# Dev helper
```broadcast --help```

```RUST_LOG=info ...```
