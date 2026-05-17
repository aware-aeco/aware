# catalog-item-get-image

Lifecycle: single

Returns the Image object. The object's width in pixels is specified by the SIZE.cx member of size. The height member of size is ignored.  If an image of the specified size is not found, the image with the next larger size is returned. If a larger-sized image is not available, then the image with the next smaller size is returned.  Returns null if no image is associated with the item.
