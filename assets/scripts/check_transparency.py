from PIL import Image
import os

path = 'assets/sprites/recovered_assets/'
files = [f for f in os.listdir(path) if f.endswith('.png')]

for f in files:
    try:
        img = Image.open(os.path.join(path, f)).convert('RGBA')
        alphas = [p[3] for p in img.getdata()]
        min_a = min(alphas)
        max_a = max(alphas)
        count_transparent = alphas.count(0)
        print(f"{f}: MinAlpha={min_a}, MaxAlpha={max_a}, TransparentPixels={count_transparent}, Size={img.size}")
    except Exception as e:
        print(f"{f}: Error {e}")
