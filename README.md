# Mandelbrot renderer
Mandelbrot set rendering engine. Written in rust (2018+)

# What is a complex number?
A complex number is a number in the form of: ``a + bi`` where we have a real part and an imaginary part. The imaginary part is: ``bi``, ``i`` is defined as the square root of -1, which is an imaginary number, but replacing the value of ``sqrt(-1)`` with ``i`` allows us to utilize a whole new number set called the complex number set. The mandelbrot set is a set of complex numbers that hold coordinates on a complex plane.

# What is the mandelbrot set? 
The mandelbrot set is a set of complex numbers generated from a function defined as: ``f(z) = z^2 + c`` where ``z`` starts iterating from ``0``. ``c`` is our complex number. The only produced complex numbers we will store in the set will be ones that constrain to some bound, if the produced complex number is in this bound we will draw it, if it's bounded towards infinity we will draw it as some background color.

# Side notes:
My approach is pretty naive but I implement data parallelism using parallel iterators, there are several optimzations to be considered. Some of which are:
  * SIMD optimizations: We can utilize SIMD registers and data types so we can process multiple pixels at once.
  * Symmetric drawing: The mandelbrot set rendering is actually symmetrical along the x-axis. So we can compute the top half of the set, and render the bottom half.
  * Algebraic simplification: In the naive algoritm approach we preform several operations, but with some algebraic simplification of the numbers we can simply reduce it.
  * GPU processing: We can use libraries such as OpenCL to put the computational power of our algorithm on the GPU instead of the CPU.

# What does it look like?
![Mandelbrot_renderer](https://github.com/LLayta/Mandelbrot_renderer/blob/main/images/mandelbrot.png)
