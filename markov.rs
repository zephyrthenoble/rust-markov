pub struct Node {
    pub word: Word,
    next: Option<Vec<Node>>,
    total: u32,
}

impl Node {
    pub fn new(w: Word) {
        Node{word:w, next: None, total:0}
    }
    pub fn add(&mut self, w: String) {
        self.total += 1;
        match self.next {
            None => {
                let vec = Vec::<Node>new();
                let n = Node::new(Word::fromString(w));
                vec.push(n);
                self.next = Some(vec);
            }
            Some(vec) => {
                let mut flag = false;
                //search for word
                for x in &vec {
                    //if we find the word, add one to part and total
                    if(x.word.value == w){
                        x.word.part = x.word.part + 1.0;
                        flag = true;
                    }
                }
                // if the word wasn't in there, create and add it
                if(!flag)
                {
                    let word = Word::fromString(w);
                }
            }
        }
    }
}

pub struct Word {
    pub value: String,
    pub part:  f32
}

impl Word {
    pub fn fromString(value: String) -> Word {
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
    let w = Word::new("test");
}
