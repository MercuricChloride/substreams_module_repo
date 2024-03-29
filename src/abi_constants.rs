pub const SEAPORT: &str = r#"[
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "conduitController",
        "type": "address"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "constructor"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "newCounter",
        "type": "uint256"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "offerer",
        "type": "address"
      }
    ],
    "name": "CounterIncremented",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "orderHash",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "offerer",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "zone",
        "type": "address"
      }
    ],
    "name": "OrderCancelled",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "orderHash",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "offerer",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "zone",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "recipient",
        "type": "address"
      },
      {
        "components": [
          {
            "internalType": "enum ItemType",
            "name": "itemType",
            "type": "uint8"
          },
          {
            "internalType": "address",
            "name": "token",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "identifier",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "amount",
            "type": "uint256"
          }
        ],
        "indexed": false,
        "internalType": "struct SpentItem[]",
        "name": "offer",
        "type": "tuple[]"
      },
      {
        "components": [
          {
            "internalType": "enum ItemType",
            "name": "itemType",
            "type": "uint8"
          },
          {
            "internalType": "address",
            "name": "token",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "identifier",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "amount",
            "type": "uint256"
          },
          {
            "internalType": "address payable",
            "name": "recipient",
            "type": "address"
          }
        ],
        "indexed": false,
        "internalType": "struct ReceivedItem[]",
        "name": "consideration",
        "type": "tuple[]"
      }
    ],
    "name": "OrderFulfilled",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "orderHash",
        "type": "bytes32"
      },
      {
        "components": [
          {
            "internalType": "address",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "zone",
            "type": "address"
          },
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endAmount",
                "type": "uint256"
              }
            ],
            "internalType": "struct OfferItem[]",
            "name": "offer",
            "type": "tuple[]"
          },
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endAmount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct ConsiderationItem[]",
            "name": "consideration",
            "type": "tuple[]"
          },
          {
            "internalType": "enum OrderType",
            "name": "orderType",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "startTime",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "endTime",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "zoneHash",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "salt",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "conduitKey",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "totalOriginalConsiderationItems",
            "type": "uint256"
          }
        ],
        "indexed": false,
        "internalType": "struct OrderParameters",
        "name": "orderParameters",
        "type": "tuple"
      }
    ],
    "name": "OrderValidated",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32[]",
        "name": "orderHashes",
        "type": "bytes32[]"
      }
    ],
    "name": "OrdersMatched",
    "type": "event"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "internalType": "address",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "zone",
            "type": "address"
          },
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endAmount",
                "type": "uint256"
              }
            ],
            "internalType": "struct OfferItem[]",
            "name": "offer",
            "type": "tuple[]"
          },
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endAmount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct ConsiderationItem[]",
            "name": "consideration",
            "type": "tuple[]"
          },
          {
            "internalType": "enum OrderType",
            "name": "orderType",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "startTime",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "endTime",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "zoneHash",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "salt",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "conduitKey",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "counter",
            "type": "uint256"
          }
        ],
        "internalType": "struct OrderComponents[]",
        "name": "orders",
        "type": "tuple[]"
      }
    ],
    "name": "cancel",
    "outputs": [
      {
        "internalType": "bool",
        "name": "cancelled",
        "type": "bool"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "address",
                "name": "offerer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "zone",
                "type": "address"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  }
                ],
                "internalType": "struct OfferItem[]",
                "name": "offer",
                "type": "tuple[]"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "address payable",
                    "name": "recipient",
                    "type": "address"
                  }
                ],
                "internalType": "struct ConsiderationItem[]",
                "name": "consideration",
                "type": "tuple[]"
              },
              {
                "internalType": "enum OrderType",
                "name": "orderType",
                "type": "uint8"
              },
              {
                "internalType": "uint256",
                "name": "startTime",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endTime",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "totalOriginalConsiderationItems",
                "type": "uint256"
              }
            ],
            "internalType": "struct OrderParameters",
            "name": "parameters",
            "type": "tuple"
          },
          {
            "internalType": "uint120",
            "name": "numerator",
            "type": "uint120"
          },
          {
            "internalType": "uint120",
            "name": "denominator",
            "type": "uint120"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          },
          {
            "internalType": "bytes",
            "name": "extraData",
            "type": "bytes"
          }
        ],
        "internalType": "struct AdvancedOrder",
        "name": "",
        "type": "tuple"
      },
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "orderIndex",
            "type": "uint256"
          },
          {
            "internalType": "enum Side",
            "name": "side",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "index",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "identifier",
            "type": "uint256"
          },
          {
            "internalType": "bytes32[]",
            "name": "criteriaProof",
            "type": "bytes32[]"
          }
        ],
        "internalType": "struct CriteriaResolver[]",
        "name": "",
        "type": "tuple[]"
      },
      {
        "internalType": "bytes32",
        "name": "fulfillerConduitKey",
        "type": "bytes32"
      },
      {
        "internalType": "address",
        "name": "recipient",
        "type": "address"
      }
    ],
    "name": "fulfillAdvancedOrder",
    "outputs": [
      {
        "internalType": "bool",
        "name": "fulfilled",
        "type": "bool"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "address",
                "name": "offerer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "zone",
                "type": "address"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  }
                ],
                "internalType": "struct OfferItem[]",
                "name": "offer",
                "type": "tuple[]"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "address payable",
                    "name": "recipient",
                    "type": "address"
                  }
                ],
                "internalType": "struct ConsiderationItem[]",
                "name": "consideration",
                "type": "tuple[]"
              },
              {
                "internalType": "enum OrderType",
                "name": "orderType",
                "type": "uint8"
              },
              {
                "internalType": "uint256",
                "name": "startTime",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endTime",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "totalOriginalConsiderationItems",
                "type": "uint256"
              }
            ],
            "internalType": "struct OrderParameters",
            "name": "parameters",
            "type": "tuple"
          },
          {
            "internalType": "uint120",
            "name": "numerator",
            "type": "uint120"
          },
          {
            "internalType": "uint120",
            "name": "denominator",
            "type": "uint120"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          },
          {
            "internalType": "bytes",
            "name": "extraData",
            "type": "bytes"
          }
        ],
        "internalType": "struct AdvancedOrder[]",
        "name": "",
        "type": "tuple[]"
      },
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "orderIndex",
            "type": "uint256"
          },
          {
            "internalType": "enum Side",
            "name": "side",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "index",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "identifier",
            "type": "uint256"
          },
          {
            "internalType": "bytes32[]",
            "name": "criteriaProof",
            "type": "bytes32[]"
          }
        ],
        "internalType": "struct CriteriaResolver[]",
        "name": "",
        "type": "tuple[]"
      },
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "orderIndex",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "itemIndex",
            "type": "uint256"
          }
        ],
        "internalType": "struct FulfillmentComponent[][]",
        "name": "",
        "type": "tuple[][]"
      },
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "orderIndex",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "itemIndex",
            "type": "uint256"
          }
        ],
        "internalType": "struct FulfillmentComponent[][]",
        "name": "",
        "type": "tuple[][]"
      },
      {
        "internalType": "bytes32",
        "name": "fulfillerConduitKey",
        "type": "bytes32"
      },
      {
        "internalType": "address",
        "name": "recipient",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "maximumFulfilled",
        "type": "uint256"
      }
    ],
    "name": "fulfillAvailableAdvancedOrders",
    "outputs": [
      {
        "internalType": "bool[]",
        "name": "",
        "type": "bool[]"
      },
      {
        "components": [
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifier",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct ReceivedItem",
            "name": "item",
            "type": "tuple"
          },
          {
            "internalType": "address",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "bytes32",
            "name": "conduitKey",
            "type": "bytes32"
          }
        ],
        "internalType": "struct Execution[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "address",
                "name": "offerer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "zone",
                "type": "address"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  }
                ],
                "internalType": "struct OfferItem[]",
                "name": "offer",
                "type": "tuple[]"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "address payable",
                    "name": "recipient",
                    "type": "address"
                  }
                ],
                "internalType": "struct ConsiderationItem[]",
                "name": "consideration",
                "type": "tuple[]"
              },
              {
                "internalType": "enum OrderType",
                "name": "orderType",
                "type": "uint8"
              },
              {
                "internalType": "uint256",
                "name": "startTime",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endTime",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "totalOriginalConsiderationItems",
                "type": "uint256"
              }
            ],
            "internalType": "struct OrderParameters",
            "name": "parameters",
            "type": "tuple"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          }
        ],
        "internalType": "struct Order[]",
        "name": "",
        "type": "tuple[]"
      },
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "orderIndex",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "itemIndex",
            "type": "uint256"
          }
        ],
        "internalType": "struct FulfillmentComponent[][]",
        "name": "",
        "type": "tuple[][]"
      },
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "orderIndex",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "itemIndex",
            "type": "uint256"
          }
        ],
        "internalType": "struct FulfillmentComponent[][]",
        "name": "",
        "type": "tuple[][]"
      },
      {
        "internalType": "bytes32",
        "name": "fulfillerConduitKey",
        "type": "bytes32"
      },
      {
        "internalType": "uint256",
        "name": "maximumFulfilled",
        "type": "uint256"
      }
    ],
    "name": "fulfillAvailableOrders",
    "outputs": [
      {
        "internalType": "bool[]",
        "name": "",
        "type": "bool[]"
      },
      {
        "components": [
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifier",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct ReceivedItem",
            "name": "item",
            "type": "tuple"
          },
          {
            "internalType": "address",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "bytes32",
            "name": "conduitKey",
            "type": "bytes32"
          }
        ],
        "internalType": "struct Execution[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "internalType": "address",
            "name": "considerationToken",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "considerationIdentifier",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "considerationAmount",
            "type": "uint256"
          },
          {
            "internalType": "address payable",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "zone",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "offerToken",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "offerIdentifier",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "offerAmount",
            "type": "uint256"
          },
          {
            "internalType": "enum BasicOrderType",
            "name": "basicOrderType",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "startTime",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "endTime",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "zoneHash",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "salt",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "offererConduitKey",
            "type": "bytes32"
          },
          {
            "internalType": "bytes32",
            "name": "fulfillerConduitKey",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "totalOriginalAdditionalRecipients",
            "type": "uint256"
          },
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct AdditionalRecipient[]",
            "name": "additionalRecipients",
            "type": "tuple[]"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          }
        ],
        "internalType": "struct BasicOrderParameters",
        "name": "parameters",
        "type": "tuple"
      }
    ],
    "name": "fulfillBasicOrder",
    "outputs": [
      {
        "internalType": "bool",
        "name": "fulfilled",
        "type": "bool"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "internalType": "address",
            "name": "considerationToken",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "considerationIdentifier",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "considerationAmount",
            "type": "uint256"
          },
          {
            "internalType": "address payable",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "zone",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "offerToken",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "offerIdentifier",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "offerAmount",
            "type": "uint256"
          },
          {
            "internalType": "enum BasicOrderType",
            "name": "basicOrderType",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "startTime",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "endTime",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "zoneHash",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "salt",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "offererConduitKey",
            "type": "bytes32"
          },
          {
            "internalType": "bytes32",
            "name": "fulfillerConduitKey",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "totalOriginalAdditionalRecipients",
            "type": "uint256"
          },
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct AdditionalRecipient[]",
            "name": "additionalRecipients",
            "type": "tuple[]"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          }
        ],
        "internalType": "struct BasicOrderParameters",
        "name": "parameters",
        "type": "tuple"
      }
    ],
    "name": "fulfillBasicOrder_efficient_6GL6yc",
    "outputs": [
      {
        "internalType": "bool",
        "name": "fulfilled",
        "type": "bool"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "address",
                "name": "offerer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "zone",
                "type": "address"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  }
                ],
                "internalType": "struct OfferItem[]",
                "name": "offer",
                "type": "tuple[]"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "address payable",
                    "name": "recipient",
                    "type": "address"
                  }
                ],
                "internalType": "struct ConsiderationItem[]",
                "name": "consideration",
                "type": "tuple[]"
              },
              {
                "internalType": "enum OrderType",
                "name": "orderType",
                "type": "uint8"
              },
              {
                "internalType": "uint256",
                "name": "startTime",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endTime",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "totalOriginalConsiderationItems",
                "type": "uint256"
              }
            ],
            "internalType": "struct OrderParameters",
            "name": "parameters",
            "type": "tuple"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          }
        ],
        "internalType": "struct Order",
        "name": "",
        "type": "tuple"
      },
      {
        "internalType": "bytes32",
        "name": "fulfillerConduitKey",
        "type": "bytes32"
      }
    ],
    "name": "fulfillOrder",
    "outputs": [
      {
        "internalType": "bool",
        "name": "fulfilled",
        "type": "bool"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "contractOfferer",
        "type": "address"
      }
    ],
    "name": "getContractOffererNonce",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "nonce",
        "type": "uint256"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "offerer",
        "type": "address"
      }
    ],
    "name": "getCounter",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "counter",
        "type": "uint256"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "internalType": "address",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "zone",
            "type": "address"
          },
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endAmount",
                "type": "uint256"
              }
            ],
            "internalType": "struct OfferItem[]",
            "name": "offer",
            "type": "tuple[]"
          },
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endAmount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct ConsiderationItem[]",
            "name": "consideration",
            "type": "tuple[]"
          },
          {
            "internalType": "enum OrderType",
            "name": "orderType",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "startTime",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "endTime",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "zoneHash",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "salt",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "conduitKey",
            "type": "bytes32"
          },
          {
            "internalType": "uint256",
            "name": "counter",
            "type": "uint256"
          }
        ],
        "internalType": "struct OrderComponents",
        "name": "",
        "type": "tuple"
      }
    ],
    "name": "getOrderHash",
    "outputs": [
      {
        "internalType": "bytes32",
        "name": "orderHash",
        "type": "bytes32"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "orderHash",
        "type": "bytes32"
      }
    ],
    "name": "getOrderStatus",
    "outputs": [
      {
        "internalType": "bool",
        "name": "isValidated",
        "type": "bool"
      },
      {
        "internalType": "bool",
        "name": "isCancelled",
        "type": "bool"
      },
      {
        "internalType": "uint256",
        "name": "totalFilled",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "totalSize",
        "type": "uint256"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "incrementCounter",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "newCounter",
        "type": "uint256"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "information",
    "outputs": [
      {
        "internalType": "string",
        "name": "version",
        "type": "string"
      },
      {
        "internalType": "bytes32",
        "name": "domainSeparator",
        "type": "bytes32"
      },
      {
        "internalType": "address",
        "name": "conduitController",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "address",
                "name": "offerer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "zone",
                "type": "address"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  }
                ],
                "internalType": "struct OfferItem[]",
                "name": "offer",
                "type": "tuple[]"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "address payable",
                    "name": "recipient",
                    "type": "address"
                  }
                ],
                "internalType": "struct ConsiderationItem[]",
                "name": "consideration",
                "type": "tuple[]"
              },
              {
                "internalType": "enum OrderType",
                "name": "orderType",
                "type": "uint8"
              },
              {
                "internalType": "uint256",
                "name": "startTime",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endTime",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "totalOriginalConsiderationItems",
                "type": "uint256"
              }
            ],
            "internalType": "struct OrderParameters",
            "name": "parameters",
            "type": "tuple"
          },
          {
            "internalType": "uint120",
            "name": "numerator",
            "type": "uint120"
          },
          {
            "internalType": "uint120",
            "name": "denominator",
            "type": "uint120"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          },
          {
            "internalType": "bytes",
            "name": "extraData",
            "type": "bytes"
          }
        ],
        "internalType": "struct AdvancedOrder[]",
        "name": "",
        "type": "tuple[]"
      },
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "orderIndex",
            "type": "uint256"
          },
          {
            "internalType": "enum Side",
            "name": "side",
            "type": "uint8"
          },
          {
            "internalType": "uint256",
            "name": "index",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "identifier",
            "type": "uint256"
          },
          {
            "internalType": "bytes32[]",
            "name": "criteriaProof",
            "type": "bytes32[]"
          }
        ],
        "internalType": "struct CriteriaResolver[]",
        "name": "",
        "type": "tuple[]"
      },
      {
        "components": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "orderIndex",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "itemIndex",
                "type": "uint256"
              }
            ],
            "internalType": "struct FulfillmentComponent[]",
            "name": "offerComponents",
            "type": "tuple[]"
          },
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "orderIndex",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "itemIndex",
                "type": "uint256"
              }
            ],
            "internalType": "struct FulfillmentComponent[]",
            "name": "considerationComponents",
            "type": "tuple[]"
          }
        ],
        "internalType": "struct Fulfillment[]",
        "name": "",
        "type": "tuple[]"
      },
      {
        "internalType": "address",
        "name": "recipient",
        "type": "address"
      }
    ],
    "name": "matchAdvancedOrders",
    "outputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifier",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct ReceivedItem",
            "name": "item",
            "type": "tuple"
          },
          {
            "internalType": "address",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "bytes32",
            "name": "conduitKey",
            "type": "bytes32"
          }
        ],
        "internalType": "struct Execution[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "address",
                "name": "offerer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "zone",
                "type": "address"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  }
                ],
                "internalType": "struct OfferItem[]",
                "name": "offer",
                "type": "tuple[]"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "address payable",
                    "name": "recipient",
                    "type": "address"
                  }
                ],
                "internalType": "struct ConsiderationItem[]",
                "name": "consideration",
                "type": "tuple[]"
              },
              {
                "internalType": "enum OrderType",
                "name": "orderType",
                "type": "uint8"
              },
              {
                "internalType": "uint256",
                "name": "startTime",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endTime",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "totalOriginalConsiderationItems",
                "type": "uint256"
              }
            ],
            "internalType": "struct OrderParameters",
            "name": "parameters",
            "type": "tuple"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          }
        ],
        "internalType": "struct Order[]",
        "name": "",
        "type": "tuple[]"
      },
      {
        "components": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "orderIndex",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "itemIndex",
                "type": "uint256"
              }
            ],
            "internalType": "struct FulfillmentComponent[]",
            "name": "offerComponents",
            "type": "tuple[]"
          },
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "orderIndex",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "itemIndex",
                "type": "uint256"
              }
            ],
            "internalType": "struct FulfillmentComponent[]",
            "name": "considerationComponents",
            "type": "tuple[]"
          }
        ],
        "internalType": "struct Fulfillment[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "name": "matchOrders",
    "outputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "enum ItemType",
                "name": "itemType",
                "type": "uint8"
              },
              {
                "internalType": "address",
                "name": "token",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "identifier",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
              },
              {
                "internalType": "address payable",
                "name": "recipient",
                "type": "address"
              }
            ],
            "internalType": "struct ReceivedItem",
            "name": "item",
            "type": "tuple"
          },
          {
            "internalType": "address",
            "name": "offerer",
            "type": "address"
          },
          {
            "internalType": "bytes32",
            "name": "conduitKey",
            "type": "bytes32"
          }
        ],
        "internalType": "struct Execution[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "name",
    "outputs": [
      {
        "internalType": "string",
        "name": "",
        "type": "string"
      }
    ],
    "stateMutability": "pure",
    "type": "function"
  },
  {
    "inputs": [
      {
        "components": [
          {
            "components": [
              {
                "internalType": "address",
                "name": "offerer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "zone",
                "type": "address"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  }
                ],
                "internalType": "struct OfferItem[]",
                "name": "offer",
                "type": "tuple[]"
              },
              {
                "components": [
                  {
                    "internalType": "enum ItemType",
                    "name": "itemType",
                    "type": "uint8"
                  },
                  {
                    "internalType": "address",
                    "name": "token",
                    "type": "address"
                  },
                  {
                    "internalType": "uint256",
                    "name": "identifierOrCriteria",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "startAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "uint256",
                    "name": "endAmount",
                    "type": "uint256"
                  },
                  {
                    "internalType": "address payable",
                    "name": "recipient",
                    "type": "address"
                  }
                ],
                "internalType": "struct ConsiderationItem[]",
                "name": "consideration",
                "type": "tuple[]"
              },
              {
                "internalType": "enum OrderType",
                "name": "orderType",
                "type": "uint8"
              },
              {
                "internalType": "uint256",
                "name": "startTime",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "endTime",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "bytes32",
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "internalType": "uint256",
                "name": "totalOriginalConsiderationItems",
                "type": "uint256"
              }
            ],
            "internalType": "struct OrderParameters",
            "name": "parameters",
            "type": "tuple"
          },
          {
            "internalType": "bytes",
            "name": "signature",
            "type": "bytes"
          }
        ],
        "internalType": "struct Order[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "name": "validate",
    "outputs": [
      {
        "internalType": "bool",
        "name": "",
        "type": "bool"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "stateMutability": "payable",
    "type": "receive"
  }
]
"#;
pub const BLUR: &str = r#"[{"inputs":[],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"address","name":"previousAdmin","type":"address"},{"indexed":false,"internalType":"address","name":"newAdmin","type":"address"}],"name":"AdminChanged","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"beacon","type":"address"}],"name":"BeaconUpgraded","type":"event"},{"anonymous":false,"inputs":[],"name":"Closed","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint8","name":"version","type":"uint8"}],"name":"Initialized","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"blockRange","type":"uint256"}],"name":"NewBlockRange","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"contract IExecutionDelegate","name":"executionDelegate","type":"address"}],"name":"NewExecutionDelegate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"feeRate","type":"uint256"}],"name":"NewFeeRate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"address","name":"feeRecipient","type":"address"}],"name":"NewFeeRecipient","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"address","name":"governor","type":"address"}],"name":"NewGovernor","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"oracle","type":"address"}],"name":"NewOracle","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"contract IPolicyManager","name":"policyManager","type":"address"}],"name":"NewPolicyManager","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"trader","type":"address"},{"indexed":false,"internalType":"uint256","name":"newNonce","type":"uint256"}],"name":"NonceIncremented","type":"event"},{"anonymous":false,"inputs":[],"name":"Opened","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"bytes32","name":"hash","type":"bytes32"}],"name":"OrderCancelled","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"maker","type":"address"},{"indexed":true,"internalType":"address","name":"taker","type":"address"},{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"indexed":false,"internalType":"struct Order","name":"sell","type":"tuple"},{"indexed":false,"internalType":"bytes32","name":"sellHash","type":"bytes32"},{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"indexed":false,"internalType":"struct Order","name":"buy","type":"tuple"},{"indexed":false,"internalType":"bytes32","name":"buyHash","type":"bytes32"}],"name":"OrdersMatched","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"implementation","type":"address"}],"name":"Upgraded","type":"event"},{"inputs":[],"name":"FEE_TYPEHASH","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"INVERSE_BASIS_POINT","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"NAME","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"ORACLE_ORDER_TYPEHASH","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"ORDER_TYPEHASH","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"POOL","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"ROOT_TYPEHASH","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"VERSION","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"WETH","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"components":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"internalType":"struct Order","name":"order","type":"tuple"},{"internalType":"uint8","name":"v","type":"uint8"},{"internalType":"bytes32","name":"r","type":"bytes32"},{"internalType":"bytes32","name":"s","type":"bytes32"},{"internalType":"bytes","name":"extraSignature","type":"bytes"},{"internalType":"enum SignatureVersion","name":"signatureVersion","type":"uint8"},{"internalType":"uint256","name":"blockNumber","type":"uint256"}],"internalType":"struct Input","name":"sell","type":"tuple"},{"components":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"internalType":"struct Order","name":"order","type":"tuple"},{"internalType":"uint8","name":"v","type":"uint8"},{"internalType":"bytes32","name":"r","type":"bytes32"},{"internalType":"bytes32","name":"s","type":"bytes32"},{"internalType":"bytes","name":"extraSignature","type":"bytes"},{"internalType":"enum SignatureVersion","name":"signatureVersion","type":"uint8"},{"internalType":"uint256","name":"blockNumber","type":"uint256"}],"internalType":"struct Input","name":"buy","type":"tuple"}],"name":"_execute","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[],"name":"blockRange","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"components":[{"components":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"internalType":"struct Order","name":"order","type":"tuple"},{"internalType":"uint8","name":"v","type":"uint8"},{"internalType":"bytes32","name":"r","type":"bytes32"},{"internalType":"bytes32","name":"s","type":"bytes32"},{"internalType":"bytes","name":"extraSignature","type":"bytes"},{"internalType":"enum SignatureVersion","name":"signatureVersion","type":"uint8"},{"internalType":"uint256","name":"blockNumber","type":"uint256"}],"internalType":"struct Input","name":"sell","type":"tuple"},{"components":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"internalType":"struct Order","name":"order","type":"tuple"},{"internalType":"uint8","name":"v","type":"uint8"},{"internalType":"bytes32","name":"r","type":"bytes32"},{"internalType":"bytes32","name":"s","type":"bytes32"},{"internalType":"bytes","name":"extraSignature","type":"bytes"},{"internalType":"enum SignatureVersion","name":"signatureVersion","type":"uint8"},{"internalType":"uint256","name":"blockNumber","type":"uint256"}],"internalType":"struct Input","name":"buy","type":"tuple"}],"internalType":"struct Execution[]","name":"executions","type":"tuple[]"}],"name":"bulkExecute","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"internalType":"struct Order","name":"order","type":"tuple"}],"name":"cancelOrder","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"internalType":"struct Order[]","name":"orders","type":"tuple[]"}],"name":"cancelOrders","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"name":"cancelledOrFilled","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"close","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"components":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"internalType":"struct Order","name":"order","type":"tuple"},{"internalType":"uint8","name":"v","type":"uint8"},{"internalType":"bytes32","name":"r","type":"bytes32"},{"internalType":"bytes32","name":"s","type":"bytes32"},{"internalType":"bytes","name":"extraSignature","type":"bytes"},{"internalType":"enum SignatureVersion","name":"signatureVersion","type":"uint8"},{"internalType":"uint256","name":"blockNumber","type":"uint256"}],"internalType":"struct Input","name":"sell","type":"tuple"},{"components":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"enum Side","name":"side","type":"uint8"},{"internalType":"address","name":"matchingPolicy","type":"address"},{"internalType":"address","name":"collection","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"address","name":"paymentToken","type":"address"},{"internalType":"uint256","name":"price","type":"uint256"},{"internalType":"uint256","name":"listingTime","type":"uint256"},{"internalType":"uint256","name":"expirationTime","type":"uint256"},{"components":[{"internalType":"uint16","name":"rate","type":"uint16"},{"internalType":"address payable","name":"recipient","type":"address"}],"internalType":"struct Fee[]","name":"fees","type":"tuple[]"},{"internalType":"uint256","name":"salt","type":"uint256"},{"internalType":"bytes","name":"extraParams","type":"bytes"}],"internalType":"struct Order","name":"order","type":"tuple"},{"internalType":"uint8","name":"v","type":"uint8"},{"internalType":"bytes32","name":"r","type":"bytes32"},{"internalType":"bytes32","name":"s","type":"bytes32"},{"internalType":"bytes","name":"extraSignature","type":"bytes"},{"internalType":"enum SignatureVersion","name":"signatureVersion","type":"uint8"},{"internalType":"uint256","name":"blockNumber","type":"uint256"}],"internalType":"struct Input","name":"buy","type":"tuple"}],"name":"execute","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[],"name":"executionDelegate","outputs":[{"internalType":"contract IExecutionDelegate","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"feeRate","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"feeRecipient","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"governor","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"incrementNonce","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"contract IExecutionDelegate","name":"_executionDelegate","type":"address"},{"internalType":"contract IPolicyManager","name":"_policyManager","type":"address"},{"internalType":"address","name":"_oracle","type":"address"},{"internalType":"uint256","name":"_blockRange","type":"uint256"}],"name":"initialize","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"isInternal","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"isOpen","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"","type":"address"}],"name":"nonces","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"open","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"oracle","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"policyManager","outputs":[{"internalType":"contract IPolicyManager","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"proxiableUUID","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"remainingETH","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"_blockRange","type":"uint256"}],"name":"setBlockRange","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"contract IExecutionDelegate","name":"_executionDelegate","type":"address"}],"name":"setExecutionDelegate","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"_feeRate","type":"uint256"}],"name":"setFeeRate","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"_feeRecipient","type":"address"}],"name":"setFeeRecipient","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"_governor","type":"address"}],"name":"setGovernor","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"_oracle","type":"address"}],"name":"setOracle","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"contract IPolicyManager","name":"_policyManager","type":"address"}],"name":"setPolicyManager","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newImplementation","type":"address"}],"name":"upgradeTo","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newImplementation","type":"address"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"upgradeToAndCall","outputs":[],"stateMutability":"payable","type":"function"}]
"#;
pub const BAYC: &str = r#"[{"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"string","name":"symbol","type":"string"},{"internalType":"uint256","name":"maxNftSupply","type":"uint256"},{"internalType":"uint256","name":"saleStart","type":"uint256"}],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":true,"internalType":"address","name":"approved","type":"address"},{"indexed":true,"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":true,"internalType":"address","name":"operator","type":"address"},{"indexed":false,"internalType":"bool","name":"approved","type":"bool"}],"name":"ApprovalForAll","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"from","type":"address"},{"indexed":true,"internalType":"address","name":"to","type":"address"},{"indexed":true,"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"Transfer","type":"event"},{"inputs":[],"name":"BAYC_PROVENANCE","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"MAX_APES","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"REVEAL_TIMESTAMP","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"apePrice","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"approve","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"balanceOf","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"baseURI","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"emergencySetStartingIndexBlock","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"flipSaleState","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"getApproved","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"address","name":"operator","type":"address"}],"name":"isApprovedForAll","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"maxApePurchase","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"numberOfTokens","type":"uint256"}],"name":"mintApe","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[],"name":"name","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ownerOf","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"reserveApes","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"bytes","name":"_data","type":"bytes"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"saleIsActive","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"operator","type":"address"},{"internalType":"bool","name":"approved","type":"bool"}],"name":"setApprovalForAll","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"baseURI","type":"string"}],"name":"setBaseURI","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"provenanceHash","type":"string"}],"name":"setProvenanceHash","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"revealTimeStamp","type":"uint256"}],"name":"setRevealTimestamp","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"setStartingIndex","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"startingIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"startingIndexBlock","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"bytes4","name":"interfaceId","type":"bytes4"}],"name":"supportsInterface","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"symbol","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"index","type":"uint256"}],"name":"tokenByIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"index","type":"uint256"}],"name":"tokenOfOwnerByIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"tokenURI","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"totalSupply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"transferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"withdraw","outputs":[],"stateMutability":"nonpayable","type":"function"}]
"#;
