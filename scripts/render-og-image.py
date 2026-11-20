#!/usr/bin/env python3
"""Regenerate the 1200x630 `og-image.png` social-share banner.

Usage:
    python scripts/render-og-image.py

The card uses a real frame from the source videos (the same cover set as the
site) so shared links carry the journal's photography, not a generic logo.
"""

from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "og-image.png"
COVER = ROOT / "content" / "assets" / "spoon-five-seasons.jpg"

W, H = 1200, 630

# Fill the card with the cover, cropping the excess width.
photo = Image.open(COVER).convert("RGB")
scale = max(W / photo.width, H / photo.height)
photo = photo.resize(
    (round(photo.width * scale), round(photo.height * scale)),
    Image.Resampling.LANCZOS,
)
left = (photo.width - W) // 2
top = (photo.height - H) // 2
photo = photo.crop((left, top, left + W, top + H))

# Darken the photo enough for the type, strongest on the left.
shade = Image.new("RGBA", (W, H), (0, 0, 0, 0))
shade_px = shade.load()
for y in range(H):
    for x in range(W):
        # 78% at the left edge, fading out around 72% of the width.
        strength = max(0.0, 1.0 - x / (W * 0.72))
        alpha = int(205 * strength)
        # A little extra darkness toward the bottom.
        alpha = max(alpha, int(90 * (y / H) ** 2))
        shade_px[x, y] = (5, 14, 13, min(alpha, 225))
photo = Image.alpha_composite(photo.convert("RGBA"), shade).convert("RGB")

img = photo
draw = ImageDraw.Draw(img, "RGBA")

accent = "#f08a63"
cream = "#f3eee4"
muted = "#b8c6bf"

font_dir = Path(r"C:\Windows\Fonts")
title_font = ImageFont.truetype(str(font_dir / "LeelaUIb.ttf"), 70)
thai_font = ImageFont.truetype(str(font_dir / "LeelawUI.ttf"), 31)
small_font = ImageFont.truetype(str(font_dir / "tahoma.ttf"), 22)
mono_font = ImageFont.truetype(str(font_dir / "consola.ttf"), 19)

# Accent rule and section label.
draw.rectangle([80, 78, 148, 83], fill=accent)
draw.text((80, 104), "FISHING NOTES  /  FIELD NOTES FROM JAPAN", font=mono_font, fill=accent)

# Headline.
draw.text((78, 158), "อ่านน้ำให้เป็น", font=title_font, fill=cream)
draw.text((78, 244), "แล้วปลาจะตามมา", font=title_font, fill=accent)

# Deck and topics.
draw.text(
    (82, 360),
    "บันทึกตกปลาเหยื่อปลอมจากคลิปโปรญี่ปุ่น",
    font=thai_font,
    fill=cream,
)
draw.text(
    (82, 414),
    "ซีบาส · แบส · อีจิ้ง · เท็นยะ",
    font=thai_font,
    fill=muted,
)

# Footer line.
draw.line([(82, 540), (1118, 540)], fill=(243, 238, 228, 55), width=1)
draw.text((82, 560), "suradet-ps.github.io/fishing-notes", font=small_font, fill=muted)
draw.text((1118, 560), "ภาพจากคลิปต้นฉบับ", font=small_font, fill=muted, anchor="ra")

img.save(OUT, "PNG", optimize=True)
print(f"saved {OUT} ({W}x{H})")
