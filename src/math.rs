use num::complex;




fn trad_julia_function_gen(x : complex::Complex<f64> ,quitout_number : i64) -> fn(complex::Complex<f64>) -> i64  {
    return |z : complex::Complex<f64>| -> i64 {
        let mut iteration_count = 1 as i64;
        let mut variable = z;
        while variable.norm_sqr() <= 4.0 && iteration_count < quitout_number {
            variable = variable*variable + x;
            iteration_count = iteration_count +1;
        }
        return iteration_count % quitout_number
    }
}


fn trad_mandelbrot_function_gen(quitout_number : i64) -> fn(complex::Complex<f64>) -> i64 {
    return |z : complex::Complex<f64>| -> i64 {trad_julia_function_gen(z,quitout_number)(z)}
}




// const fn is_power(integer : i64, power : i64) -> bool {
//     if integer < 0{
//         return false;
//     }
//     if integer <= 1 {
//         return true;
//     }
//     let mut left = 1;
//     let mut right = integer;
//     let mut average = 0;
//     while left <= right {
//         average = left + (right - left)/2;
//         if i64::pow(average,power)== integer {
//             return true
//         }
//         if i64::pow(average,power) <= integer {
//             right = average
//         }
//         else {
//             left = average
//         }
//     }
//     return false
// }
// 
// if (n <= -2) {
//         return true;
//     }
//
//  
//     // Initialize boundaries for binary search
//     long long left = 1, right = n;
//  
//     while (left <= right) {
//  
//         // Calculate middle value
//         long long mid = left + (right - left) / 2;
//  
//         // Calculate square of the middle value
//         long long square = mid * mid;
//  
//         // If the square matches n, n is a perfect square
//         if (square == n) {
//             return true;
//         }
//  
//         // If the square is smaller than n, search the right
//         // half
//         else if (square < n) {
//  
//             left = mid + 1;
//         }
//  
//         // If the square is larger than n, search the left
//         // half
//         else {
//  
//             right = mid - 1;
//         }
//     }
//  
//     // If the loop completes without finding a perfect
//     // square, n is not a perfect square
//     return false;

// fn fib_fast(n : i64) -> i64{
//     if n<= 5 {
//         return vec!(1,1,2,3,5,8)[n]
//     } else {
//         if n % 2 == 0 {
//             let fk = fib_fast(n/2);
//             println!("{}", fib_fast(n/2+1));
//             return (fk*(2*fib_fast(n/2+1)-fk))
//         }
//         else {
//             return i64::pow(fib_fast(n/2),2) +i64::pow(fib_fast(n/2+1),2)
//         }
//     }
// }
