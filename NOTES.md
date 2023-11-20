# Merging SubsPlease/EN & LoliHouse/ZH-TW .ass subtitles

## Goal

Given two MKVs with subtitles - one English and the other Traditional Chinese -
do two things:

1. Extract both subtitle streams in ASS format and merge the English subtitles
   into the Traditional Chinese base. English subtitles should be displayed at
   the top of the screen ("toptitles", in ASS this is `Alignment=8`). The result
   should be a an external subtile file named `<episode>.zh-TW.ass`.
2. Extract all fonts from the Traditional Chinese MKV and merge them into the
   English MKV.

The result should be the English MKV now containing all the fonts required to
display the subtitles from both sources, and an external subtitle file with both
the Traditional Chinese (bottom) and English (top) subtitles with their original
styles.

## Rough working strategy

- ffmpeg extract both .ass files (see below)
- take `Default` style from .en.ass and add it to .zh-TW.ass, changing name to
  `DefaultEN` and size to 70
- cat all the `Default` dialogue lines from .en.ass into .zh-TW.ass, modifying
  them to `DefaultEN` as well
- Remove all lines saying `- JP`

This seems to accomplish our goal. Fonts are not copied, but they still seem to
display regardless.

## Examples

### Extract Traditional Chinese subtitles

NOTE: Simplified Chinese subtitles also exist at stream `0:2`

```bash
input='[LoliHouse] Rurouni Kenshin (2023) - 01 [WebRip 1080p HEVC-10bit AAC ASSx2].mkv'
output="${input%.*}.zh-TW.ass"
ffmpeg -i "${input}" -map 0:3 -c copy "${output}"
```

### Extract English subtitles

```bash
input='Rurouni Kenshin (2023) - S01E01 - 001 - Kenshin Himura Battosai [HDTV-1080p][8bit][x264][AAC 2.0][JA]-SubsPlease.mkv'
output="${input%.*}.en.ass"
ffmpeg -i "${input}" -map 0:2 -c copy "${output}"
```
