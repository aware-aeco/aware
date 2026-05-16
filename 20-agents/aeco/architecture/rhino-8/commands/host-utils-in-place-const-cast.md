# host-utils-in-place-const-cast

Lifecycle: single

DO NOT USE UNLESS YOU ARE CERTAIN ABOUT THE IMPLICATIONS.              This is an expert user function which should not be needed in most              cases. This function is similar to a const_cast in C++ to allow an object              to be made temporarily modifiable without causing RhinoCommon to convert              the class from const to non-const by creating a duplicate.You must call this function with a true parameter, make your              modifications, and then restore the const flag by calling this function              again with a false parameter. If you have any questions, please              contact McNeel developer support before using!
