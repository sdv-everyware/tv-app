# tv-app

A home screen in the style of a smart TV launcher on harry (1920 x 1080): a wallpaper, a
greeting, the time and the weather, a ribbon of device cards that scrolls sideways (a horizontal
ListView of #tile) and round buttons. It is written against the EveryWare UI library
(`vendor/sdv-everyware/ui-lib`) only, as a test of the library from an app other than smarthome-
app.

## The design

`data/tv.dcf` is made without Figma: `design/tv-patch.js` (the screen, `#tv-root` from Library
slots) on the EveryWare Library, then ui-lib's `design/dark-theme-patch.js`, then `fig-to-dcf.sh`.
`design/build.sh` runs the whole chain:

```bash
packages/services/display_safety/utils/tv-app/design/build.sh     # from anywhere
```

It builds the Library (ui-lib `design/build-library.sh`, `design/build/library.fig` there), this
app's `design/tv.fig` (in git: the file a designer opens, see below) and
`data/tv.dcf`; `CHECK='#tv/time #tv/...'` names what must be in the .dcf.
fig-to-dcf.sh imports with everyware's `tools/dcf` (dcf-fetch `--local`, on our fork of
DesignCompose). A new board like this one: `vendor/sdv-everyware/ui-lib/skills/new-app/SKILL.md`.

The names under `#tv-root` (`#tv/*`) are the contract with `src/ui_model.rs`: rename both or neither.


## The design in Figma, without scripts

`design/tv.fig` is this app's screen as a Figma file: what is built, checked into git, and
what a designer opens. The plain path, no agent and no tools:

1. **Open it**: Figma, File > Import, `design/tv.fig`. The screen is the frame `#tv-root`;
   the Library (the components the screen is made of) is on its Components page.
2. **Change it** as in any Figma file: move, recolour, add a layer, swap an instance's variant.
   The layer names starting with `#` are the contract with the code (`src/ui_model.rs`): keep
   them; a new element the app should drive gets a `#tv/<name>` of its own, and the app a
   field with that name.
3. **Hand it over**: share the Figma file. The developer fetches it into the app once, from
   the tree's top, with a Figma token exported in the shell (never written down):
   `FIGMA_TOKEN=... vendor/sdv-everyware/everyware/scripts/fetch-dcf.sh packages/services/display_safety/utils/tv-app/data/tv.dcf <file key>`
   (the key is in the file's URL, `figma.com/design/<key>/...`), and the board shows it.

What the scripts add on top is only the other direction: `design/build.sh` writes this `.fig`
from `design/tv-patch.js` (an agent's or a developer's drawing in code) and the Library, and
makes the `.dcf` without Figma. Both ways meet in the same `.fig` and the same `.dcf`; the
`#names` are what keeps them together.

## Build and run on the RB3

```bash
# from the tree's top: build (sdv_media_db845c, out_rb3)
source device/rb3/sdv_media_db845c/tools/env.sh; m tv_app

# the panel's dashboard bundle on 127.0.0.1:7100 serves one app at a time: stop the other one first
adb shell mkdir -p /data/local/tmp/tv
adb push out_rb3/target/product/sdv_media_db845c/system/bin/tv_app /data/local/tmp/tv/
adb push packages/services/display_safety/utils/tv-app/data/tv.dcf /data/local/tmp/tv/
adb shell "cd /data/local/tmp/tv; LD_LIBRARY_PATH=/system/lib64:/vendor/lib64:/apex/com.everyware.smarthome/lib64 \
    ./tv_app --data-path /data/local/tmp/tv --touch-size 1920x1080"
```

The fonts are compiled in (`data/*.otf`); the time comes from the board's clock, the weather
from the dashboard bundle.
