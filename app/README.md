# Deploy on Raspberry Pi

## Frontend

1. install `nginx`
1. create production version of view
   1. `pnpm install`
   1. `pnpm run build`
1. Copy content from `dist` folder to `/var/www/html/` on pi

## Backend

1. Create binary for `broadcast`

   1. see [broadcast/README](broadcast/README.md)

1. Create binary for `monitor`

   1. see [monitor/README](monitor/README.md)

1. Create image for `detect_card`
   1. `podman build --target rabbit_service --rm -t detect_card_service .`
