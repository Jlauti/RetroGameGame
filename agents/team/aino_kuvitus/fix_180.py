#!/usr/bin/env python3
import os
from PIL import Image

TARGET_DIR = "/home/jl/git/RetroGameGame/assets/sprites/future/nebula_bouncer"

# Files to rotate 180 degrees to fix the "looks like you rotated them 180 degrees" issue
FILES_TO_FIX = [
    "sprite_player_ship.png",
    "sprite_enemy_scout.png",
    "sprite_enemy_heavy.png",
    "sprite_enemy_interceptor.png",
    "vfx_projectile_core.png"
]

def fix_rotation():
    for filename in FILES_TO_FIX:
        path = os.path.join(TARGET_DIR, filename)
        if not os.path.exists(path):
            print(f"Skipping {filename}: Not found.")
            continue
            
        print(f"Rotating {filename} 180 degrees...")
        with Image.open(path) as img:
            rotated = img.rotate(180, expand=False)
            rotated.save(path)

if __name__ == "__main__":
    fix_rotation()
    print("Corrections applied.")
