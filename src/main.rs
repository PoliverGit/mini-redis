mod store;
use store::Store;
fn main()
{
    let mut store = Store::new();
    println!("{:?}", store.set("pseudo", "paul"));
    println!("{:?}", store.set("pseudo", "paula"));
    println!("{:?}", store);
    println!("{:?}", store.get("pseud"));
    println!("{:?}", store.del("psdo"));
    println!("{:?}", store);
}
