# replicator-compute-all-reduced-params

Lifecycle: single

For each parameter, if there is a replication instruction for it, and             if it is an array, expand parameter list based on the types of elements             in that array. For example, for parameters                              {p1, p2, ..., pk, ..., pn} where pk is an array                                   {a1:int, a2:string, a3:double, ...}                           and there is a Cartesian replication on pk, the parameter list will be             expanded to                              {p1, p2, ..., a1, ..., pn}                 {p1, p2, ..., a2, ..., pn}                 {p1, p2, ..., a3, ..., pn}                 ...
