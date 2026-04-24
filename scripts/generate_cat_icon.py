"""Generate cat app icon using Pillow (no external SVG deps)."""
from PIL import Image, ImageDraw
import math
import os

ICONS_DIR = os.path.join(os.path.dirname(__file__), '..', 'src-tauri', 'icons')

def draw_cat(draw, size, cx, cy, r):
    """Draw a cute cat face at (cx, cy) with radius r."""
    accent = (0, 229, 255)
    accent_dim = (0, 180, 200)
    bg_dark = (14, 14, 24)
    white = (255, 255, 255)
    white_dim = (200, 200, 220)

    lw = max(1, size // 32)  # adaptive line width

    # Head circle
    head_r = int(r * 0.75)
    draw.ellipse([cx - head_r, cy - head_r, cx + head_r, cy + head_r], fill=accent, outline=accent_dim, width=lw)

    # Left ear
    ear_h = int(r * 0.6)
    ear_base_y = cy - head_r + int(r * 0.1)
    left_ear = [
        (cx - int(r * 0.52), ear_base_y),
        (cx - int(r * 0.25), ear_base_y - ear_h),
        (cx - int(r * 0.02), ear_base_y + int(r * 0.05)),
    ]
    draw.polygon(left_ear, fill=accent, outline=accent_dim, width=lw)

    # Right ear
    right_ear = [
        (cx + int(r * 0.52), ear_base_y),
        (cx + int(r * 0.25), ear_base_y - ear_h),
        (cx + int(r * 0.02), ear_base_y + int(r * 0.05)),
    ]
    draw.polygon(right_ear, fill=accent, outline=accent_dim, width=lw)

    # Inner ears
    left_inner = [
        (cx - int(r * 0.42), ear_base_y + int(r * 0.04)),
        (cx - int(r * 0.25), ear_base_y - int(ear_h * 0.55)),
        (cx - int(r * 0.12), ear_base_y + int(r * 0.08)),
    ]
    draw.polygon(left_inner, fill=bg_dark)
    right_inner = [
        (cx + int(r * 0.42), ear_base_y + int(r * 0.04)),
        (cx + int(r * 0.25), ear_base_y - int(ear_h * 0.55)),
        (cx + int(r * 0.12), ear_base_y + int(r * 0.08)),
    ]
    draw.polygon(right_inner, fill=bg_dark)

    # Eyes - big cute eyes
    eye_y = cy - int(r * 0.06)
    eye_spacing = int(r * 0.3)
    eye_r = max(2, int(r * 0.19))
    pupil_r = max(1, int(r * 0.12))

    # Eye whites
    draw.ellipse([cx - eye_spacing - eye_r, eye_y - eye_r,
                   cx - eye_spacing + eye_r, eye_y + eye_r], fill=white)
    draw.ellipse([cx + eye_spacing - eye_r, eye_y - eye_r,
                   cx + eye_spacing + eye_r, eye_y + eye_r], fill=white)

    # Pupils
    draw.ellipse([cx - eye_spacing - pupil_r + 1, eye_y - pupil_r,
                   cx - eye_spacing + pupil_r + 1, eye_y + pupil_r], fill=bg_dark)
    draw.ellipse([cx + eye_spacing - pupil_r + 1, eye_y - pupil_r,
                   cx + eye_spacing + pupil_r + 1, eye_y + pupil_r], fill=bg_dark)

    # Eye highlights
    hl_r = max(1, int(r * 0.07))
    hl_offset = max(1, int(r * 0.07))
    draw.ellipse([cx - eye_spacing + hl_offset - hl_r, eye_y - hl_offset - hl_r,
                   cx - eye_spacing + hl_offset + hl_r, eye_y - hl_offset + hl_r], fill=white)
    draw.ellipse([cx + eye_spacing + hl_offset - hl_r, eye_y - hl_offset - hl_r,
                   cx + eye_spacing + hl_offset + hl_r, eye_y - hl_offset + hl_r], fill=white)

    # Nose
    nose_y = cy + int(r * 0.2)
    nose_s = max(1, int(r * 0.09))
    draw.polygon([
        (cx, nose_y - nose_s),
        (cx - nose_s, nose_y + nose_s),
        (cx + nose_s, nose_y + nose_s),
    ], fill=(255, 150, 180))

    # Mouth
    mouth_y = nose_y + nose_s + 1
    mouth_w = max(1, int(r * 0.14))
    mw = max(1, lw)
    draw.arc([cx - mouth_w, mouth_y, cx, mouth_y + int(r * 0.14)],
             0, 180, fill=white_dim, width=mw)
    draw.arc([cx, mouth_y, cx + mouth_w, mouth_y + int(r * 0.14)],
             0, 180, fill=white_dim, width=mw)

    # Whiskers
    whisk_y = cy + int(r * 0.17)
    wl_len = int(r * 0.38)
    for dy in [-3, 0, 3]:
        wy = whisk_y + dy
        draw.line([cx - int(r * 0.32), wy, cx - int(r * 0.32) - wl_len, wy + dy * 2],
                  fill=white_dim, width=max(1, lw))
        draw.line([cx + int(r * 0.32), wy, cx + int(r * 0.32) + wl_len, wy + dy * 2],
                  fill=white_dim, width=max(1, lw))


def create_icon(size):
    """Create a cat icon at the given size."""
    img = Image.new('RGBA', (size, size), (14, 14, 24, 255))
    draw = ImageDraw.Draw(img)

    # Rounded background
    margin = max(1, size // 16)
    radius = max(2, size // 4)
    draw.rounded_rectangle([margin, margin, size - margin, size - margin],
                          radius=radius, fill=(14, 14, 24, 255),
                          outline=(0, 229, 255, 80), width=max(1, size // 32))

    # Draw cat
    cx = size // 2
    cy = size // 2 + size // 12  # slightly lower
    r = (size - 2 * margin) // 2 - size // 16
    draw_cat(draw, size, cx, cy, r)

    return img


os.makedirs(ICONS_DIR, exist_ok=True)

# Generate PNG files
for size in [32, 64, 128, 256, 512]:
    img = create_icon(size)
    if size == 32:
        img.save(os.path.join(ICONS_DIR, '32x32.png'))
    elif size == 64:
        img.save(os.path.join(ICONS_DIR, '64x64.png'))
    elif size == 128:
        img.save(os.path.join(ICONS_DIR, '128x128.png'))
    elif size == 256:
        img.save(os.path.join(ICONS_DIR, '128x128@2x.png'))
    elif size == 512:
        img.save(os.path.join(ICONS_DIR, 'icon.png'))

# Generate ICO (multi-size)
ico_sizes = [16, 32, 48, 64, 128, 256]
ico_images = [create_icon(s) for s in ico_sizes]
ico_images[0].save(
    os.path.join(ICONS_DIR, 'icon.ico'),
    format='ICO',
    sizes=[(s, s) for s in ico_sizes],
    append_images=ico_images[1:]
)

# Generate Store logos
for name, sz in [
    ('StoreLogo', 50),
    ('Square30x30Logo', 30),
    ('Square44x44Logo', 44),
    ('Square71x71Logo', 71),
    ('Square150x150Logo', 150),
    ('Square310x310Logo', 310),
]:
    img = create_icon(sz)
    img.save(os.path.join(ICONS_DIR, f'{name}.png'))

print("All cat icons generated!")
