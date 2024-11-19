fn main() {
    println!("Hello, world!");
    // 適当なAPIにアクセスしてみる
    let url = "https://jsonplaceholder.typicode.com/todos";
    let body = ureq::get(&url).call().unwrap().into_string().unwrap();
    println!("{}", body);
}
