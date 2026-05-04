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

## Development

We currently maintain forks of various Zephyr modules while integrating the s32k142 support:

- Zephyr (<https://github.com/inomotech-foss/zephyr>) - Device tree bindings.
- Zephyr HAL NXP (<https://github.com/inomotech-foss/zephyr-hal-nxp>) - Support for the S32K142 MCU.
- Zephyr Lang Rust (<https://github.com/inomotech-foss/zephyr-lang-rust>) - More Rust bindings for the included application.
