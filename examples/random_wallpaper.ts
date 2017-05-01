
task change_wallpaper:
  = wallpaper (s "path/to/wallpaper_" (format_decimal (* (random) 5) 0) ".jpg"))
  set_wallpaper (wallpaper)
  say "changing wallpaper!"
end

do every minute:
  change_wallpaper
end

