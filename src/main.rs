use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Tree{
score : u64,
label : String,
left : Option<Box<Tree>>,
right : Option<Box<Tree>>,
}
impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the order to prioritize lower scores
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Tree {}

fn frequencies(payload : String,mut memo : HashMap<char, u64>) -> HashMap<char,u64>{
for chara in payload.chars(){
*memo.entry(chara).or_insert(0)+=1;
}
return memo ;
}
fn main(){
let text = "Huffman coding is a lossless data compression algorithm. It is used to compress data in a way that reduces the overall size of the data without losing any information.";
let payload  = String::from(text);
let memo :HashMap<char, u64> = HashMap::new();    
let freq = frequencies(payload, memo);
let mut min_heap : BinaryHeap<Tree> = BinaryHeap::new();
//add all elements to the current BinaryHeap
for (u,v) in &freq{
let current_Tree = Tree{
score : *v,
label : String::from(*u),
left : None,
right : None
};

min_heap.push(current_Tree)
}
// at this point we are ready to start the algo 
// while we have more than one element >=2 in BinaryHeap 
// we pop two we addup their score 
// create a new parent node with the concatenation of their label and has this just calculated
// score 
// add it to BinaryHeap
while min_heap.len()>=2{
let node1 = min_heap.pop().unwrap();
let node2 = min_heap.pop().unwrap();
let newScore = node2.score+node1.score;
let mut newLabel = node1.label.clone();
let oldLabel = node2.label.as_str();
newLabel.push_str(oldLabel);
let newNode = Tree{
score : newScore,
label : newLabel,
left : Some(Box::new(node1)),
right : Some(Box::new(node2)) 
};

min_heap.push(newNode);
}
let head = min_heap.pop().unwrap();

let mut codes : HashMap<String,String> = HashMap::new();

let code = String::new(); 
findCharacter(head, &mut codes, code);
let mut size_original = 0;
let mut size_compressed = 0;

for (u,v) in codes{
size_compressed += (v.len() as u64)*freq[&u.chars().next().unwrap()];
size_original+=freq[&u.chars().next().unwrap()]*8;

}

println!("The size of the original is : {} the size_compressed one {}",size_original,size_compressed);

}
fn findCharacter(head : Tree ,codes :  &mut HashMap<String,String>,mut current_code :  String){
if let None = head.left{
codes.entry(head.label).or_insert(current_code );
return;
}else{
let mut copy = current_code.clone(); 

current_code.push('0');
findCharacter(*head.left.unwrap() ,codes, current_code);
copy.push('1');
findCharacter(*head.right.unwrap() ,codes, copy);
}





}











