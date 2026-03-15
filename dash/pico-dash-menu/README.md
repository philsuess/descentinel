This project is for the Raspberry Pi Pico W.

It implements the vision for a Descent hero dashboard ([see here](../README.md)). The gadget requires

- Pico W
- ST7735 screen
- 8 push buttons
- ina219-compatible power source
- lots of wires

# Code setup

Currently, vscode works best, as it is the easiest to configure to work with the Pico.

1. Install vscode and ninja (cmake, gcc etc as well)
2. Add extensions 
    - `raspberry-pi.raspberry-pi-pico`
3. Use extension to compile code

# Wiring

![[../readme-images/pico-dash_schem.png]]