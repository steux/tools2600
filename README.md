# sprites2600

_sprites2600_ is a tool (written in Rust) designed to generate C code for sprites to be displayed by cc2600 multisprite engine. 
![image](https://github.com/steux/tools2600/assets/22678486/0b67b974-b2db-44db-970d-6210d250bd6c)

_sprites2600_ processes a YAML file that describes all the sprites to be generated (associated to .png spritesheets). For instance, for the picture above (some shmup sprites - drawn with GIMP), the YAML file is :

```yaml
sprite_sheets:
  - image: shmup.png
    sprites:
      - name: spaceship 
        top: 0
        left: 0
        height: 16
        color_copy: spaceship_exhaust
      - name: spaceship_exhaust
        top: 0
        left: 0
        height: 24
      - name: fire
        top: 0
        left: 64
        height: 9
        color_copy: spaceship_exhaust
        color_offset: 15
      - name: bullet
        top: 9
        left: 64
        height: 9
      - name: explosion1
        top: 0
        left: 80
        height: 9
        color_copy: explosion3
      - name: explosion2
        top: 0
        left: 88
        height: 11 
        color_copy: explosion3
      - name: explosion3
        top: 0
        left: 96 
        height: 12 
      - name: explosion4
        top: 0
        left: 104
        height: 12 
        color_copy: explosion3
      - name: explosion5
        top: 0
        left: 112
        height: 12 
        color_copy: explosion3
      - name: enemy1 
        top: 0
        left: 72
        height: 16
      - name: bigboss
        top: 24
        left: 0
        height: 32
        pixel_width: 2
      - name: letter_g
        top: 0
        left: 8
        height: 16
        color_copy: spaceship_exhaust
      - name: letter_a
        top: 0
        left: 16 
        height: 16
        color_copy: spaceship_exhaust
      - name: letter_m
        top: 0
        left: 24
        height: 16
        color_copy: spaceship_exhaust
      - name: letter_e
        top: 0
        left: 32
        height: 16
        color_copy: spaceship_exhaust
      - name: letter_o
        top: 0
        left: 40
        height: 16
        color_copy: spaceship_exhaust
      - name: letter_v
        top: 0
        left: 48
        height: 16
        color_copy: spaceship_exhaust
      - name: letter_r
        top: 0
        left: 56
        height: 16
        color_copy: spaceship_exhaust
```
 
The generated C code for the command __sprites2600 shmup.yaml__ (_examples/example_shmup_gfx.c_ in cc2600 repositorty) will be :

```C
MS_KERNEL_BANK const char spaceship_gfx[20] = {0, 0, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3c, 0x18, 0x18, 0x3c, 0xbd, 0xff, 0xdb, 0xdb, 0xdb, 0x66, 0x66, 0, 0};
MS_KERNEL_BANK const char spaceship_exhaust_gfx[28] = {0, 0, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3c, 0x18, 0x18, 0x3c, 0xbd, 0xff, 0xdb, 0xdb, 0xdb, 0x66, 0x66, 0xf6, 0xff, 0xff, 0x6f, 0x76, 0x6e, 0x66, 0x22, 0, 0};
#ifdef PAL
MS_KERNEL_BANK const char spaceship_exhaust_colors[26] = {0, 0, 0x04, 0x04, 0xd4, 0xd0, 0xd0, 0x06, 0x08, 0x08, 0x0a, 0x0a, 0x0a, 0x0c, 0x0c, 0x0e, 0x0e, 0x40, 0x42, 0x44, 0x46, 0x48, 0x2a, 0x2a, 0x2c, 0x1c};
#else
MS_KERNEL_BANK const char spaceship_exhaust_colors[26] = {0, 0, 0x04, 0x04, 0x84, 0x80, 0x80, 0x06, 0x08, 0x08, 0x0a, 0x0a, 0x0a, 0x0c, 0x0c, 0x0e, 0x0e, 0x30, 0x32, 0x34, 0x36, 0x38, 0x1a, 0x1a, 0x1c, 0x1c};
#endif
MS_KERNEL_BANK const char fire_gfx[13] = {0, 0, 0x42, 0xe7, 0xe7, 0xe7, 0x42, 0x42, 0x42, 0x42, 0x42, 0, 0};
MS_KERNEL_BANK const char bullet_gfx[13] = {0, 0, 0x3c, 0x7e, 0x7e, 0x7e, 0x7e, 0x7e, 0x3c, 0x3c, 0x18, 0, 0};
#ifdef PAL
MS_KERNEL_BANK const char bullet_colors[11] = {0, 0, 0x2c, 0x2c, 0x2a, 0x2a, 0x48, 0x46, 0x44, 0x42, 0x30};
#else
MS_KERNEL_BANK const char bullet_colors[11] = {0, 0, 0x1c, 0x1c, 0x1a, 0x1a, 0x38, 0x36, 0x34, 0x32, 0x30};
#endif
MS_KERNEL_BANK const char explosion1_gfx[13] = {0, 0, 0x00, 0x00, 0x00, 0x00, 0x18, 0x0c, 0x24, 0x34, 0x08, 0, 0};
MS_KERNEL_BANK const char explosion2_gfx[15] = {0, 0, 0x00, 0x00, 0x08, 0x3c, 0x12, 0x26, 0x64, 0x2c, 0x2e, 0x30, 0x08, 0, 0};
MS_KERNEL_BANK const char explosion3_gfx[16] = {0, 0, 0x06, 0x68, 0x77, 0xcf, 0xa3, 0x93, 0xd3, 0x41, 0xf5, 0x48, 0x6a, 0x1e, 0, 0};
#ifdef PAL
MS_KERNEL_BANK const char explosion3_colors[14] = {0, 0, 0x2e, 0x2c, 0x2a, 0x48, 0x64, 0x60, 0x60, 0x64, 0x48, 0x2a, 0x2c, 0x1e};
#else
MS_KERNEL_BANK const char explosion3_colors[14] = {0, 0, 0x1e, 0x1c, 0x1a, 0x38, 0x44, 0x40, 0x40, 0x44, 0x38, 0x1a, 0x1c, 0x1e};
#endif
MS_KERNEL_BANK const char explosion4_gfx[16] = {0, 0, 0x23, 0xe5, 0xaa, 0x84, 0x47, 0x40, 0x00, 0x86, 0xa3, 0xf7, 0x76, 0x52, 0, 0};
MS_KERNEL_BANK const char explosion5_gfx[16] = {0, 0, 0x25, 0xc2, 0x41, 0x81, 0x00, 0x00, 0x00, 0x00, 0xc1, 0x82, 0x64, 0x43, 0, 0};
MS_KERNEL_BANK const char enemy1_gfx[20] = {0, 0, 0x22, 0x44, 0x6e, 0xf7, 0x66, 0x5a, 0xdb, 0xdb, 0x5a, 0x7e, 0x3c, 0x18, 0x58, 0x58, 0x58, 0x18, 0, 0};
#ifdef PAL
MS_KERNEL_BANK const char enemy1_colors[18] = {0, 0, 0x2e, 0x2a, 0x48, 0x64, 0x60, 0x28, 0x58, 0x28, 0x28, 0x28, 0x26, 0xd0, 0x26, 0x58, 0x64, 0x40};
#else
MS_KERNEL_BANK const char enemy1_colors[18] = {0, 0, 0x1e, 0x1a, 0x38, 0x44, 0x40, 0xe8, 0xc8, 0xe8, 0xe8, 0xe8, 0xe6, 0x80, 0xe6, 0xc8, 0x44, 0x40};
#endif
MS_KERNEL_BANK const char bigboss_gfx[36] = {0, 0, 0x04, 0x0c, 0x0c, 0x0e, 0x1e, 0x5e, 0x5e, 0x5e, 0x5f, 0x6f, 0xeb, 0xeb, 0xeb, 0xe7, 0xe7, 0xf7, 0xf7, 0x77, 0x73, 0x73, 0xf7, 0xff, 0xff, 0xff, 0x7f, 0x7f, 0x3f, 0x3f, 0x1e, 0x0c, 0x1e, 0x0c, 0, 0};
#ifdef PAL
MS_KERNEL_BANK const char bigboss_colors[34] = {0, 0, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xac, 0xa2, 0xac, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa6, 0xa2, 0xa6, 0xa6, 0xa2, 0xa2, 0xa2, 0xa4, 0xa4, 0xa2, 0xa4, 0xa2, 0xa2, 0x40, 0x46, 0x1c};
#else
MS_KERNEL_BANK const char bigboss_colors[34] = {0, 0, 0x62, 0x62, 0x62, 0x62, 0x62, 0x6c, 0x62, 0x6c, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x66, 0x62, 0x66, 0x66, 0x62, 0x62, 0x62, 0x64, 0x64, 0x62, 0x64, 0x62, 0x62, 0x30, 0x36, 0x1c};
#endif
MS_KERNEL_BANK const char letter_g_gfx[20] = {0, 0, 0x3c, 0x3e, 0x76, 0x60, 0xe0, 0xc0, 0xc0, 0xcf, 0xc3, 0xc3, 0xc3, 0xc7, 0x66, 0x7e, 0x3e, 0x3c, 0, 0};
MS_KERNEL_BANK const char letter_a_gfx[20] = {0, 0, 0x18, 0x18, 0x1c, 0x3e, 0x36, 0x36, 0x36, 0x62, 0x63, 0x63, 0xc3, 0xff, 0xc3, 0xc3, 0xc3, 0xc3, 0, 0};
MS_KERNEL_BANK const char letter_m_gfx[20] = {0, 0, 0xfc, 0xde, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb, 0, 0};
MS_KERNEL_BANK const char letter_e_gfx[20] = {0, 0, 0x3f, 0x3f, 0x70, 0x60, 0xe0, 0xc0, 0xc0, 0xfc, 0xc0, 0xc0, 0xc0, 0xe0, 0x7f, 0x7f, 0x3f, 0x3f, 0, 0};
MS_KERNEL_BANK const char letter_o_gfx[20] = {0, 0, 0x3c, 0x3e, 0x76, 0x63, 0xe3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc7, 0x66, 0x7e, 0x3c, 0x3c, 0, 0};
MS_KERNEL_BANK const char letter_v_gfx[20] = {0, 0, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc7, 0xe6, 0x66, 0x6c, 0x6c, 0x3c, 0x38, 0x18, 0x18, 0, 0};
MS_KERNEL_BANK const char letter_r_gfx[20] = {0, 0, 0xfe, 0xc7, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xff, 0xfc, 0xcc, 0xc6, 0xc6, 0xc3, 0xc3, 0, 0};
#define MS_NB_SPRITES_DEF 18
MS_KERNEL_BANK const char *ms_grptr[MS_NB_SPRITES_DEF] = {spaceship_gfx, spaceship_exhaust_gfx, fire_gfx, bullet_gfx, explosion1_gfx, explosion2_gfx, explosion3_gfx, explosion4_gfx, explosion5_gfx, enemy1_gfx, bigboss_gfx, letter_g_gfx, letter_a_gfx, letter_m_gfx, letter_e_gfx, letter_o_gfx, letter_v_gfx, letter_r_gfx};
MS_KERNEL_BANK const char *ms_coluptr[MS_NB_SPRITES_DEF] = {spaceship_exhaust_colors, spaceship_exhaust_colors, spaceship_exhaust_colors + 15, bullet_colors, explosion3_colors, explosion3_colors, explosion3_colors, explosion3_colors, explosion3_colors, enemy1_colors, bigboss_colors, spaceship_exhaust_colors, spaceship_exhaust_colors, spaceship_exhaust_colors, spaceship_exhaust_colors, spaceship_exhaust_colors, spaceship_exhaust_colors, spaceship_exhaust_colors};
MS_KERNEL_BANK const char ms_height[MS_NB_SPRITES_DEF] = {19, 27, 12, 12, 12, 14, 15, 15, 15, 19, 35, 19, 19, 19, 19, 19, 19, 19};
#define SPRITE_SPACESHIP 0
#define SPRITE_SPACESHIP_EXHAUST 1
#define SPRITE_FIRE 2
#define SPRITE_BULLET 3
#define SPRITE_EXPLOSION1 4
#define SPRITE_EXPLOSION2 5
#define SPRITE_EXPLOSION3 6
#define SPRITE_EXPLOSION4 7
#define SPRITE_EXPLOSION5 8
#define SPRITE_ENEMY1 9
#define SPRITE_BIGBOSS 10
#define SPRITE_LETTER_G 11
#define SPRITE_LETTER_A 12
#define SPRITE_LETTER_M 13
#define SPRITE_LETTER_E 14
#define SPRITE_LETTER_O 15
#define SPRITE_LETTER_V 16
#define SPRITE_LETTER_R 17
```
which is a code ready to be used with the cc2600 multisprite code.

Node that drawing the picture with GIMP requires a few steps :

- The NTSC Atari 2600 Palette should be used in GIMP. Only these colors will be processed by _sprites2600_ (logically). The .gpl GIMP palette file is in the current repository and is shown in the GIMP picture above.

- Only one color per sprite line. This is an Atari 2600...

- You can reuse others sprites color, using the color_copy option

Note that _sprites2600_ generates the NTSC and PAL colors (activated by `#define PAL` or `-DPAL` on the cc2600 compilation command line).

Note also that GIMP is perfectly suited to draw Atari 2600 sprites, since it can be configured to use "fat pixels".

For this, just change the Image/Print size... to set a non square DPI, like in the picture below :
![image](https://github.com/steux/tools2600/assets/22678486/0eb72493-5108-4b1f-820d-33eecb027148)

And click in View/Dot for Dot to disable the square display. You will then be able to edit pictures with flat pixels.
![image](https://github.com/steux/tools2600/assets/22678486/d7f7db6b-dd5e-4cc3-8f4d-5d20b647ced8)

