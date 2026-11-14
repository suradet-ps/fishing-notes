#!/usr/bin/env python3
"""Regenerate the 1200x630 `og-image.png` social-share banner.

Usage:
    python scripts/render-og-image.py
"""

from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "og-image.png"

W, H = 1200, 630
img = Image.new("RGB", (W, H), "#061a20")
draw = ImageDraw.Draw(img, "RGBA")

# Vertical deep-sea gradient.
for y in range(H):
    t = y / (H - 1)
    r = int(6 + (11 - 6) * t)
    g = int(26 + (58 - 26) * t)
    b = int(32 + (66 - 32) * t)
    draw.line([(0, y), (W, y)], fill=(r, g, b, 255))

# Soft wave bands.
for i in range(7):
    y = 470 + i * 24
    color = (94, 234, 212, 10 + i * 3)
    draw.arc([-220 + i * 40, y - 150, 700 + i * 40, y + 80], 200, 340, fill=color, width=2)
    draw.arc([500 + i * 30, y - 170, 1480 + i * 30, y + 90], 200, 340, fill=color, width=2)

# Accent rule.
draw.rectangle([90, 86, 168, 92], fill="#5eead4")

# Hook emblem on the right.
cx, cy = 930, 250
draw.ellipse([cx - 132, cy - 132, cx + 132, cy + 132], outline=(94, 234, 212, 80), width=2)
draw.line([(cx - 30, cy - 92), (cx - 30, cy + 26)], fill="#f4ead9", width=14)
draw.arc([cx - 30, cy + 2, cx + 82, cy + 114], 180, 360, fill="#f4ead9", width=14)
draw.arc([cx - 52, cy - 112, cx - 8, cy - 68], 0, 360, fill="#5eead4", width=8)
draw.line([(cx + 52, cy + 26), (cx + 24, cy + 44)], fill="#5eead4", width=10)
draw.arc([cx - 128, cy + 92, cx + 128, cy + 196], 200, 340, fill="#5eead4", width=5)

# Text.
font_dir = Path(r"C:\Windows\Fonts")
title_font = ImageFont.truetype(str(font_dir / "LeelawUI.ttf"), 74)
thai_font = ImageFont.truetype(str(font_dir / "LeelawUI.ttf"), 34)
small_font = ImageFont.truetype(str(font_dir / "tahoma.ttf"), 25)

draw.text((90, 120), "FISHING NOTES", font=small_font, fill="#5eead4")
draw.text((90, 168), "บันทึกการตกปลา", font=title_font, fill="#f4ead9")
draw.text((90, 254), "เหยื่อปลอมจากคลิปญี่ปุ่น", font=title_font, fill="#f4ead9")
draw.text((90, 372), "ซีบาส · แบส · อีจิ้ง · เท็นยะ", font=thai_font, fill="#a7d9cf")
draw.text((90, 430), "อ่านน้ำให้เป็น แล้วปลาจะตามมา", font=thai_font, fill="#7fb8ae")
draw.text((90, 548), "suradet-ps.github.io/fishing-notes", font=small_font, fill="#6f9d95")

img.save(OUT, "PNG", optimize=True)
print(f"saved {OUT}")
