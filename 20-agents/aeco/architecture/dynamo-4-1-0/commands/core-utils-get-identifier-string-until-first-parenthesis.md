# core-utils-get-identifier-string-until-first-parenthesis

Lifecycle: single

Retrieves the string format of the identifier list from left to right, leaving out any symbols after the last identifier.             Given: A.B()                 Return: "A.B"             Given: A.B.C()[0]                 Return: "A.B.C"             Given: A.B().C                 Return: "A.B"             Given: A.B[0].C                 Return: "A.B[0].C"
