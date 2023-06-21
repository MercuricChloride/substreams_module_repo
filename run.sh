#!/usr/bin/fish
set bayc_address "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D"
set bayc_abi (cat ./bayc_abi.json)
set blur_address "0x000000000000Ad05Ccc4F10045630fb830B95127"
set blur_abi (cat ./blur_abi.json)

set map_events_input (string join "\&\&" $bayc_address $bayc_abi $blur_address $blur_abi)
set filter_events_input "Transfer\&\&Approval"

echo $map_events_input
exit 1

if test -e substreams.yaml.bak
   # if the backup file exists, it means the script was interrupted so we need to restore the original file
   mv substreams.yaml.bak substreams.yaml
end

# make a backup of the original file
cp substreams.yaml substreams.yaml.bak

# replace the placeholder with the input string
sed -i "s/MAP_EVENTS_PARAMS/$map_events_input/g" substreams.yaml
sed -i "s/FILTER_EVENTS_PARAMS/$filter_events_input/g" substreams.yaml

# run the substream
substreams run map_events \
--start-block 17000205 \
--stop-block +1

# restore the original file
mv substreams.yaml.bak substreams.yaml
