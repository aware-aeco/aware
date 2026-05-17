# core-utils-get-identifier-except-method-name

Lifecycle: single

Retrieves the string format of the identifier list from left to right, leaving out any symbols after the last identifier.             Given: A.B()                 Return: "A"             Given: A.B.C()[0]                 Return: "A.B"             Given: A.B().C                 Return: "A"             Given: A.B[0].C                 Return: "A.B[0].C"             Given: A().B (global function)                 Return: empty string             Given: A.B[0].C()                 Return: "A.B[0]"
