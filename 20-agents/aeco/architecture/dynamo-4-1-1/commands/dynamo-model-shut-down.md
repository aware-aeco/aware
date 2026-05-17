# dynamo-model-shut-down

Lifecycle: single

External components call this method to shutdown DynamoModel. This             method marks 'ShutdownRequested' property to 'true'. This method is             used rather than a public virtual method to ensure that the value of             ShutdownRequested is set to true.
