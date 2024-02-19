# attested-img-editor
pass in an image with a sequence of transformations that you want to do on that image, defined in `Transformation` format.
Using the power of ZK, get back an image with those transformations applied without revealing the original image or the transformations done on the image. We also verify the given image signature using ECDSA signature verification.

Inputs (private): The original image (in bytes), signature, desired transformations, width, height.
Outputs (public): ZK proof, edited image (in bytes format -- which our script decodes to an image file).

## Supported Image Transformations
- crop
- rotate (90, 180, 270)
- flip horizontal
- flip vertical
- brighten
- contrast
