var gSchemaDefinitions = {
    "Elements": {
        "type": "array",
        "description": "A list of elements.",
        "items": {
            "$ref": "#/ElementIdArrayItem"
        }
    },
    "ElementIdArrayItem": {
        "type": "object",
        "properties": {
            "elementId": {
                "$ref": "#/ElementId"
            }
        },
        "additionalProperties": false,
        "required": [
            "elementId"
        ]
    },
    "ElementId": {
        "type": "object",
        "description": "The identifier of an element.",
        "properties": {
            "guid": {
                "$ref": "#/Guid"
            }
        },
        "additionalProperties": false,
        "required": [
            "guid"
        ]
    },
    "SurfaceType": {
        "type": "string",
        "description": "The type of a surface material.",
        "enum": [
            "General",
            "Simple",
            "Matte",
            "Metal"
        ]
    }
}
;
