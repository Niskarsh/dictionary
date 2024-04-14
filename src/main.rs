use dictionary::Node;
fn main() {
    let words = ["the", "this", "apple"];
    let mut trie = Node::new();
    for word in words {
        let characterArray = word.chars().collect::<Vec<_>>();
        trie.addWord(&characterArray);
    }
    println!("Trie is: {:?}", trie);
}
