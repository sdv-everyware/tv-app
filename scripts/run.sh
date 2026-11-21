#!/usr/bin/env bash
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Puts the built tv_app on a device and runs it in place of the panel.
#
#     scripts/run.sh        push and run
#     scripts/run.sh stop   stops it and starts the panel again
#     scripts/run.sh log    its log lines so far
#
# The device is the one adb talks to (ANDROID_SERIAL when several are connected). It says which
# product it is (ro.product.device), and the script takes the newest tv_app built for that
# product in the tree (<out dir>/target/product/<product>/system/bin). Build first, from the
# tree's top, with the product's environment, for example for the Cuttlefish VM:
#     source device/cf-everyware/sdv_media_everyware_cf/tools/env.sh; m tv_app
#
# The dashboard bundle talks to one screen at a time (the binder service IScreenLink), so the
# panel is stopped while this app runs. Another app started the same way has to be stopped
# with its own script first.
#
# Environment: ANDROID_SERIAL, TOUCH_SIZE=WxH (default 1920x1080).

set -euo pipefail

NAME=tv
APP="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TREE="$(cd "$APP/../../../../.." && pwd)"
REMOTE=/data/local/tmp/$NAME
# the libraries of the panel's APEX (the renderer's), then the image's
LIBS=/system/lib64:/vendor/lib64:/apex/com.everyware.smarthome/lib64

mode="${1:-run}"
case "$mode" in
run | stop | log) ;;
*)
    echo "usage: $0 [run|stop|log]"
    exit 1
    ;;
esac

# adb takes ANDROID_SERIAL itself, and says so when several devices are connected
A() {
    adb "$@"
}

if [ "$mode" = log ]; then
    A logcat -d | grep -E "${NAME}-app|${NAME}_app|dashboard_bundle" || true
    exit 0
fi

# the display and the binder service need root; adbd restarts after "adb root"
A root >/dev/null 2>&1 || true
A wait-for-device
[ "$(A shell id -u | tr -d '\r')" = 0 ] || { echo "adb is not root (a userdebug or eng image is needed)"; exit 1; }

if [ "$mode" = stop ]; then
    A shell "kill \$(pidof ${NAME}_app) 2>/dev/null; start smarthome"
    echo "${NAME}_app stopped, the panel started"
    exit 0
fi

product="$(A shell getprop ro.product.device | tr -d '\r')"
# the newest build for this product: a tree can hold several out dirs
bin="$(ls -t "$TREE"/*/target/product/"$product"/system/bin/"${NAME}_app" 2>/dev/null | head -1)"
[ -n "$bin" ] || { echo "no ${NAME}_app built for $product: m ${NAME}_app"; exit 1; }
echo "$product: ${bin#"$TREE"/}"

A shell "mkdir -p $REMOTE"
A push "$bin" "$REMOTE/" >/dev/null 2>&1
A push "$APP/data/$NAME.dcf" "$REMOTE/" >/dev/null 2>&1
A shell "stop smarthome; kill \$(pidof ${NAME}_app) 2>/dev/null; true"
A shell "cd $REMOTE; LD_LIBRARY_PATH=$LIBS nohup ./${NAME}_app --data-path $REMOTE --touch-size ${TOUCH_SIZE:-1920x1080} >$REMOTE/out.log 2>&1 &"
sleep 3
pid="$(A shell "pidof ${NAME}_app" | tr -d '\r')"
[ -n "$pid" ] || { echo "${NAME}_app did not stay up:"; A shell "tail -20 $REMOTE/out.log"; exit 1; }
echo "${NAME}_app runs (pid $pid); scripts/run.sh log, scripts/run.sh stop"
