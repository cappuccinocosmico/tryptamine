Go ahead and check out all the code for generating fratals in:
/home/nicole/Documents/mycorrhizae/tryptamine/mathematics-rust/src/math/fractals.rs

Now write a cli tool using clap that will generate these fractal images in:

/home/nicole/Documents/mycorrhizae/tryptamine/mathematics-rust/src/main.rs

This cli should only activate in this mode when given the fractal keyword, and after this it should be the word of the fractal to be generated. For example to generate a test regular julia set it would be  

./tryptamine-math fractal julia

which would generate the default julia set at a resolution of 1000.

and the same would be true for 

./tryptamine-math fractal sinjulia


could you also add paramaters that would let me set the resolution, image type and output path?

Before you finish your task run ` RUSTFLAGS="-A warnings" cargo check --message-format=short` (Some optimisations to weed out a bunch of unneded tokens) to make sure you havent made any mistakes. Also try to avoid modifying any code that isnt absolutely essential to implement your feature.


