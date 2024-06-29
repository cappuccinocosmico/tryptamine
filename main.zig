const std = @import("std");
const fs = std.fs;
const Managed = std.math.big.int.Managed;


fn map(comptime Src: type, comptime Dst: type, comptime len: usize, array: [len]Src, function: fn(Src) Dst) [len]Dst {
    var result: [len]Dst = undefined;
    for (result) |*res ,index| {
        res.* = function(array[index]);
    }
    return result;
}
const Complex = struct {
    real: f64,
    imag: f64,

    fn magnitude(self: Complex) f64 {
        return std.math.sqrt(self.real * self.real + self.imag * self.imag);
    }
    fn magnitude2(self: Complex) f64 {
        return self.real * self.real + self.imag * self.imag;
    }

    fn square(self: Complex) Complex {
        return Complex{
            .real = self.real * self.real - self.imag * self.imag,
            .imag = 2.0 * self.real * self.imag,
        };
    }

    fn add(self: Complex, other: Complex) Complex {
        return Complex{
            .real = self.real + other.real,
            .imag = self.imag + other.imag,
        };
    }
};

fn mandelbrot_iterations(c: Complex, bailout: usize) usize {
    var z = Complex{ .real = 0.0, .imag = 0.0 };
    const uzero: usize = 0;
    for (uzero..bailout) |i| {
        z = z.square().add(c);
        if (z.magnitude2() > 4.0) {
            return i;
        }
    }
    return bailout;
}

fn fib_fast(n : i64) i64 {
    if n <= 7 {
        return [1,1,2,3,5,8,13,21][n]
    }
    fhalf = std.math.pow(fib_fast(n/2+1),2)
    fhalfplusone = std.math.pow(fib_fast(n/2+1),2)
    if (n % 2 ==1) {
        return std.math.pow(fhalfplusone,2)+std.math.pow(fhalf,2)
    }
    fhalf = 
}

fn miller_rabin_primality(n : I)


pub fn main() void {
    const c = Complex{ .real = -0.5, .imag = 0.27015 };
    const bailout = 100000;
    const iterations = mandelbrot_iterations(c, bailout);
    std.debug.print("Iterations: {}\n", .{iterations});
}
