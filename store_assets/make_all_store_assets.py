import os
import math
from PIL import Image, ImageDraw, ImageFont, ImageFilter

OUT_DIR = "store_assets"
os.makedirs(OUT_DIR, exist_ok=True)

master_logo_path = "widget/Assets/MasterLogo.png"
master = Image.open(master_logo_path).convert("RGBA")

font_bold_path = "C:/Windows/Fonts/segoeuib.ttf"
font_regular_path = "C:/Windows/Fonts/segoeui.ttf"

def get_font(size, bold=False):
    path = font_bold_path if bold else font_regular_path
    try:
        return ImageFont.truetype(path, size)
    except:
        return ImageFont.load_default()


# =========================================================================
# 1. STORE DISPLAY IMAGES (App Tile Icons)
# =========================================================================
# 300 x 300
tile_300 = master.resize((300, 300), Image.Resampling.LANCZOS)
p300 = os.path.join(OUT_DIR, "Store_App_Tile_300x300.png")
tile_300.save(p300, "PNG")
print(f"Created {p300} (300x300)")

# 150 x 150
tile_150 = master.resize((150, 150), Image.Resampling.LANCZOS)
p150 = os.path.join(OUT_DIR, "Store_App_Tile_150x150.png")
tile_150.save(p150, "PNG")
print(f"Created {p150} (150x150)")

# 71 x 71
tile_71 = master.resize((71, 71), Image.Resampling.LANCZOS)
p71 = os.path.join(OUT_DIR, "Store_App_Tile_71x71.png")
tile_71.save(p71, "PNG")
print(f"Created {p71} (71x71)")


# =========================================================================
# 2. STORE LOGOS (1:1 Box Art & 9:16 Poster Art)
# =========================================================================
# 1:1 Box Art (1080 x 1080)
box_w, box_h = 1080, 1080
box_img = Image.new("RGBA", (box_w, box_h), (15, 20, 28, 255))
glow_layer = Image.new("RGBA", (box_w, box_h), (0, 0, 0, 0))
glow_draw = ImageDraw.Draw(glow_layer)
glow_draw.ellipse([box_w * 0.2, box_h * 0.2, box_w * 0.8, box_h * 0.8], fill=(0, 210, 255, 30))
glow_draw.ellipse([box_w * 0.3, box_h * 0.3, box_w * 0.9, box_h * 0.9], fill=(255, 160, 40, 25))
glow_layer = glow_layer.filter(ImageFilter.GaussianBlur(120))
box_img = Image.alpha_composite(box_img, glow_layer)

icon_size = 740
master_box = master.resize((icon_size, icon_size), Image.Resampling.LANCZOS)
shadow_layer = Image.new("RGBA", (box_w, box_h), (0, 0, 0, 0))
shadow_draw = ImageDraw.Draw(shadow_layer)
shadow_pos = ((box_w - icon_size) // 2, (box_h - icon_size) // 2 + 18)
shadow_draw.rounded_rectangle(
    [shadow_pos[0] + 40, shadow_pos[1] + 40, shadow_pos[0] + icon_size - 40, shadow_pos[1] + icon_size - 40],
    radius=160,
    fill=(0, 0, 0, 140)
)
shadow_layer = shadow_layer.filter(ImageFilter.GaussianBlur(40))
box_img = Image.alpha_composite(box_img, shadow_layer)
icon_pos = ((box_w - icon_size) // 2, (box_h - icon_size) // 2)
box_img.paste(master_box, icon_pos, master_box)

p_box = os.path.join(OUT_DIR, "Store_Box_Art_1_1.png")
box_img.convert("RGB").save(p_box, "PNG", quality=100)
print(f"Created {p_box} (1080x1080)")

# 9:16 Poster Art (720 x 1080)
poster_w, poster_h = 720, 1080
poster_img = Image.new("RGBA", (poster_w, poster_h), (13, 17, 24, 255))
pglow = Image.new("RGBA", (poster_w, poster_h), (0, 0, 0, 0))
pglow_draw = ImageDraw.Draw(pglow)
pglow_draw.ellipse([60, 100, 560, 600], fill=(0, 220, 255, 38))
pglow_draw.ellipse([160, 300, 660, 800], fill=(255, 150, 30, 30))
pglow = pglow.filter(ImageFilter.GaussianBlur(130))
poster_img = Image.alpha_composite(poster_img, pglow)

p_icon_size = 460
master_poster = master.resize((p_icon_size, p_icon_size), Image.Resampling.LANCZOS)
p_icon_x = (poster_w - p_icon_size) // 2
p_icon_y = 170

pshadow = Image.new("RGBA", (poster_w, poster_h), (0, 0, 0, 0))
psdraw = ImageDraw.Draw(pshadow)
psdraw.rounded_rectangle(
    [p_icon_x + 30, p_icon_y + 40, p_icon_x + p_icon_size - 30, p_icon_y + p_icon_size + 10],
    radius=100,
    fill=(0, 0, 0, 160)
)
pshadow = pshadow.filter(ImageFilter.GaussianBlur(35))
poster_img = Image.alpha_composite(poster_img, pshadow)
poster_img.paste(master_poster, (p_icon_x, p_icon_y), master_poster)

p_draw = ImageDraw.Draw(poster_img)
font_title = get_font(68, bold=True)
font_sub = get_font(28, bold=False)
font_pill = get_font(22, bold=True)

title_text = "Net Flow"
title_bbox = p_draw.textbbox((0, 0), title_text, font=font_title)
title_w = title_bbox[2] - title_bbox[0]
title_y = p_icon_y + p_icon_size + 45
p_draw.text(((poster_w - title_w) // 2, title_y), title_text, font=font_title, fill=(255, 255, 255, 255))

sub_text = "Live Bandwidth Monitor"
sub_bbox = p_draw.textbbox((0, 0), sub_text, font=font_sub)
sub_w = sub_bbox[2] - sub_bbox[0]
sub_y = title_y + 80
p_draw.text(((poster_w - sub_w) // 2, sub_y), sub_text, font=font_sub, fill=(160, 178, 204, 255))

pill_text = "WINDOWS 11 WIDGET"
pill_bbox = p_draw.textbbox((0, 0), pill_text, font=font_pill)
pw = pill_bbox[2] - pill_bbox[0] + 36
ph = pill_bbox[3] - pill_bbox[1] + 18
pill_x = (poster_w - pw) // 2
pill_y = sub_y + 65

p_draw.rounded_rectangle(
    [pill_x, pill_y, pill_x + pw, pill_y + ph],
    radius=ph // 2,
    fill=(25, 34, 48, 200),
    outline=(0, 210, 255, 100),
    width=1
)
p_draw.text(
    (pill_x + 18, pill_y + 8),
    pill_text,
    font=font_pill,
    fill=(0, 210, 255, 230)
)

p_poster = os.path.join(OUT_DIR, "Store_Poster_9_16.png")
poster_img.convert("RGB").save(p_poster, "PNG", quality=100)
print(f"Created {p_poster} (720x1080)")


# =========================================================================
# 3. STORE SCREENSHOT (1920 x 1080 Desktop)
# =========================================================================
W, H = 1920, 1080
s_img = Image.new("RGBA", (W, H), (15, 18, 28, 255))
s_draw = ImageDraw.Draw(s_img)

for y in range(H):
    ratio = y / H
    r = int(12 + ratio * 10)
    g = int(14 + ratio * 12)
    b = int(24 + ratio * 20)
    s_draw.line([(0, y), (W, y)], fill=(r, g, b, 255))

bloom = Image.new("RGBA", (W, H), (0, 0, 0, 0))
bdraw = ImageDraw.Draw(bloom)
bdraw.ellipse([800, 200, 1600, 900], fill=(0, 120, 212, 35))
bdraw.ellipse([1100, 350, 1800, 1000], fill=(136, 51, 238, 28))
bdraw.ellipse([300, 100, 900, 700], fill=(56, 217, 240, 20))
bloom = bloom.filter(ImageFilter.GaussianBlur(120))
s_img = Image.alpha_composite(s_img, bloom)
s_draw = ImageDraw.Draw(s_img)

tb_h = 48
tb_y = H - tb_h
s_draw.rectangle([0, tb_y, W, H], fill=(24, 25, 32, 230))
s_draw.line([(0, tb_y), (W, tb_y)], fill=(255, 255, 255, 20), width=1)

center_x = W // 2 - 120
for row in range(2):
    for col in range(2):
        s_draw.rectangle([center_x + col*11, tb_y + 14 + row*11, center_x + col*11 + 8, tb_y + 14 + row*11 + 8], fill=(0, 164, 239))
s_draw.rounded_rectangle([center_x + 35, tb_y + 8, center_x + 180, tb_y + 40], radius=16, fill=(40, 42, 52), outline=(255, 255, 255, 25))
s_draw.text((center_x + 55, tb_y + 13), "Search", fill=(180, 185, 200), font=get_font(14))

s_draw.rounded_rectangle([16, tb_y + 6, 120, tb_y + 42], radius=6, fill=(48, 52, 68), outline=(56, 217, 240, 80))
s_draw.text((28, tb_y + 13), "NET 42.8 MB/s", fill=(56, 217, 240), font=get_font(12, bold=True))

s_draw.text((W - 130, tb_y + 7), "6:45 PM", fill=(240, 240, 240), font=get_font(13))
s_draw.text((W - 145, tb_y + 25), "9/16/2026", fill=(170, 175, 190), font=get_font(12))

wb_w = 640
wb_h = H - tb_h - 16
wb_box = [16, 16, wb_w + 16, wb_h + 16]

panel_shadow = Image.new("RGBA", (W, H), (0, 0, 0, 0))
sdraw = ImageDraw.Draw(panel_shadow)
sdraw.rounded_rectangle([wb_box[0]-10, wb_box[1]-6, wb_box[2]+15, wb_box[3]+10], radius=16, fill=(0, 0, 0, 140))
panel_shadow = panel_shadow.filter(ImageFilter.GaussianBlur(18))
s_img = Image.alpha_composite(s_img, panel_shadow)
s_draw = ImageDraw.Draw(s_img)

s_draw.rounded_rectangle(wb_box, radius=14, fill=(28, 30, 38, 235), outline=(255, 255, 255, 30), width=1)
s_draw.text((36, 32), "Widgets", fill=(255, 255, 255), font=get_font(22, bold=True))

# Net Flow Large Widget Card
cw_x = 36
cw_y = 76
cw_w = wb_w - 40
cw_h = 490

w_shadow = Image.new("RGBA", (W, H), (0, 0, 0, 0))
w_sdraw = ImageDraw.Draw(w_shadow)
w_sdraw.rounded_rectangle([cw_x-4, cw_y+2, cw_x+cw_w+4, cw_y+cw_h+8], radius=12, fill=(0, 0, 0, 80))
w_shadow = w_shadow.filter(ImageFilter.GaussianBlur(10))
s_img = Image.alpha_composite(s_img, w_shadow)
s_draw = ImageDraw.Draw(s_img)

s_draw.rounded_rectangle([cw_x, cw_y, cw_x+cw_w, cw_y+cw_h], radius=10, fill=(35, 38, 48, 245), outline=(255, 255, 255, 25), width=1)

# Card Header
s_draw.text((cw_x + 18, cw_y + 16), "Wi-Fi (Bharti-5G)", fill=(240, 245, 255), font=get_font(15, bold=True))
s_draw.text((cw_x + cw_w - 95, cw_y + 16), "213 conns", fill=(140, 150, 175), font=get_font(13))

# Speeds
s_draw.text((cw_x + 18, cw_y + 44), "Download", fill=(56, 217, 240), font=get_font(12, bold=True))
s_draw.text((cw_x + 18, cw_y + 60), "42.8 MB/s", fill=(255, 255, 255), font=get_font(26, bold=True))
s_draw.text((cw_x + 18, cw_y + 94), "Peak: 89.4 MB/s", fill=(130, 140, 160), font=get_font(11))

s_draw.text((cw_x + cw_w // 2, cw_y + 44), "Upload", fill=(255, 170, 50), font=get_font(12, bold=True))
s_draw.text((cw_x + cw_w // 2, cw_y + 60), "14.2 MB/s", fill=(255, 255, 255), font=get_font(26, bold=True))
s_draw.text((cw_x + cw_w // 2, cw_y + 94), "Peak: 31.0 MB/s", fill=(130, 140, 160), font=get_font(11))

# Live Dual-Stream Chart
chart_x = cw_x + 18
chart_y = cw_y + 120
chart_w = cw_w - 36
chart_h = 100
s_draw.rectangle([chart_x, chart_y, chart_x + chart_w, chart_y + chart_h], fill=(22, 24, 32, 200), outline=(255, 255, 255, 15))
mid_y = chart_y + chart_h // 2
s_draw.line([(chart_x, mid_y), (chart_x + chart_w, mid_y)], fill=(70, 75, 90), width=1)

# Wave plot
rx_pts = []
tx_pts = []
for i in range(chart_w):
    rx_val = math.sin(i * 0.05) * 20 + math.cos(i * 0.02) * 12 + 25
    tx_val = math.cos(i * 0.04) * 14 + math.sin(i * 0.03) * 8 + 18
    rx_pts.append((chart_x + i, mid_y - rx_val))
    tx_pts.append((chart_x + i, mid_y + tx_val))

for i in range(len(rx_pts) - 1):
    s_draw.line([rx_pts[i], rx_pts[i+1]], fill=(56, 217, 240), width=2)
    s_draw.line([tx_pts[i], tx_pts[i+1]], fill=(255, 170, 50), width=2)

# Active Apps list
apps_y = chart_y + chart_h + 16
s_draw.text((chart_x, apps_y), "Active applications", fill=(160, 170, 195), font=get_font(12, bold=True))
apps_data = [
    ("Chrome", "24.1 MB/s", "1.2 MB/s", (0, 180, 255)),
    ("Steam", "17.4 MB/s", "0.2 MB/s", (80, 120, 240)),
    ("Antigravity IDE", "1.1 MB/s", "12.4 MB/s", (180, 100, 255)),
    ("Discord", "0.2 MB/s", "0.4 MB/s", (114, 137, 218)),
]
for idx, (app_name, down, up, color) in enumerate(apps_data):
    row_y = apps_y + 24 + idx * 36
    s_draw.ellipse([chart_x, row_y + 4, chart_x + 16, row_y + 20], fill=color)
    s_draw.text((chart_x + 26, row_y + 2), app_name, fill=(230, 235, 245), font=get_font(13))
    s_draw.text((cw_x + cw_w - 180, row_y + 2), f"↓ {down}  ↑ {up}", fill=(150, 160, 180), font=get_font(12))

# Weather / Second widget
w2_y = cw_y + cw_h + 20
s_draw.rounded_rectangle([cw_x, w2_y, cw_x+cw_w, w2_y+200], radius=10, fill=(35, 38, 48, 200), outline=(255, 255, 255, 20), width=1)
s_draw.text((cw_x + 18, w2_y + 16), "Weather", fill=(240, 245, 255), font=get_font(15, bold=True))
s_draw.text((cw_x + 18, w2_y + 50), "72° Mostly Sunny", fill=(255, 255, 255), font=get_font(22, bold=True))
s_draw.text((cw_x + 18, w2_y + 85), "San Francisco, CA • H: 75° L: 58°", fill=(140, 150, 175), font=get_font(12))

p_screen = os.path.join(OUT_DIR, "Store_Screenshot_1.png")
s_img.convert("RGB").save(p_screen, "PNG", quality=100)
print(f"Created {p_screen} (1920x1080)")
