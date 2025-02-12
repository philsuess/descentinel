# Generated Cmake Pico project file

cmake_minimum_required(VERSION 3.13)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Initialise pico_sdk from installed location
# (note this can come from environment, CMake cache etc)

# == DO NOT EDIT THE FOLLOWING LINES for the Raspberry Pi Pico VS Code Extension to work ==
if(WIN32)
    set(USERHOME $ENV{USERPROFILE})
else()
    set(USERHOME $ENV{HOME})
endif()
set(sdkVersion 2.1.0)
set(toolchainVersion 13_3_Rel1)
set(picotoolVersion 2.1.0)
set(picoVscode ${USERHOME}/.pico-sdk/cmake/pico-vscode.cmake)
if (EXISTS ${picoVscode})
    include(${picoVscode})
endif()
# ====================================================================================
set(PICO_BOARD pico_w CACHE STRING "Board type")

# Pull in Raspberry Pi Pico SDK (must be before project)
include(pico_sdk_import.cmake)

project(hero_dash C CXX ASM)

# Initialise the Raspberry Pi Pico SDK
pico_sdk_init()


set(SPI_TFT_PORT "spi1" CACHE STRING "TFT spi port-name")
set(SPI_TFT_CS   "9"    CACHE STRING "TFT CS  pin number")
set(SPI_TFT_DC   "10"   CACHE STRING "TFT DC  pin number")
set(SPI_TFT_RST  "11"   CACHE STRING "TFT RST pin number")
set(TFT_OPTIONS TFT_ENABLE_BLACK TFT_ENABLE_RESET TFT_ENABLE_TEXT TFT_ENABLE_SHAPES
                TFT_ENABLE_ROTATE TFT_ENABLE_SCROLL
            CACHE STRING "TFT options/functions")

#program configuration
set(SPI_PORT "spi1" CACHE STRING "spi port-name")
set(SPI_RX   "12"   CACHE STRING "RX  pin number")
set(SPI_TX   "15"   CACHE STRING "TX  pin number")
set(SPI_SCK  "14"   CACHE STRING "SCK pin number")

add_compile_definitions(SPI_PORT=${SPI_PORT})
add_compile_definitions(SPI_RX=${SPI_RX})
add_compile_definitions(SPI_TX=${SPI_TX})
add_compile_definitions(SPI_SCK=${SPI_SCK})
foreach(opt IN LISTS TFT_OPTIONS)
    message(">>> using: ${opt}")
    add_compile_definitions(${opt})
endforeach(opt)

# ENABLE_FONTS: set on command-line or select from
# VSCode-menu (see .vscode/cmake-variants.json)
if(ENABLE_FONTS)
  add_compile_definitions(TFT_ENABLE_FONTS)
endif()





# Add executable. Default name is the project name, version 0.1

add_executable(hero_dash
    draw_image_on_tft.c
    )

pico_enable_stdio_uart(${PROJECT_NAME} 1)
pico_enable_stdio_usb(${PROJECT_NAME} 0)

pico_set_program_name(${PROJECT_NAME} "hero_dash")
pico_set_program_version(${PROJECT_NAME} "0.1")

add_subdirectory(libs/st7735)

# pull in common dependencies
target_link_libraries(hero_dash pico_stdlib hardware_spi st7735)

if (PICO_CYW43_SUPPORTED)
    target_link_libraries(hero_dash pico_cyw43_arch_none)
endif()

# create map/bin/hex file etc.
pico_add_extra_outputs(${PROJECT_NAME})

# add url via pico_set_program_url
