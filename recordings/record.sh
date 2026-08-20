#!/bin/sh
set -eu

if [ "$#" -gt 1 ]; then
    echo "usage: record-examples [example]" >&2
    exit 2
fi

if [ "$#" -eq 1 ]; then
    case "$1" in
        *[!a-z0-9_]*)
            echo "invalid example name: $1" >&2
            exit 2
            ;;
    esac
    tapes="/vhs/recordings/$1.tape"
    if [ ! -f "$tapes" ]; then
        echo "no recording tape exists for example '$1'" >&2
        exit 2
    fi
else
    tapes="/vhs/recordings/*.tape"
fi

for tape in $tapes; do
    if [ "$(basename "$tape")" = "common.tape" ]; then
        continue
    fi

    example="$(basename "$tape" .tape)"
    echo "Recording $example..."
    /usr/bin/vhs "$tape"

    output="/vhs/assets/$example.gif"
    if [ ! -s "$output" ]; then
        echo "$tape did not create $output" >&2
        exit 1
    fi

    expected_width="$(awk '$1 == "Set" && $2 == "Width" { print $3 }' /vhs/recordings/common.tape)"
    expected_height="$(awk '$1 == "Set" && $2 == "Height" { print $3 }' "$tape")"
    actual_dimensions="$(ffprobe -v error -select_streams v:0 -show_entries stream=width,height -of csv=s=x:p=0 "$output")"
    if [ "$actual_dimensions" != "${expected_width}x${expected_height}" ]; then
        echo "$output is $actual_dimensions; expected ${expected_width}x${expected_height}" >&2
        exit 1
    fi
done
