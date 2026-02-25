from PIL import Image, ImageDraw

def generate_ground_tile(output_path, size=64):
    # Deep space black / Near-black metal
    bg_color = (10, 10, 18, 255) # #0a0a12
    grid_color = (26, 26, 46, 255) # #1a1a2e
    
    img = Image.new("RGBA", (size, size), bg_color)
    draw = ImageDraw.Draw(img)
    
    # Draw a faint grid
    # Horizontal line
    draw.line([(0, 0), (size, 0)], fill=grid_color, width=1)
    # Vertical line
    draw.line([(0, 0), (0, size)], fill=grid_color, width=1)
    
    # Add some subtle noise or texture?
    # For now, keep it clean as requested for synthwave
    
    img.save(output_path)
    print(f"Generated ground tile: {output_path}")

def generate_wall_tile(output_path, size=64):
    # Dark metallic panel
    bg_color = (20, 20, 30, 255) # #14141e
    circuit_color = (0, 255, 255, 255) # #00ffff (Cyan)
    accent_color = (155, 89, 240, 255) # #9b59f0 (Purple)
    
    img = Image.new("RGBA", (size, size), bg_color)
    draw = ImageDraw.Draw(img)
    
    # Draw border (to make it look like a panel)
    draw.rectangle([0, 0, size-1, size-1], outline=(40, 40, 50, 255), width=2)
    
    # Draw some "circuit lines"
    # Vertical line
    draw.line([(size//4, 0), (size//4, size)], fill=circuit_color, width=1)
    # Branch
    draw.line([(size//4, size//2), (size//2, size//2)], fill=circuit_color, width=1)
    draw.line([(size//2, size//2), (size//2, size)], fill=circuit_color, width=1)
    
    # Small purple accent dot (LED/component)
    draw.rectangle([size-15, 10, size-10, 15], fill=accent_color)
    
    img.save(output_path)
    print(f"Generated wall tile: {output_path}")

def generate_orb(output_path, size=32):
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    cyan = (0, 255, 255, 255)
    
    # Draw glowing orb (outer halo)
    for i in range(size//2, 0, -1):
        alpha = int(255 * (i / (size//2)))
        r = (size//2) - i
        draw.ellipse([r, r, size-r, size-r], fill=(0, 255, 255, alpha//4))
    
    # Inner bright core
    core_r = size // 4
    draw.ellipse([size//2 - core_r, size//2 - core_r, size//2 + core_r, size//2 + core_r], fill=cyan)
    
    img.save(output_path)
    print(f"Generated player orb: {output_path}")

if __name__ == "__main__":
    generate_ground_tile("assets/sprites/future/nebula_bouncer/sprite_ground_tile.png")
    generate_wall_tile("assets/sprites/future/nebula_bouncer/sprite_wall_tile.png")
    generate_orb("assets/sprites/future/nebula_bouncer/sprite_player_orb.png")
