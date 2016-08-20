#[macro_use]
extern crate indep;
#[macro_use]
extern crate log;
//`sync` mod contains DI set for single-threaded environments (uses Rc<RefCell<>> as an abstraction).

//We pretend that all the DI module traits and implementations are separated into different mods.

//Base trait for all depencencies. May contain no functions, `init` here is just an example.
pub mod base {
    pub trait Base {
        fn init(&mut self);
    }
}

//Sample trait #1
pub mod t1 {
    pub trait Trait1 {
        fn do1(&self);
    }
}

//Sample trait #1
pub mod t2 {
    pub trait Trait2 {
        fn do2(&self) -> String;
    }
}

//Sample trait #1
pub mod t3 {
    pub trait Trait3 {
        fn do3(&mut self);
    }
}

//Sample implementation struct #1 - implements trait #1 and trait #2 (and, of course, trait Base).
pub mod i1 {
    use super::{Dependency,Dependent,Implementation};
    
    use super::base::Base;
    use super::t1::Trait1;
    use super::t2::Trait2;
    
    use std::rc::Rc;
    use std::cell::RefCell;
    
    struct Impl1;
    
    impl Impl1 {
        pub fn foo(&self) {
            println!("foo from Impl1");
        }
        
        pub fn new() -> Impl1 {
            Impl1
        }
    }
    
    impl Trait1 for Box<Impl1> {
        fn do1(&self) {
            self.foo();
        }
    }
    
    impl Trait2 for Box<Impl1> {
        fn do2(&self) -> String {
            format!("Impl1 says 'Trait2'")
        }
    }
    
    impl Base for Box<Impl1> {
        fn init(&mut self) {
            self.foo();
        }
    }
    
    //Here comes `indep`.
    //This macro defines requirements of the DI module implementation. 
    //The syntax is {Impl_Name, [requirement_name_1: requirement_trait_1, requirement_name_2: requirement_trait_2], ... }.
    //Here `Impl1` does not have dependecies, so its requirement array is empty.
    indep_reqs_sync!{Impl1, []}
    
    //This macro defines the implementations of the DI module. The syntax is {Impl_Name, Base_Trait_Name, [trait_1, trait_2, ... ]}
    //`Impl1` implements `Trait1` and `Trait2`, so it is stated in a macro.
    indep_impls_sync!{Impl1, Base, [Trait1,Trait2]}
    
    //This macro generates default implementation of the `new()` function, that returns Box<Dependency>.
    //The `Dependency` itself is an trait that DI pool accepts. Internally it is always implemented by a struct `RcRefCellBox`,
    //which is a wrapper for Rc<RefCell<Box<Impl1>>>. This wrapper is required for Rust typecasting.
    indep_default_new_sync!{Impl1}
}

//Sample implementation struct #2 - implements trait #2 (and trait Base). Depends on `Trait1` - see the mention of `indep_reqs_sync` below.
pub mod i2 {
    use super::{Dependency,Dependent,Implementation};
    
    use super::t1::Trait1;
    use super::t2::Trait2;
    use super::base::Base;
    
    use std::rc::Rc;
    use std::cell::RefCell;
    
    struct Impl2 {
    	//It is an Indep library requirement to have injectable dependencies in a form of Option<Rc<RefCell<Trait>>>
        t1: Option<Rc<RefCell<Trait1>>>
    }
    
    impl Impl2 {
        pub fn boo(&self) {
            println!("boo from Impl2");
            let b = self.t1.as_ref().unwrap();
            b.borrow().do1();
        }
        
        pub fn new() -> Impl2 {
            Impl2 {
                t1:None
            }
        }
    }
    
    impl Trait2 for Box<Impl2> {
        fn do2(&self) -> String {
            self.boo();
            format!("Impl2 says 'Trait2'")
        }
    }
    
    impl Base for Box<Impl2> {
        fn init(&mut self) {
            self.boo();
        }
    }
    
    //`Impl2` requires `Trait1` inside as member named `t1`. `t1` should have type Option<Rc<RefCell<Trait1>>>.
    indep_reqs_sync!{Impl2, [Trait1: [t1]]}
    
    //See corresponding statement for `Impl1` above. The implementation struct should (obviously) implement at least one DI trait.
    indep_impls_sync!{Impl2, Base, [Trait2]}
    
    //See corresponding statement for `Impl1` above.
    indep_default_new_sync!{Impl2}
}

//Sample implementation struct #3 - implements trait #3 (and trait Base). Depends on `Trait1` as `t1_1`, 
//`Trait2` as `t2_1` and `t2_2` (sepatare instances) - see the mention of `indep_reqs_async` below.
pub mod i3 {
    use super::{Dependency,Dependent,Implementation};

    use super::t3::Trait3;
    use super::t2::Trait2;
    use super::t1::Trait1;
    use super::base::Base;
    
    use std::rc::Rc;
    use std::cell::RefCell;
    
    //It is an Indep library requirement to have injectable dependencies in a form of Option<Rc<RefCell<Trait>>>
    struct Impl3 {
        t2_1: Option<Rc<RefCell<Trait2>>>,
        t2_2: Option<Rc<RefCell<Trait2>>>,
        t1_1: Option<Rc<RefCell<Trait1>>>
    }
    
    impl Impl3 {
        pub fn oo(&mut self) {
            let b1 = self.t2_1.as_mut().unwrap();
            let b2 = self.t2_2.as_mut().unwrap();
            let b3 = self.t1_1.as_mut().unwrap();
            println!("oo from Impl3: \n1: {}\n2: {}",
                b1.borrow_mut().do2(),
                b2.borrow_mut().do2()
            );
            b3.borrow().do1();
        }
        
        pub fn new() -> Impl3 {
            Impl3 {
                t1_1: None,
                t2_1: None,
                t2_2: None,
            }
        }
    }
    
    impl Trait3 for Box<Impl3> {
        fn do3(&mut self) {
            self.oo();
        }
    }
    
    impl Base for Box<Impl3> {
        fn init(&mut self) {
            self.oo();
        }
    }
    
    //`Impl3` depends on `Trait1` and two different instances of `Trait2`. 
    indep_reqs_sync!{Impl3, [Trait1: [t1_1], Trait2: [t2_1,t2_2]]}
    
    //See corresponding statement for `Impl1` above. The implementation struct should (obviously) implement at least one DI trait.
    indep_impls_sync!{Impl3, Base, [Trait3]}
    
    //See corresponding statement for `Impl1` above.
    indep_default_new_sync!{Impl3}
}

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Display,Formatter,Result};

use self::t3::Trait3;
use self::t2::Trait2;
use self::t1::Trait1;
use self::base::Base;
    
//Initialize all the DI classes and traits  The syntax is {Base_Trait, Trait1, Trait2, ... }, where Trait# is a trait which needs to be DI-enabled.    
indep_pool_sync!{Base, Trait1,Trait2,Trait3}    

fn main() {
	//`Pool` is a structure created by `indep_pool_async` macro.
    let mut pool = Pool::new();
    
    let t1 = i1::new_dep();
    let t2 = i2::new_dep();
    let t3 = i3::new_dep();
    
    //Here we mark this struct with a special tag so it will be injected only to similarly named member of a dependent struct.
    pool.add_tagged(t1, vec!["t1_1".to_string()]);
    //Add DI dependency with no tag, which means it will be injected to any struct that has a dependency of the corresponding trait.
	pool.add(t2);
	//Same here.
	pool.add(t3);
	
	//`stat()` is a simple utility method of a `Pool` that shows its content name-wise.
	println!("Pool stat: {}", pool.stat());
}