# Blabla

Please note that for some reason this process CANNOT be done on macOS. Trust
me, even `mtools` doesn't do the trick here.
You have to do this on either Linux or Windows. The process is the same on all
platforms.

## Preparation

1. Make sure J104 is set to position 1-2.
2. Press and hold the reset button on the board.
3. Connect the USB cable to the board.
4. Verify that you see a new mount named "BOOTLOADER".
5. Open the file "SDA_INFO.HTML" in the "BOOTLOADER" drive to find out the
   correct bootloader version. It will likely be version 1, but make sure to
   check.
6. Download the correct OpenSDA application matching your bootloader version
   from Segger's website: <https://www.segger.com/downloads/jlink/#JLinkOpenSDAGenericFirmwares>
7. Move the downloaded file into the "BOOTLOADER" drive.
8. Make sure to eject the drive cleanly after the file transfer is complete.
9. Unplug and replug the USB cable to the board.
10. Verify that you see a new USB device named "J-Link" and that the "BOOTLOADER" drive is no longer present.
