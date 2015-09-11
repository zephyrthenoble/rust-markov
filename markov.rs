#[derive(Debug)]
pub struct Root {
    next: Option<Vec<Node>>,
    total: u32,
}
impl Root{
    pub fn new() -> Root {
        Root{next: None, total:0}
    }
    pub fn add(&mut self, w: String) {
        self.total += 1;
        match self.next {
            None => {
                let mut vec: Vec<Node> = Vec::new();
                let n = Node::from_string(w);
                vec.push(n);
                self.next = Some(vec);
            }
            Some(ref mut vec) => {
                let mut flag = false;
                //search for word
                for x in vec.iter_mut() {
                    //if we find the word, add one to part and total
                    if x.word.value == w {
                        x.word.part = x.word.part + 1.0;
                        flag = true;
                    }
                }
                // if the word wasn't in there, create and add it
                if !flag
                {
                    let n = Node::from_string(w);
                    vec.push(n);
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct Node {
    pub word: Word,
    next: Option<Vec<Node>>,
    total: u32,
}

impl Node {
    pub fn new(w: Word) -> Node {
        Node{word:w, next: None, total:1}
    }
    pub fn from_string(s: String) -> Node {
        Node{word:Word::from_string(s), next: None, total:1}
    }
    pub fn is_leaf(&self) -> bool {
        match self.next {
            None => return true,
            Some(ref vec)=> return vec.len() == 0,
        }
    }
    pub fn add(&mut self, w: String) {
        self.total += 1;
        match self.next {
            None => {
                let mut vec: Vec<Node> = Vec::new();
                let n = Node::from_string(w);
                vec.push(n);
                self.next = Some(vec);
            }
            Some(ref mut vec) => {
                let mut flag = false;
                //search for word
                for x in vec.iter_mut() {
                    //if we find the word, add one to part and total
                    if x.word.value == w {
                        x.word.part = x.word.part + 1.0;
                        flag = true;
                    }
                }
                // if the word wasn't in there, create and add it
                if !flag
                {
                    let n = Node::from_string(w);
                    vec.push(n);
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct Word {
    pub value: String,
    pub part:  f32
}
impl Word {
    pub fn from_string(value: String) -> Word {
        Word{ value:value, part:1.0 }
    }
    pub fn new(value: &'static str) -> Word {
        Word{ value:String::from(value), part:1.0}
    }
    pub fn prob(&self, total: f32) -> f32 {
        return self.part/total;
    }
}

fn main() {
    let mut r = Root::new();
    r.add("test".to_string());
    r.add("test".to_string());
    r.add("post".to_string());
    println!("{:?}", r);
}
