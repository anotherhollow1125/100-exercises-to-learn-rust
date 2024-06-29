use orphan::MyU32;

fn main() {
    let a = MyU32(1);
    let b = MyU32(2);

    let _ = a == b;
}
