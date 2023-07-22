#!/bin/bash
bayc_address="0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D"
bayc_abi=$(cat ./bayc_abi.json)
blur_address="0x000000000000Ad05Ccc4F10045630fb830B95127"
blur_abi=$(cat ./blur_abi.json)
milady_address="0x5Af0D9827E0c53E4799BB226655A1de152A425a5"
doodles_address="0x8a90cab2b38dba80c64b7734e58ee1db38b8992e"

map_events_input="${bayc_address}\&\&${bayc_abi}"
filter_events_input="Transfer"
#filter_blur_trades_input="0x5Af0D9827E0c53E4799BB226655A1de152A425a5"
filter_blur_trades_input="0x8a90cab2b38dba80c64b7734e58ee1db38b8992e\&\&0x8a90cab2b38dba80c64b7734e58ee1db38b8992e"

if [ -e substreams.yaml.bak ]
then
   # if the backup file exists, it means the script was interrupted so we need to restore the original file
   mv substreams.yaml.bak substreams.yaml
fi

# make a backup of the original file
cp substreams.yaml substreams.yaml.bak

# replace the placeholder with the input string
sed -i "s|MAP_EVENTS_PARAMS|$map_events_input|g" substreams.yaml
sed -i "s|FILTER_EVENTS_PARAMS|$filter_events_input|g" substreams.yaml
sed -i "s|FILTER_BLUR_TRADES_PARAMS|$filter_blur_trades_input|g" substreams.yaml
sed -i "s|FILTER_SEAPORT_TRADES_PARAMS|$filter_blur_trades_input|g" substreams.yaml
sed -i "s|ETHERSCAN_OVERVIEW_PARAMS|$map_events_input|g" substreams.yaml

# run the substream
substreams gui blur_trades \
--start-block 17730372 \
--stop-block +5000 #\
#-e eth.firehose.pinax.network:9000


# restore the original file
mv substreams.yaml.bak substreams.yaml
