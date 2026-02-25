import sys
import os
from PIL import Image

def remove_green_bg(input_path, output_path, tolerance=50, target_size=None):
    """
    Removes the green background from an image and saves it as a transparent PNG.
    Optionally resizes the image.
    """
    if not os.path.exists(input_path):
        print(f"Error: File not found: {input_path}")
        return

    img = Image.open(input_path).convert("RGBA")
    
    # Resize first if target_size is provided (higher quality to do it before alpha removal)
    if target_size:
        img = img.resize(target_size, Image.Resampling.LANCZOS)

    datas = img.getdata()

    new_data = []
    for item in datas:
        # item is (r, g, b, a)
        r, g, b, a = item
        
        # Chromakey logic:
        # If green is significantly higher than red and blue, it's likely the background.
        # Pure green is (0, 255, 0).
        if g > 150 and g > r + tolerance and g > b + tolerance:
            new_data.append((r, g, b, 0))
        else:
            new_data.append(item)

    img.putdata(new_data)
    img.save(output_path, "PNG")
    print(f"Processed: {input_path} -> {output_path} (Size: {img.size})")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python remove_green_bg.py <input_path> <output_path> [tolerance] [width] [height]")
        sys.exit(1)

    input_p = sys.argv[1]
    output_p = sys.argv[2]
    tol = int(sys.argv[3]) if len(sys.argv) > 3 else 50
    
    target_s = None
    if len(sys.argv) > 5:
        target_s = (int(sys.argv[4]), int(sys.argv[5]))

    if os.path.isdir(input_p):
        if not os.path.exists(output_p):
            os.makedirs(output_p)
        for filename in os.listdir(input_p):
            if filename.lower().endswith((".png", ".jpg", ".jpeg")):
                remove_green_bg(
                    os.path.join(input_p, filename),
                    os.path.join(output_p, filename),
                    tol,
                    target_s
                )
    else:
        remove_green_bg(input_p, output_p, tol, target_s)
