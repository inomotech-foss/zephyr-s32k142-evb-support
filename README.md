# Zephyr S32K142 support

```bash
# This is the directory containing the whole Zephyr workspace.
# Use whatever name you like!
ZEPHYR_WORKSPACE_DIR="zephyr-s32k142"
west init -m https://github.com/inomotech-foss/zephyr-s32k142-evb-support "$ZEPHYR_WORKSPACE_DIR"
cd "$ZEPHYR_WORKSPACE_DIR/zephyr-s32k142-evb-support"
west update
```
