#!/bin/bash

# Check if the correct number of arguments was provided
if [[ $# -ne 2 ]]; then
    echo "Usage: $0 <string> <filepath>"
    exit 1
fi

# Assign the arguments to variables
user_string="$1"
file_path="$2"

# Validate the file path
if [[ -e $file_path ]]; then
    # Concatenate the string and the file path, separated by &&
    abi=$(cat $file_path)
    echo $abi
    concat_string='${user_string}&&${abi}'

    # Print the concatenated string
    substreams run map_abi_events \
    --stop-block +100 \
    -p map_abi_events="${concat_string}"
else
    echo "The file path you provided does not exist."
fi

# --start-block 17436825 \
# --stop-block +100 \
# -p map_event="0x5Af0D9827E0c53E4799BB226655A1de152A425a5&&(Transfer&address_indexed_from&address_indexed_to&uint256_indexed_tokenId)" \
#--stop-block +10 \
#-p map_event="0xdAC17F958D2ee523a2206206994597C13D831ec7&&(Transfer&address_indexed_from&address_indexed_to&uint256_value)" \
#-p map_event="0x5Af0D9827E0c53E4799BB226655A1de152A425a5&&(Approval&address_indexed_owner&address_indexed_approved&uint256_indexed_tokenId)"
#-p map_event="0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D&&(Transfer&address_indexed_from&address_indexed_to&uint256_indexed_value)"
