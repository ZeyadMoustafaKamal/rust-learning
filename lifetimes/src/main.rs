fn main() {
    /*
    fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let s = "Hello world";
    let s2 = "Longer hello world";
    println!("The longest is '{}'", get_longest(s, s2));
    */
    /*
    struct ImportantExcerpt<'a> {
        part: &'a str
    }
    impl<'a> ImportantExcerpt<'a> {
        fn return_part(&self, announcement: &str) -> &str {
            println!("Attention please {}", announcement);
            self.part
        }
    }
    let novel = "Callme Ishmael. Some years ago...".to_owned();
    let first_sentence = novel.split('.').next().expect("Couldn't find anything here");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    */
    use std::fmt::Display;
    fn longest_with_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        announcement: T
    ) -> &'a str
        where T: Display,
    {
        println!("accouncement: {}", announcement);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

}


