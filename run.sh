#!/usr/bin/env sh

# substreams run map_hotdog \
# --start-block 420 \
# --stop-block +10 \
# -p map_key_value="epic string!" \
# -p map_hotdog="foo-bar"

substreams run map_event \
--start-block 17436825 \
--stop-block +10 \
-p map_event="0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D&&(Transfer&address_indexed_from&address_indexed_to&uint256_indexed_value)"
