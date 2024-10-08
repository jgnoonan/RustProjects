/*
What are Hashmaps in Rust?

Rust's standard collection library has implemented some of the most common
data structures for general purposes.  We've seen vectors previously, now 
let's take a look at hashmaps.

The type HashMap<K, V> stores a mapping of keys of type K to values of type V.
It does this via a hashing function, which determines how it places these keys
and values into memory.  Many different programming languages support this kind
of data structure, but often use a different name, such as hash, map, object, 
hash table, or associative array, just to name a few.

Hash maps are useful for when you want to look up data not by an index, as you
can with vectors, but by using a key that can be of any type.  For example, in a
game, you could keep track of each team's score in a hash map where each key is
the team's name, and the values are each team's score.  Given a team name, you 
can retrieve its score.

No two entries in a map can have the same key.  In short, a map is a lookup table.

1.  insert()
pub fn insert(&mut self, k: K, v: V) -> Option
Inserts a key/value pair into the hashmap

2.  len()
pub fn len(&self) -> usize
Finds and returns how many elements are in the map

3.  get()
pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V> where K:Borrow and Q:Hash + Eq
Returns a reference to a value of a key

4.  iter()
pub fn iter(&self) -> Iter<K, V>
Iteration through all key/value pairs

5.  contains_key()
pub fn contains_key<Q: ?Sized>(mut& self, k: &Q) -> bool
returns true if a value is true for a specified key

6.  remove()
pub fn remove<Q>(&mut self, k: &Q) -> Option<V> where K: Borrow<Q>, and Q: Hash + Eq + ?Sized
Removes a key from the map, returning the value at the key if the key was present in the map

*/

/*
Exercise - Build a hash table with Hashmap in Rust

1.  Create a new HashMap instance variable barDrinks and using the insert method
    add three new key/value pairs to your table: vodka, beer, and whiskey as keys
    and the values should be set so that vodka and whiskey indicate the bar has some
    left, while beer has run out.
2.  Remove the whiskey from the local table

*/

// HashMap Example
use std::collections::HashMap;

fn main() {
    
    let mut bar_drinks = HashMap::new();
    bar_drinks.insert("Vodka", true);
    bar_drinks.insert("Whiskey", true);
    bar_drinks.insert("Beer", false);

    println!("The size of the hashmap is: {}", bar_drinks.len());
    println!("Current list: {:?}", bar_drinks);
    
    bar_drinks.remove("Whiskey");

    println!("The size of the hashmap is: {}", bar_drinks.len());
    println!("Current list: {:?}", bar_drinks);
}
