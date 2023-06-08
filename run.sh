#!/usr/bin/env sh

substreams run map_hotdog \
--start-block 420 \
--stop-block +10 \
-p map_key_value="epic string!" \
-p map_hotdog="foo"
