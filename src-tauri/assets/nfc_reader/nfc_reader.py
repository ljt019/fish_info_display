#!/usr/bin/env python3
"""
NFC Reader Script to Extract Number After 'en' Using PN532 Module

This script reads the raw memory data of NFC tags, searches for the sequence `en`,
and extracts the number following it. It only prints the extracted number.
"""

import RPi.GPIO as GPIO
import time
import sys
import signal
from pn532 import PN532_SPI  # Importing the PN532_SPI class from the pn532 library

# Configuration Constants
SPI_CS_PIN = 4        # Chip Select pin for SPI (GPIO pin 4)
PN532_RESET_PIN = 20  # Reset pin for PN532 (GPIO pin 20)

def signal_handler(sig, frame):
    """Handle exit signals to ensure GPIO cleanup."""
    GPIO.cleanup()
    sys.exit(0)

def initialize_pn532():
    """Initialize the PN532 module and return the PN532 object."""
    pn532 = PN532_SPI(cs=SPI_CS_PIN, reset=PN532_RESET_PIN, debug=False)
    pn532.SAM_configuration()
    return pn532

def read_and_extract_number(pn532):
    """
    Read the raw memory data from the NFC tag and extract the number after 'en'.

    Args:
        pn532: Initialized PN532 object.
    """
    total_pages = 45
    all_data = []

    for page in range(total_pages):
        block_data = pn532.ntag2xx_read_block(page)
        if block_data is not None:
            all_data.extend(block_data)

    if not all_data:
        return

    # Convert data to bytes for easier searching
    data_bytes = bytes(all_data)

    # Search for the 'en' sequence
    index = data_bytes.find(b'en')
    if index != -1 and index + 2 < len(data_bytes):
        number = data_bytes[index + 2:index + 3].decode('utf-8', errors='ignore')
        if number.isdigit():
            print(number)

def main():
    """Main function to run the NFC reader."""
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

    pn532 = initialize_pn532()

    while True:
        uid = pn532.read_passive_target(timeout=1.0)
        if uid is not None:
            read_and_extract_number(pn532)

            # Wait until the tag is removed before continuing
            while pn532.read_passive_target(timeout=1.0) is not None:
                time.sleep(0.1)

if __name__ == '__main__':
    main()
