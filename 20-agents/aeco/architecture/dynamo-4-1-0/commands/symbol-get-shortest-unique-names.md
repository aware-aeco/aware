# symbol-get-shortest-unique-names

Lifecycle: single

Given a list of conflicting namespaces, finds the shortest partial name for a namespace             that can be resolved uniquely. For example, given {"A.B.C.D.E", "X.Y.A.B.E.C.E", "X.Y.A.C.B.E"},             all with the same class E, their shortest unique names would be: {"D.E", "E.E", "C.B.E"}
