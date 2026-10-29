# direct-shape-library-add-definition-type

Lifecycle: single

Add a definition to be reused by instances. Adding a definition type will change how the instances are created.    When asked to create a definition, the library object will look for a corresponding type object.    If one is found, it will create an instance of geometry stored in the type object. If it is not found,    the library will look for a list of geometry objects stored as definition, and will copy and transform these    to create an instance.
