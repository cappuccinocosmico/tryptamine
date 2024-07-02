const std = @import("std");
const fs = std.fs;

pub fn main() void {

    // Image
    const image_width: i32 = 256;
    const image_height: i32 = 256;

    // Render
    std.debug.print("P3\n{} {}\n255\n", .{ image_width, image_height });
    for (0..image_height) |j| {
        for (0..image_width) |i| {
            const r: f64 = @as(f64, @floatFromInt(i)) / (image_width - 1);
            const g: f64 = @as(f64, @floatFromInt(j)) / (image_height - 1);
            const b: f64 = 0.0;
            const ir = @as(i32, @intFromFloat(r * 255.999));
            const ig = @as(i32, @intFromFloat(g * 255.999));
            const ib = @as(i32, @intFromFloat(b * 255.999));
            std.debug.print("{} {} {}\n", .{ ir, ig, ib });
        }
    }
}

// #include <iostream>
//
// int main() {
//
//     // Image
//
//     int image_width = 256;
//     int image_height = 256;
//
//     // Render
//
//     std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";
//
//     for (int j = 0; j < image_height; j++) {
//         for (int i = 0; i < image_width; i++) {
//             auto r = double(i) / (image_width-1);
//             auto g = double(j) / (image_height-1);
//             auto b = 0.0;
//
//             int ir = int(255.999 * r);
//             int ig = int(255.999 * g);
//             int ib = int(255.999 * b);
//
//             std::cout << ir << ' ' << ig << ' ' << ib << '\n';
//         }
//     }
// }
