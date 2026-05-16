# annotation-base-set-override-dim-style

Lifecycle: single

Set a style including overrides for this annotation object.             The DimensionStyle OverrideStyle must have the override fields marked              as overridden and must have it's Id set to nil.             Use DimensinoStyle.SetFieldOverride(Field field) and related functions             to manage override settings. To override a field, the field value must be set             and the field must be marked as an override.              The DimensionStyle passed in here must not be in the dimstyle table
