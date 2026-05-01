board_runner_args(jlink
  "--device=S32K142"
  "--speed=4000"
  "--iface=swd"
  "--reset"
)

include(${ZEPHYR_BASE}/boards/common/jlink.board.cmake)
