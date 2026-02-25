"""Generate a hex outline PNG for Nebula Bouncer topography."""
from PIL import Image, ImageDraw
import math
import sys

size = 128
center = size // 2
radius = 54  # Leave margin for anti-aliasing
line_width = 2

img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Pointy-top hex vertices
vertices = []
for i in range(6):
    angle = math.radians(60 * i - 30)
    x = center + radius * math.cos(angle)
    y = center + radius * math.sin(angle)
    vertices.append((x, y))

# Draw hex outline with line segments for thickness control
for i in range(6):
    start = vertices[i]
    end = vertices[(i + 1) % 6]
    draw.line([start, end], fill=(255, 255, 255, 255), width=line_width)

out = r"C:\Users\jlaut\git\RetroGameGame\assets\sprites\future\nebula_bouncer\hex_outline.png"
img.save(out)
print(f"Saved hex outline to {out}")
