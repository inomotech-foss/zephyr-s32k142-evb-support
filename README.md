# Zephyr S32K142 support

Instead of cloning this repository directly, prefer using `west` to set up a Zephyr workspace.

```bash
# This is the directory containing the whole Zephyr workspace.
# Use whatever name you like!
ZEPHYR_WORKSPACE_DIR="zephyr-s32k142"
west init -m https://github.com/inomotech-foss/zephyr-s32k142-evb-support "$ZEPHYR_WORKSPACE_DIR"
cd "$ZEPHYR_WORKSPACE_DIR/app"
west update
```

Build and flash the included application:

```bash
west build -b s32k142_evb app
west flash
```
