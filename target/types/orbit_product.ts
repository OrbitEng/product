export type OrbitProduct = {
  "version": "0.1.0",
  "name": "orbit_product",
  "instructions": [
    {
      "name": "initRecentListings",
      "docs": [
        "INIT"
      ],
      "accounts": [
        {
          "name": "physicalRecentListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalRecentListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "commissionRecentListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initCommissionsListings",
      "docs": [
        "VENDOR LISTINGS UTILS"
      ],
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initDigitalListings",
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initPhysicalListings",
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "listCommissionProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "prod",
          "type": {
            "defined": "OrbitProductStruct"
          }
        }
      ]
    },
    {
      "name": "unlistCommissionProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "listDigitalProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "prod",
          "type": {
            "defined": "OrbitProductStruct"
          }
        },
        {
          "name": "fileType",
          "type": {
            "defined": "DigitalFileTypes"
          }
        }
      ]
    },
    {
      "name": "unlistDigitalProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "listPhysicalProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "prod",
          "type": {
            "defined": "OrbitProductStruct"
          }
        },
        {
          "name": "quantity",
          "type": "u32"
        }
      ]
    },
    {
      "name": "unlistPhysicalProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "transferVendorListingsOwnership",
      "docs": [
        "OWNERSHIP TRANSFER"
      ],
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "transferStruct",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "transferAllVendorListingsOwnership",
      "accounts": [
        {
          "name": "physicalVendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalVendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "commissionVendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "transferStruct",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateProductQuantity",
      "docs": [
        "PHYSICAL"
      ],
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "qnt",
          "type": "u32"
        }
      ]
    },
    {
      "name": "updateProductQuantityInternal",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "qnt",
          "type": "u32"
        }
      ]
    },
    {
      "name": "physicalIncrementTimesSold",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "physicalUpdateInfo",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "info",
          "type": "string"
        }
      ]
    },
    {
      "name": "physicalUpdatePrice",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "physicalUpdateDeliveryEstimate",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "deliveryEstimate",
          "type": "u8"
        }
      ]
    },
    {
      "name": "physicalUpdateMedia",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "link",
          "type": "string"
        }
      ]
    },
    {
      "name": "physicalMarkAvailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "physicalMarkUnavailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "markPhysicalSearchable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "setFileType",
      "docs": [
        "DIGITAL"
      ],
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "fileType",
          "type": {
            "defined": "DigitalFileTypes"
          }
        }
      ]
    },
    {
      "name": "digitalIncrementTimesSold",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "digitalUpdateInfo",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "info",
          "type": "string"
        }
      ]
    },
    {
      "name": "digitalUpdatePrice",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "digitalUpdateDeliveryEstimate",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "deliveryEstimate",
          "type": "u8"
        }
      ]
    },
    {
      "name": "digitalUpdateMedia",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "link",
          "type": "string"
        }
      ]
    },
    {
      "name": "digitalMarkAvailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "digitalMarkUnavailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "markDigitalSearchable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "commissionIncrementTimesSold",
      "docs": [
        "COMMISSIONS"
      ],
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "commissionUpdateInfo",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "info",
          "type": "string"
        }
      ]
    },
    {
      "name": "commissionUpdatePrice",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "commissionUpdateDeliveryEstimate",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "deliveryEstimate",
          "type": "u8"
        }
      ]
    },
    {
      "name": "commissionUpdateMedia",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "link",
          "type": "string"
        }
      ]
    },
    {
      "name": "commissionMarkAvailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "commissionMarkUnavailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "markCommissionSearchable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "flushListings",
      "docs": [
        "FLUSH LISTINGS"
      ],
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "listingsStruct",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "listingsOwner",
            "type": "publicKey"
          },
          {
            "name": "addressAvailable",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "productAvailable",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          }
        ]
      }
    },
    {
      "name": "digitalProduct",
      "docs": [
        "DIGITAL"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": {
              "defined": "OrbitProductStruct"
            }
          },
          {
            "name": "digitalFileType",
            "type": {
              "defined": "DigitalFileTypes"
            }
          }
        ]
      }
    },
    {
      "name": "physicalProduct",
      "docs": [
        "PHYSICAL"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": {
              "defined": "OrbitProductStruct"
            }
          },
          {
            "name": "quantity",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "commissionProduct",
      "docs": [
        "COMMISSION"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": {
              "defined": "OrbitProductStruct"
            }
          }
        ]
      }
    },
    {
      "name": "recentMarketListings",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "marketType",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "pubkeys",
            "type": {
              "array": [
                "publicKey",
                25
              ]
            }
          },
          {
            "name": "index",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "OrbitProductStruct",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "info",
            "type": "string"
          },
          {
            "name": "ownerCatalog",
            "type": "u64"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "deliveryEstimate",
            "type": "u8"
          },
          {
            "name": "media",
            "type": "string"
          },
          {
            "name": "timesSold",
            "type": "u32"
          },
          {
            "name": "searchIndexed",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "ListingsType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Commissions"
          },
          {
            "name": "Digital"
          },
          {
            "name": "Physical"
          }
        ]
      }
    },
    {
      "name": "DigitalFileTypes",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Text"
          },
          {
            "name": "Video"
          },
          {
            "name": "Audio"
          },
          {
            "name": "Image"
          },
          {
            "name": "Folder"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "AddressUnavailable",
      "msg": "requested PDA is unavailable"
    },
    {
      "code": 6001,
      "name": "InvalidCatalogType",
      "msg": "given address is not a recent catalog"
    },
    {
      "code": 6002,
      "name": "InvalidParameter",
      "msg": "invalid parameter"
    }
  ]
};

export const IDL: OrbitProduct = {
  "version": "0.1.0",
  "name": "orbit_product",
  "instructions": [
    {
      "name": "initRecentListings",
      "docs": [
        "INIT"
      ],
      "accounts": [
        {
          "name": "physicalRecentListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalRecentListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "commissionRecentListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initCommissionsListings",
      "docs": [
        "VENDOR LISTINGS UTILS"
      ],
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initDigitalListings",
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initPhysicalListings",
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "productProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "accountsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "listCommissionProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "prod",
          "type": {
            "defined": "OrbitProductStruct"
          }
        }
      ]
    },
    {
      "name": "unlistCommissionProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "listDigitalProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "prod",
          "type": {
            "defined": "OrbitProductStruct"
          }
        },
        {
          "name": "fileType",
          "type": {
            "defined": "DigitalFileTypes"
          }
        }
      ]
    },
    {
      "name": "unlistDigitalProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "listPhysicalProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "prod",
          "type": {
            "defined": "OrbitProductStruct"
          }
        },
        {
          "name": "quantity",
          "type": "u32"
        }
      ]
    },
    {
      "name": "unlistPhysicalProduct",
      "accounts": [
        {
          "name": "prod",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "transferVendorListingsOwnership",
      "docs": [
        "OWNERSHIP TRANSFER"
      ],
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "transferStruct",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "transferAllVendorListingsOwnership",
      "accounts": [
        {
          "name": "physicalVendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "digitalVendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "commissionVendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "transferStruct",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateProductQuantity",
      "docs": [
        "PHYSICAL"
      ],
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "qnt",
          "type": "u32"
        }
      ]
    },
    {
      "name": "updateProductQuantityInternal",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "qnt",
          "type": "u32"
        }
      ]
    },
    {
      "name": "physicalIncrementTimesSold",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "physicalUpdateInfo",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "info",
          "type": "string"
        }
      ]
    },
    {
      "name": "physicalUpdatePrice",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "physicalUpdateDeliveryEstimate",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "deliveryEstimate",
          "type": "u8"
        }
      ]
    },
    {
      "name": "physicalUpdateMedia",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "link",
          "type": "string"
        }
      ]
    },
    {
      "name": "physicalMarkAvailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "physicalMarkUnavailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "markPhysicalSearchable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "setFileType",
      "docs": [
        "DIGITAL"
      ],
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "fileType",
          "type": {
            "defined": "DigitalFileTypes"
          }
        }
      ]
    },
    {
      "name": "digitalIncrementTimesSold",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "digitalUpdateInfo",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "info",
          "type": "string"
        }
      ]
    },
    {
      "name": "digitalUpdatePrice",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "digitalUpdateDeliveryEstimate",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "deliveryEstimate",
          "type": "u8"
        }
      ]
    },
    {
      "name": "digitalUpdateMedia",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "link",
          "type": "string"
        }
      ]
    },
    {
      "name": "digitalMarkAvailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "digitalMarkUnavailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "markDigitalSearchable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "commissionIncrementTimesSold",
      "docs": [
        "COMMISSIONS"
      ],
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "commissionUpdateInfo",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "info",
          "type": "string"
        }
      ]
    },
    {
      "name": "commissionUpdatePrice",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "commissionUpdateDeliveryEstimate",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "deliveryEstimate",
          "type": "u8"
        }
      ]
    },
    {
      "name": "commissionUpdateMedia",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "link",
          "type": "string"
        }
      ]
    },
    {
      "name": "commissionMarkAvailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "commissionMarkUnavailable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "markCommissionSearchable",
      "accounts": [
        {
          "name": "product",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vendorAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "flushListings",
      "docs": [
        "FLUSH LISTINGS"
      ],
      "accounts": [
        {
          "name": "vendorListings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "listingsStruct",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "listingsOwner",
            "type": "publicKey"
          },
          {
            "name": "addressAvailable",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "productAvailable",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          }
        ]
      }
    },
    {
      "name": "digitalProduct",
      "docs": [
        "DIGITAL"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": {
              "defined": "OrbitProductStruct"
            }
          },
          {
            "name": "digitalFileType",
            "type": {
              "defined": "DigitalFileTypes"
            }
          }
        ]
      }
    },
    {
      "name": "physicalProduct",
      "docs": [
        "PHYSICAL"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": {
              "defined": "OrbitProductStruct"
            }
          },
          {
            "name": "quantity",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "commissionProduct",
      "docs": [
        "COMMISSION"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "metadata",
            "type": {
              "defined": "OrbitProductStruct"
            }
          }
        ]
      }
    },
    {
      "name": "recentMarketListings",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "marketType",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "pubkeys",
            "type": {
              "array": [
                "publicKey",
                25
              ]
            }
          },
          {
            "name": "index",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "OrbitProductStruct",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "info",
            "type": "string"
          },
          {
            "name": "ownerCatalog",
            "type": "u64"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "deliveryEstimate",
            "type": "u8"
          },
          {
            "name": "media",
            "type": "string"
          },
          {
            "name": "timesSold",
            "type": "u32"
          },
          {
            "name": "searchIndexed",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "ListingsType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Commissions"
          },
          {
            "name": "Digital"
          },
          {
            "name": "Physical"
          }
        ]
      }
    },
    {
      "name": "DigitalFileTypes",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Text"
          },
          {
            "name": "Video"
          },
          {
            "name": "Audio"
          },
          {
            "name": "Image"
          },
          {
            "name": "Folder"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "AddressUnavailable",
      "msg": "requested PDA is unavailable"
    },
    {
      "code": 6001,
      "name": "InvalidCatalogType",
      "msg": "given address is not a recent catalog"
    },
    {
      "code": 6002,
      "name": "InvalidParameter",
      "msg": "invalid parameter"
    }
  ]
};
