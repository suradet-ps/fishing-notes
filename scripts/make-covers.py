#!/usr/bin/env python3
"""Build the 21:9 article covers used by the home page and article headers.

The covers are made from real frames of the source videos.  Each one is
cropped to a cinematic 21:9 ratio (which also removes most broadcast
lower-thirds) and receives a light, consistent grade so stills from different
channels sit together as one editorial set.

Usage:
    python scripts/make-covers.py

Output:
    content/assets/<slug>-cover.jpg

Posts that are not listed here fall back to their first Markdown image, which
is still cropped by CSS.  `sinking-pencil` is intentionally absent: its opening
night shot is already the best cover, so the article renderer reuses it and
moves it out of the body instead.
"""

from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw, ImageEnhance, ImageFilter

ROOT = Path(__file__).resolve().parent.parent
ASSETS = ROOT / "content" / "assets"

# slug -> source frame in content/assets/
COVERS: dict[str, str] = {
    "area-trout-beginner-complete-guide": "area-trout-cover-src.jpg",
    "bass-point-strategy": "bass-point-overhang.jpg",
    "chuzenji-opening-day-2026": "chuzenji-opening-day-first-fish.jpg",
    "chuzenji-tinyklash-dark-hours": "chuzenji-opening.jpg",
    "chuzenji-x9-resolution": "varivas19-lake.jpg",
    "daiwa-silver-creek-stream": "stream-cover.jpg",
    "eging-basics": "eging-basics-cast.jpg",
    "eging-beginner": "eging-beginner-cast.jpg",
    "eging-casting": "eging-casting-cast.jpg",
    "fuefuki-snow-stream-minnowing": "fuefuki-cover-src.jpg",
    "genkai-otoshikomi": "genkai-opening.jpg",
    "hiramoto-7-stream-items": "smith-7-items-belt.jpg",
    "hirose14-rock-shore-layers": "hirose14-rock-shore.jpg",
    "kansai-reservoir-lake-strategy-2026": "kansai-cover-src.jpg",
    "kuma-river-ayu": "kuma-ayu-anglers.jpg",
    "metal-vibration-day-game-guide": "ima-wade.jpg",
    "murata-baitcasting": "murata-bait-overhead2.jpg",
    "murata-spinning-casting": "murata-spin-overhead.jpg",
    "namiki-6-essentials": "namiki-big-fish.jpg",
    "ono-casting-clinic": "ono-casting-crowd.jpg",
    "popper-lily-pad-pond": "popper-pond-catch.jpg",
    "real-steel-steel-upper": "onuma-catch.jpg",
    "ridemarvel-switchback": "switchback-fight.jpg",
    "rivanyoro-feet-bite": "rivanyoro-catch.jpg",
    "sai-river-autumn-trout": "sai-opening.jpg",
    "saltwater-spoon-shore": "saltwater-spoon-catch.jpg",
    "seabass-20hour-challenge": "seabass-20h-night-fish.jpg",
    "seabass-bachi-nuke": "seabass-bachi-overhead.jpg",
    "seabass-preparation": "seabass-prep-lures.jpg",
    "seabass-spoon-shallow-river": "seabass-spoon-river-catch.jpg",
    "shore-metal-vibration": "shore-vibration-cast.jpg",
    "spoon-five-techniques": "spoon-five-seasons.jpg",
    "surface-start-rotation": "surface-start-rotation-strike.jpg",
    "tenya-madai": "tenya-big-fish.jpg",
    "tilapia-popper-koz": "tilapia-koz-fight.jpg",
    "tilapia-territory-okinawa": "tilapia-tb-catch.jpg",
    "timon-rhomy-higashiyama": "rhomy-opening.jpg",
    "tsurihack-tada-maki-challenge": "tsurihack-sunset.jpg",
    "ueno-kanna-winter-hakosuchi": "hakosuchi-biha.jpg",
    "vibration-spring-summer": "vibration-ono-lineup.jpg",
    "zamami-slj-tenya": "zamami-gt.jpg",
}


# Frames with a channel bug or title card on the left can trim it away before
# the cinematic crop. The value is the fraction removed from the left edge.
LEFT_CROP: dict[str, float] = {
    "daiwa-silver-creek-stream": 0.06,
    "murata-baitcasting": 0.22,
    "murata-spinning-casting": 0.22,
    "timon-rhomy-higashiyama": 0.06,
}

# Some stills need the cinematic crop nudged upward to clear a lower-third
# caption without cutting the subject. The value shifts the crop window.
TOP_OFFSET: dict[str, int] = {
    "area-trout-beginner-complete-guide": 45,
    "chuzenji-x9-resolution": -23,
    "genkai-otoshikomi": -33,
    "hiramoto-7-stream-items": 40,
    "rivanyoro-feet-bite": -34,
    "ueno-kanna-winter-hakosuchi": -55,
    "zamami-slj-tenya": -33,
}


def grade(image: Image.Image) -> Image.Image:
    """A restrained, shared grade: slightly muted, gently contrasty."""
    image = ImageEnhance.Color(image).enhance(0.92)
    image = ImageEnhance.Contrast(image).enhance(1.06)
    image = ImageEnhance.Brightness(image).enhance(0.98)

    width, height = image.size
    mask = Image.new("L", (width, height), 0)
    draw = ImageDraw.Draw(mask)
    draw.ellipse(
        [-width * 0.15, -height * 0.2, width * 1.15, height * 1.2],
        fill=255,
    )
    mask = mask.filter(ImageFilter.GaussianBlur(width * 0.08))
    vignette = Image.new("RGB", (width, height), (4, 12, 14))
    return Image.composite(image, vignette, mask)


def main() -> None:
    for slug, source_name in COVERS.items():
        source = ASSETS / source_name
        if not source.is_file():
            raise SystemExit(f"missing source frame: {source}")

        image = Image.open(source).convert("RGB")
        left = round(image.width * LEFT_CROP.get(slug, 0.0))
        if left:
            image = image.crop((left, 0, image.width, image.height))
        # 21:9, full width. The small top offset trims channel bugs and keeps
        # the subject from sitting against the very top edge.
        height = round(image.width * 9 / 21)
        top = max(
            0,
            (image.height - height) // 2 - 12 + TOP_OFFSET.get(slug, 0),
        )
        image = image.crop((0, top, image.width, top + height))
        image = grade(image)

        out = ASSETS / f"{slug}-cover.jpg"
        image.save(out, quality=88, optimize=True)
        print(f"{out.relative_to(ROOT)}  {image.width}x{image.height}")


if __name__ == "__main__":
    main()
