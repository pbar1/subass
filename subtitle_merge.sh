#!/usr/bin/env bash

set -euo pipefail -x

for input in *.mkv; do
  # map to sonarr-managed files
  n="$(echo "$input" | awk '{print $6}')"
  target="$(ls '/data/media/tv/Rurouni Kenshin (2023)/Season 01/'*"S01E$n"*'.mkv')"

  ass_en="${target%.*}.en.ass"
  ass_zh="${target%.*}.zh-TW.ass"

  # dump english subtitles
  rm -f "$ass_en"
  ffmpeg -i "$target" -map 0:2 -c copy "$ass_en"

  # dump traditional chinese subtitles
  rm -f "$ass_zh"
  ffmpeg -i "$input" -map 0:3 -c copy "$ass_zh"

  # remove japanese lines from chinese subtitles
  sed -i '/- JP/d' "$ass_zh"

  # append english subtitles to the chinese subtitles
  grep ,Default, "$ass_en" | sed 's|,Default,|,DefaultEN,|g' >> "$ass_zh"
  
  # inject english dialogue style
  sed -i '39 i Style: DefaultEN,Roboto Medium,70,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,1.3,0,8,20,20,23,0' "$ass_zh"

  # cleanup unnecessary english subtitle file
  rm "$ass_en"
done

