#! /bin/bash

ffmpeg -i $1 -b:a 192K -ar 44100 $2
