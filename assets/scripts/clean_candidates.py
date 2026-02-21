import os
import sys
from PIL import Image
from pathlib import Path
import numpy as np

def get_bg_color(img):
    """Sample corners to find the likely background color."""
    w, h = img.size
    samples = []
    s = 10 # Sample size
    # Corners
    samples.append(img.crop((0, 0, s, s)))
    samples.append(img.crop((w-s, 0, w, s)))
    samples.append(img.crop((0, h-s, s, h)))
    samples.append(img.crop((w-s, h-s, w, h)))
    
    colors = []
    for sample in samples:
        arr = np.array(sample.convert("RGB")).reshape(-1, 3)
        colors.extend(arr)
    
    # Return median color to avoid outliers
    return np.median(colors, axis=0)

def clean_image(path, threshold=70.0):
    img = Image.open(path).convert("RGBA")
    arr = np.array(img).astype(np.float32)
    
    # Get the background color from corners
    bg_color = get_bg_color(img)
    print(f"Detected BG color for {path.name}: {bg_color}")
    
    # Calculate distance to background color
    # arr[:,:,:3] is RGB
    diff = arr[:, :, :3] - bg_color
    dist = np.sqrt(np.sum(diff**2, axis=2))
    
    # Mask pixels close to the background color
    mask = dist < threshold
    arr[mask, 3] = 0 # Set alpha to 0
    
    # Convert back to image
    img = Image.fromarray(arr.astype(np.uint8), "RGBA")

    # Now Autocrop and Center
    bbox = img.getbbox()
    if bbox:
        sprite = img.crop(bbox)
        # Create a clean 640x640 canvas
        canvas = Image.new("RGBA", (640, 640), (0, 0, 0, 0))
        
        sw, sh = sprite.size
        # Simple scale if too big
        scale = 1.0
        if sw > 600 or sh > 600:
            scale = 600.0 / max(sw, sh)
            sprite = sprite.resize((int(sw * scale), int(sh * scale)), Image.LANCZOS)
            sw, sh = sprite.size

        # Paste centered
        x = (640 - sw) // 2
        y = (640 - sh) // 2
        canvas.paste(sprite, (x, y))
        canvas.save(path)
        print(f"Cleaned and centered {path} (bbox={bbox}, scale={scale:.2f})")
    else:
        print(f"Warning: {path} is empty after cleaning.")

def main():
    if len(sys.argv) < 2:
        print("Usage: python clean_candidates.py <file_or_dir>")
        return

    target = Path(sys.argv[1])
    if target.is_dir():
        for f in target.glob("*.png"):
            if "cand" in f.name or "decal" in f.name:
                clean_image(f)
    else:
        clean_image(target)

if __name__ == "__main__":
    main()
