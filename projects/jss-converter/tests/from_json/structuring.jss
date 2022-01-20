{
    "$id": "https://example.com/schemas/customer",
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "properties": {
        "first_name": {
            "type": "string"
        },
        "last_name": {
            "type": "string"
        },
        "shipping_address": {
            "$ref": "/schemas/address"
        },
        "billing_address": {
            "$ref": "/schemas/address"
        }
    },
    "required": [
        "first_name",
        "last_name",
        "shipping_address",
        "billing_address"
    ],
    "$defs": {
        "address": {
            "$id": "/schemas/address",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "street_address": {
                    "type": "string"
                },
                "city": {
                    "type": "string"
                },
                "state": {
                    "$ref": "#/definitions/state"
                }
            },
            "required": [
                "street_address",
                "city",
                "state"
            ],
            "definitions": {
                "state": {
                    "enum": [
                        "CA",
                        "NY",
                        "... etc ..."
                    ]
                }
            }
        }
    }
}