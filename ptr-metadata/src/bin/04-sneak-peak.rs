use std::mem;

trait Pet {
    fn name(&self) -> &'static str;
}

struct Dog {
    age: u8,
}

impl Pet for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

fn main() {
    // Create a new dog.
    let dog = Dog { age: 10 };

    // Store a a reference to dog as a trait object, erasing its type.
    let erased_ptr: &dyn Pet = &dog;

    // Decompose our fat pointer into the thin pointer and vtable metadata.
    let (dog_ptr, vtable): (*const Dog, *const usize) = unsafe { mem::transmute(erased_ptr) };

    // Here be dragons, there's no going back after this point.
    unsafe {
        // Make sure that actually is the pointer to our dog.
        assert_eq!((*dog_ptr).age, 10);

        // struct DogVTable {
        //     drop_in_place: fn(*mut Dog),
        //     size: usize,
        //     align: usize,
        //     pet_name: fn(&Dog) -> &'static str,
        // }

        // Assert first entry is a function pointer to drop_in_place.
        assert_eq!(*vtable, std::ptr::drop_in_place::<Dog> as usize);

        // Assert second and third entries are size and alignment.
        assert_eq!(*vtable.add(1), mem::size_of::<Dog>());
        assert_eq!(*vtable.add(2), mem::align_of::<Dog>());

        // Assert fourth entry is Dog::name implementation.
        assert_eq!(*vtable.add(3), Dog::name as usize);

        // Run Dog::name and confirm output.
        let dog_name: fn(*const Dog) -> &'static str = mem::transmute(*vtable.add(3));
        assert_eq!((dog_name)(dog_ptr), "Dog");
    }
}
