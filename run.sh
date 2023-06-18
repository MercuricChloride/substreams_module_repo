#!/usr/bin/env sh

substreams run map_event \
--start-block 17436825 \
--stop-block +100 \
-p map_event="0x5Af0D9827E0c53E4799BB226655A1de152A425a5&&(Transfer&address_indexed_from&address_indexed_to&uint256_indexed_tokenId)" \
#--stop-block +10 \
#-p map_event="0xdAC17F958D2ee523a2206206994597C13D831ec7&&(Transfer&address_indexed_from&address_indexed_to&uint256_value)" \
#-p map_event="0x5Af0D9827E0c53E4799BB226655A1de152A425a5&&(Approval&address_indexed_owner&address_indexed_approved&uint256_indexed_tokenId)"
#-p map_event="0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D&&(Transfer&address_indexed_from&address_indexed_to&uint256_indexed_value)"
