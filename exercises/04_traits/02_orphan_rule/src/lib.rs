// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

/*
impl PartialEq for u32 {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
*/

#[derive(Clone, Copy, Debug)]
pub struct MyU32(pub u32);

impl PartialEq for MyU32 {
    fn eq(&self, other: &Self) -> bool {
        let res = self.0 == other.0;
        println!("Eq method called. {:?} == {:?} is {}", self, other, res);
        res
    }
}
