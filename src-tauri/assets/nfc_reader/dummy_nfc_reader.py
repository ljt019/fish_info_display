#!/usr/bin/env python3
"""
Dummy NFC Reader Script

This script simulates an NFC reader by generating a random number between 1 and 5
every 45 seconds and outputting it exactly as the NFC reader would (just the number).
"""

import time
import random

def main():
    """Main function to simulate NFC tag reading."""
    while True:
        # Generate a random number between 1 and 5
        number = random.randint(1, 5)
        # Print the number (no additional output)
        print(number, flush=True)
        # Wait for 45 seconds before generating the next number
        time.sleep(45)

if __name__ == '__main__':
    main()
