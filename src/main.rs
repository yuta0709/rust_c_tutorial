#[link(name="hello", kind="static")]
extern "C"{
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
    println!("Hello, world from Rust!");
}
