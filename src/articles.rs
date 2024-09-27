pub struct Tweet {
    title: String,
    content: String,
    author: String,
}
pub struct Article {
    title: String,
    content: String,
}

pub struct A {
    title: String,
    content: String,
}
pub trait CustomDisplay {
    fn display(&self);
}

impl CustomDisplay for Tweet {
    fn display(&self) {
        let s = format!("author:{}, title:{}, content:{}", self.author, self.title, self.content);
        println!("{}", s);
    }
}

impl CustomDisplay for Article {
    fn display(&self) {
        let s = format!("no author.title:{}, content:{}", self.title, self.content);
        println!("{}", s);
    }
}
impl Tweet {
    pub fn new() -> Tweet {
        Tweet {
            title: String::from("A"),
            content: String::from("B"),
            author: String::from("C"),
        }
    }
}
impl Article {
    pub fn new() -> Article {
        Article {
            title: String::from("X"),
            content: String::from("Y"),
        }
    }
}

impl A {
    pub fn new() -> A {
        A {
            content: String::from("1"),
            title: String::from("2"),
        }
    }
}