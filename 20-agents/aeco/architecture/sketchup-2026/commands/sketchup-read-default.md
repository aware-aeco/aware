# sketchup-read-default

Lifecycle: single

Be aware that the method is not capable of handling Length objects. You can convert the value to a Float before writing and convert back to Length when reading the value. Don't store the value as a String as this rounds the value and formats it in a way that can't be read if the system setting for decimal separator changes.
