#!/bin/bash
# This app's design, end to end: the EveryWare Library (ui-lib design/build-library.sh), this
# app's patch on it, the Library's dark theme, then data/<app>.dcf (fig-to-dcf.sh, the previous
# one kept next to it). The app's name is its folder's (<app>-app). Usage, from anywhere:
#
#   design/build.sh                        # -> design/<app>.fig and data/<app>.dcf
#   CHECK='#<app>/time #<app>/ok' design/build.sh   # names that must be in the .dcf
#
# design/<app>.fig is the result, in git: the screen as a designer opens it in Figma (README,
# "The design in Figma, without scripts"). design/build/ holds the intermediate files, not in
# git. The tools are everyware's design/figma (EVERYWARE_FIGMA), the Library ui-lib
# (EVERYWARE_UI), both found in the tree by default. Needs node, unzip, zip and the tree's
# importer (cargo).
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
TOP=$(cd "$HERE/../../../../../.." && pwd)
APP=$(basename "$(dirname "$HERE")"); APP=${APP%-app}
FIGMA=${EVERYWARE_FIGMA:-$TOP/vendor/sdv-everyware/everyware/design/figma}
UI=${EVERYWARE_UI:-$TOP/vendor/sdv-everyware/ui-lib}
B="$HERE/build"
mkdir -p "$B"
"$UI/design/build-library.sh"
node "$FIGMA/roundtrip.js" "$UI/design/build/library.fig" "$B/$APP-light.fig" "$HERE/$APP-patch.js"
FIG="$HERE/$APP.fig"
node "$FIGMA/roundtrip.js" "$B/$APP-light.fig" "$FIG" "$UI/design/dark-theme-patch.js"
echo "built $FIG"
DCF_CHECK="${CHECK:-#$APP/time}" \
    "$TOP/vendor/sdv-everyware/everyware/scripts/fig-to-dcf.sh" "$FIG" "$HERE/../data/$APP.dcf"
# the last line: the file to give the user for a review
echo "review in Figma (File > Import): $FIG"
