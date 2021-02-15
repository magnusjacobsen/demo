from PIL import Image, ImageFont, ImageDraw
import numpy as np
import argparse

FONTS = {
    'minecraft': 'fonts/Minecraft.ttf',
    'poppkorn' : 'fonts/PoppkornRegular-MzKY.ttf',
    'atari'    : 'fonts/ATARCC.ttf'
}

def char_to_pixels(text, path='', fontsize=14):
    """
    Based on https://stackoverflow.com/a/27753869/190597 (jsheperd)
    """
    font = ImageFont.truetype(path, fontsize)
    w,h = font.getsize(text)
    h *= 2
    image = Image.new('L', (w, h), 1)
    draw = ImageDraw.Draw(image)
    draw.text((0,0), text, font=font)
    arr = np.asarray(image)
    arr = np.where(arr, 0, 1)
    arr = arr[(arr != 0).any(axis=1)]
    return arr

def display(arr):
    result = np.where(arr, '#', ' ')
    print('\n'.join([''.join(row) for row in result]))

def main(args):
    alphabet_str = 'a_' #'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!.?_'
    font_path = FONTS[args.font]
    alphabet_to_id = {c: i for (i,c) in enumerate(alphabet_str)}
    id_to_vec = [None] * len(alphabet_to_id)
    for c in alphabet_str:
        print(f'character: {c}:')
        arr = char_to_pixels(c, font_path, fontsize=26)
        print!("vec![", )
        for row in arr:
            for col in arr:

        display(arr)
        print()

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--font", type=str, default="minecraft")
    parser.add_argument("--print-alphabet", action="store_true")
    parser.add_argument("--text", type=str, default="")
    args = parser.parse_args()
    main(args)