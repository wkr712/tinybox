"""Generate beautiful cat app icon — cute, polished, transparent background."""
from PIL import Image, ImageDraw, ImageFilter
import os
import math

ICONS_DIR = os.path.join(os.path.dirname(__file__), '..', 'src-tauri', 'icons')


def draw_rounded_triangle(draw, points, radius, **kwargs):
    """Draw a triangle with rounded corners using multiple overlapping shapes."""
    fill = kwargs.get('fill', (0, 0, 0, 0))
    # Draw main polygon
    draw.polygon(points, fill=fill)
    # Draw circles at each vertex for rounded effect
    r = radius
    for px, py in points:
        draw.ellipse([px - r, py - r, px + r, py + r], fill=fill)


def create_icon(size):
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    cx, cy = size // 2, size // 2
    s = size * 0.42

    # Color palette
    body_color = (210, 215, 230, 255)       # soft silver
    body_shadow = (185, 190, 210, 255)       # slightly darker
    highlight = (230, 232, 242, 255)         # light highlight
    ear_inner = (195, 175, 195, 140)         # pinkish inner ear
    eye_bg = (245, 245, 250, 255)            # eye white
    pupil = (40, 45, 65, 255)                # dark pupil
    pupil_shine = (255, 255, 255, 240)       # eye shine
    nose = (200, 155, 165, 220)              # soft pink nose
    mouth = (180, 180, 200, 100)             # subtle mouth
    whisker = (190, 195, 215, 90)            # subtle whisker

    head_cy = cy + int(s * 0.12)
    head_r = int(s * 0.52)

    # ─── Ears ───
    ear_round = max(2, int(s * 0.06))
    ear_base_y = head_cy - int(s * 0.18)

    # Left ear — taller, more elegant
    draw_rounded_triangle(draw, [
        (cx - int(s * 0.52), ear_base_y),
        (cx - int(s * 0.10), head_cy - int(s * 0.72)),
        (cx + int(s * 0.06), ear_base_y),
    ], ear_round, fill=body_color)

    # Right ear
    draw_rounded_triangle(draw, [
        (cx + int(s * 0.52), ear_base_y),
        (cx + int(s * 0.10), head_cy - int(s * 0.72)),
        (cx - int(s * 0.06), ear_base_y),
    ], ear_round, fill=body_color)

    # Inner ears
    if size >= 32:
        inner_inset = int(s * 0.09)
        inner_base = ear_base_y - inner_inset // 2
        draw_rounded_triangle(draw, [
            (cx - int(s * 0.42), inner_base),
            (cx - int(s * 0.10), head_cy - int(s * 0.60)),
            (cx + int(s * 0.02), inner_base),
        ], max(1, ear_round - 1), fill=ear_inner)
        draw_rounded_triangle(draw, [
            (cx + int(s * 0.42), inner_base),
            (cx + int(s * 0.10), head_cy - int(s * 0.60)),
            (cx - int(s * 0.02), inner_base),
        ], max(1, ear_round - 1), fill=ear_inner)

    # ─── Head ───
    draw.ellipse(
        [cx - head_r, head_cy - head_r, cx + head_r, head_cy + head_r],
        fill=body_color,
    )

    # Cheek shadows (two subtle darker ellipses on sides)
    if size >= 48:
        cheek_r = int(head_r * 0.45)
        cheek_y = head_cy + int(s * 0.08)
        # Left cheek
        draw.ellipse([
            cx - int(s * 0.28) - cheek_r, cheek_y - int(cheek_r * 0.5),
            cx - int(s * 0.28) + cheek_r, cheek_y + int(cheek_r * 0.5),
        ], fill=body_shadow)
        # Right cheek
        draw.ellipse([
            cx + int(s * 0.28) - cheek_r, cheek_y - int(cheek_r * 0.5),
            cx + int(s * 0.28) + cheek_r, cheek_y + int(cheek_r * 0.5),
        ], fill=body_shadow)

    # Forehead highlight
    if size >= 48:
        hl_r = int(head_r * 0.55)
        hl_cy = head_cy - int(s * 0.12)
        draw.ellipse([
            cx - hl_r, hl_cy - int(hl_r * 0.6),
            cx + hl_r, hl_cy + int(hl_r * 0.6),
        ], fill=highlight)

    # ─── Eyes ───
    eye_spacing = int(s * 0.24)
    eye_y = head_cy - int(s * 0.04)
    eye_rx = max(2, int(s * 0.11))  # horizontal radius
    eye_ry = max(2, int(s * 0.13))  # vertical radius (slightly taller)

    # Eye background (white)
    draw.ellipse([
        cx - eye_spacing - eye_rx, eye_y - eye_ry,
        cx - eye_spacing + eye_rx, eye_y + eye_ry,
    ], fill=eye_bg)
    draw.ellipse([
        cx + eye_spacing - eye_rx, eye_y - eye_ry,
        cx + eye_spacing + eye_rx, eye_y + eye_ry,
    ], fill=eye_bg)

    # Pupils (large, round, cute)
    pr = max(1, int(s * 0.08))
    draw.ellipse([
        cx - eye_spacing - pr, eye_y - pr,
        cx - eye_spacing + pr, eye_y + pr,
    ], fill=pupil)
    draw.ellipse([
        cx + eye_spacing - pr, eye_y - pr,
        cx + eye_spacing + pr, eye_y + pr,
    ], fill=pupil)

    # Eye shine (two highlights per eye for depth)
    if size >= 32:
        # Main shine (top-right)
        sr = max(1, int(s * 0.04))
        draw.ellipse([
            cx - eye_spacing + int(s * 0.03) - sr, eye_y - int(s * 0.04) - sr,
            cx - eye_spacing + int(s * 0.03) + sr, eye_y - int(s * 0.04) + sr,
        ], fill=pupil_shine)
        draw.ellipse([
            cx + eye_spacing + int(s * 0.03) - sr, eye_y - int(s * 0.04) - sr,
            cx + eye_spacing + int(s * 0.03) + sr, eye_y - int(s * 0.04) + sr,
        ], fill=pupil_shine)

    if size >= 64:
        # Secondary small shine (bottom-left)
        sr2 = max(1, int(s * 0.02))
        draw.ellipse([
            cx - eye_spacing - int(s * 0.02) - sr2, eye_y + int(s * 0.02) - sr2,
            cx - eye_spacing - int(s * 0.02) + sr2, eye_y + int(s * 0.02) + sr2,
        ], fill=(255, 255, 255, 180))
        draw.ellipse([
            cx + eye_spacing - int(s * 0.02) - sr2, eye_y + int(s * 0.02) - sr2,
            cx + eye_spacing - int(s * 0.02) + sr2, eye_y + int(s * 0.02) + sr2,
        ], fill=(255, 255, 255, 180))

    # ─── Nose ───
    if size >= 24:
        nose_y = head_cy + int(s * 0.1)
        nose_w = max(1, int(s * 0.05))
        nose_h = max(1, int(s * 0.035))
        # Rounded nose shape
        draw.rounded_rectangle(
            [cx - nose_w, nose_y - nose_h, cx + nose_w, nose_y + nose_h],
            radius=max(1, nose_h),
            fill=nose,
        )

    # ─── Mouth ───
    if size >= 48:
        mouth_y = nose_y + nose_h + max(1, int(s * 0.01))
        mouth_w = max(2, int(s * 0.07))
        mouth_drop = max(1, int(s * 0.035))
        lw = max(1, int(s * 0.012))
        # Left curve
        draw.arc(
            [cx - mouth_w, mouth_y, cx, mouth_y + mouth_drop * 2],
            0, 180, fill=mouth, width=lw,
        )
        # Right curve
        draw.arc(
            [cx, mouth_y, cx + mouth_w, mouth_y + mouth_drop * 2],
            0, 180, fill=mouth, width=lw,
        )

    # ─── Whiskers ───
    if size >= 64:
        wl = max(3, int(s * 0.25))
        wh_y = head_cy + int(s * 0.08)
        ww = max(1, int(s * 0.01))
        angles_left = [-8, 2, 12]
        angles_right = [8, -2, -12]
        for angle in angles_left:
            rad = math.radians(angle)
            x2 = cx - int(s * 0.18) - int(wl * math.cos(rad))
            y2 = wh_y + int(wl * math.sin(rad))
            draw.line([(cx - int(s * 0.18), wh_y), (x2, y2)], fill=whisker, width=ww)
        for angle in angles_right:
            rad = math.radians(angle)
            x2 = cx + int(s * 0.18) + int(wl * math.cos(rad))
            y2 = wh_y + int(wl * math.sin(rad))
            draw.line([(cx + int(s * 0.18), wh_y), (x2, y2)], fill=whisker, width=ww)

    return img


os.makedirs(ICONS_DIR, exist_ok=True)

# Generate PNG files
for px, name in [
    (32, '32x32.png'),
    (64, '64x64.png'),
    (128, '128x128.png'),
    (256, '128x128@2x.png'),
    (512, 'icon.png'),
]:
    create_icon(px).save(os.path.join(ICONS_DIR, name))

# ICO
ico_sizes = [16, 32, 48, 64, 128, 256]
imgs = [create_icon(s) for s in ico_sizes]
imgs[0].save(
    os.path.join(ICONS_DIR, 'icon.ico'),
    format='ICO',
    sizes=[(s, s) for s in ico_sizes],
    append_images=imgs[1:],
)

# Store logos
for name, sz in [
    ('StoreLogo', 50),
    ('Square30x30Logo', 30),
    ('Square44x44Logo', 44),
    ('Square71x71Logo', 71),
    ('Square150x150Logo', 150),
    ('Square310x310Logo', 310),
]:
    create_icon(sz).save(os.path.join(ICONS_DIR, f'{name}.png'))

print("Icons generated!")
