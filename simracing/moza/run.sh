#!/bin/bash
cd "$(dirname "$0")"
cargo run &
RUNNER_PID=$!

cd -

echo "PID is $RUNNER_PID"

WINDOW_NAME="AMS2 RPM Overlay"

while ! xwininfo -root -tree | grep "$WINDOW_NAME"; do
    :
done

echo "Window has launched."
WINDOW_ID=$(xwininfo -root -tree | grep "$WINDOW_NAME" | awk '{print $1}')
WINDOW_ID=${WINDOW_ID#0x}
WINDOW_ID=$(printf '%8s' "$WINDOW_ID" | tr ' ' 0 | tr '[:lower:]' '[:upper:]')
echo "Window ID is $WINDOW_ID"

while ! bspc query -N -n .window | grep "0x$WINDOW_ID"; do
    :
done

# bspc query -T -n "0x$WINDOW_ID"
# echo $?
bspc node "0x$WINDOW_ID" -l above

wait $RUNNER_PID
echo "Done!"
