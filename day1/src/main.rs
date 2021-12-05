mod soln;

/*
   BROKEN: Why does this not compile?
*/
// pub fn main() {
//     println!("Hello World");
//     let contents: &str = include_str!("../day1.txt");
//     let soln = soln::Soln1;
//     soln.parse(contents);
// }

// reproducible case
fn main() {
    trait Woofer {
        fn bark(tree: String) -> String;
    }

    struct Dog {}
    impl Dog {
        fn constant_bark() -> String {
            "grrrr".to_string()
        }
    }

    impl Woofer for Dog {
        fn bark(tree: String) -> String {
            Self::constant_bark()
        }
    }

    let d = Dog {};
    d.bark("pine");
}
