# attested-img-editor

This attested image editing library uses ZK to apply transformations on a given authentic image, built using SP1. Pass in an image with a sequence of transformations that you want to do on that image, defined in `Transformation` format. Get back an image with those transformations applied without revealing the original image or the transformations done on the image. We also verify the given image ECDSA signature which verifies that the given image is real (i.e. signed by camera hardware, not AI-generated).

**Inputs (private)**: The original image (in bytes), signature, desired transformations, public key.
**Outputs (public)**: ZK proof, edited image (in bytes format -- which our script decodes to an image file).

## Supported Image Transformations
- crop
- rotate (90, 180, 270)
- flip horizontal
- flip vertical
- brighten
- contrast

-------

### Motivations

In today's digital world, it's hard to distinguish between real and fake content. We need a way to certify if something came from a real camera that can't be forged. We verify that images are authentic using the magic of zero-knowledge cryptography and blockchains.

In the past, trust in the digital world was simple: a picture was a snapshot of reality. Today, that's no longer true. Generative AI is completely disrupting our relationship with authentic content. In today's digital world, anything is possible: from dogs hosting podcasts on mountains and the pope wearing puffers. Just this past week, we've seen the hyper realistic videos Sora is capable of producing; people are already falling victim to deepfakes. Soon, it will be impossible to distinguish what is reality and what is fiction.

It's time to separate what's real from what's fake. We need a solution that is practical, uncensorable, and unforgeable. That's where the the power of zero-knowledge and blockchains come in. Cryptography is the only way to ensure that the content we see in the digital world is real. We want to live in a future rooted in truth, not trust.

### Limitations

Because the image transformations are expensive, proof generation takes quite a long time and can only support small images (benchmarked with 256x256 sized images).

### Acknowledgements 

We built this project as part of Stanford TreeHacks. Thank you to Succinct Labs for their excellent work on SP1, as well as sponsors QED protocol and Caldera for their support.
