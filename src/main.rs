use dictionary::Node;
fn main() {
    let words = ["the", "this"];
    let mut trie = Node::new();
    for word in words {
        let characterArray = word.chars().collect::<Vec<_>>();
        trie.add(&characterArray);
    }
    println!("Trie is: {:?}", trie);
}
