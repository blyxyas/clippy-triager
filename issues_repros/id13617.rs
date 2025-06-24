//ISSUE #13617 - C-bug

pub trait MyTrait {
    type MyAssociatedType;
}

pub trait MyStricterTrait: MyTrait<MyAssociatedType: Clone> {}

pub struct MyStruct;
pub struct MyStruct2;

impl MyTrait for MyStruct {
    type MyAssociatedType = u64;
}

impl MyTrait for MyStruct2 {
    type MyAssociatedType = Vec<u64>;
}

impl MyStricterTrait for MyStruct {
}

pub fn f() -> impl MyStricterTrait + MyTrait<MyAssociatedType: Copy> {
    MyStruct
}
