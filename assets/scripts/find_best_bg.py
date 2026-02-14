from PIL import Image
from collections import Counter
import numpy as np
import os

def count_components(mask):
    # Simple 4-connectivity component count using a flood fill style
    h, w = mask.shape
    visited = np.zeros_like(mask, dtype=bool)
    count = 0
    ys, xs = np.where(mask)
    for y, x in zip(ys, xs):
        if visited[y, x]: continue
        count += 1
        stack = [(y, x)]
        visited[y, x] = True
        while stack:
            cy, cx = stack.pop()
            for dy, dx in [(-1,0),(1,0),(0,-1),(0,1)]:
                ny, nx = cy+dy, cx+dx
                if 0 <= ny < h and 0 <= nx < w and mask[ny, nx] and not visited[ny, nx]:
                    visited[ny, nx] = True
                    stack.append((ny, nx))
    return count

path = 'assets/sprites/recovered_assets/tunnel_miner_spritesheet_1771011926616.png'
img = Image.open(path).convert('RGB')
rgba = np.array(img)
data = [tuple(p) for p in rgba.reshape(-1, 3)]
common = Counter(data).most_common(10)

print(f"Testing top colors for {path}...")
for color, freq in common:
    mask = np.any(rgba != color, axis=-1)
    # Filter noise
    comp_count = count_components(mask)
    print(f"Color {color} (freq {freq}): {comp_count} components")
