// use std::fmt::Display;
// fn Maybe<A, B>(f: fn(A) -> B) -> impl Fn(Option<A>) -> Option<B> {
//     return |a: Option<A>| match a {
//         Some(x) => Some(f(x)),
//         None => None,
//     };
// }
// fn print_list<T: Display + Clone>(vec: &Vec<T>) {
//     match vec[..] {
//         [] => {}
//
//         [a, ref b @ ..] => {
//             println!("{}", a);
//             print_list(&b.to_vec());
//         }
//     }
// }
