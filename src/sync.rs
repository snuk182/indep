#[macro_export]
macro_rules! indep_pool_sync {
    ($base:ty, $($tra:ident),+) => {
        pub enum Implementation {
            $(
            $tra(::std::rc::Rc<::std::cell::RefCell<$tra>>)
            ),+
        }
        
        impl ::std::fmt::Display for Implementation {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $(&Implementation::$tra(_) => write!(f, "{}", stringify!($tra))),*
                }
            }
        }
        
        pub trait Dependency {
            fn impls(&self) -> Vec<Implementation>;
            fn as_dependent(&self) -> ::std::rc::Rc<::std::cell::RefCell<Dependent>>;
            fn as_base(&self) -> ::std::rc::Rc<::std::cell::RefCell<$base>>;
            fn dep_id(&self) -> &'static str;
        }
        
        pub trait Dependent {
            fn set(&mut self, &Implementation, &Vec<String>);
        }
        
        pub struct Pool {
            deps: Vec<(Vec<String>, Box<Dependency>)>,
        }
        
        impl Pool {
            pub fn new() -> Pool {
                Pool {
                    deps: Vec::new()
                }
            }
            
            pub fn stat(&self) -> String {
                let mut s = String::new();
                
                for &(ref ptags, ref pd) in &self.deps {
                    s.push_str(format!("{}", pd.dep_id()).as_str());
                    if ptags.len() > 0 {
                        s.push_str(" as ");
                        for ptag in ptags {
                            s.push_str(format!("{},", ptag.as_str()).as_str());
                        }
                    }
                    
                    s.push_str(" (");
                    for ref pimp in pd.impls() {
                        s.push_str(format!("{},", pimp).as_str());
                    }
                    s.push_str(") ");
                    
                    s.push_str(" / ");
                }
                
                s
            }
            
            pub fn add(&mut self, ad: Box<Dependency>) {
                self.add_tagged(ad, Vec::with_capacity(0));
            }
            
            pub fn add_tagged(&mut self, ad: Box<Dependency>, atag: Vec<String>) {
                let adc = ad.as_dependent().clone();
                let mut adep = adc.borrow_mut();
            
                for &(ref ptag, ref pd) in &self.deps {
                    let pdc = pd.as_dependent();
                    let mut pdep = pdc.borrow_mut(); 
                    
                    for ref aimp in ad.impls() {
                        pdep.set(aimp, &atag);
                    }
                    
                    for ref pimp in pd.impls() {
                        adep.set(pimp, &ptag);
                    }
                }
                
                self.deps.push((atag, ad));
            } 
        }
    }
}

#[macro_export]
macro_rules! indep_impls_sync {
    ($imp:ty, $base:ty, [$($tra:ident),+]) => {                
        struct RcRefCellBox(::std::rc::Rc<::std::cell::RefCell<Box<$imp>>>);
        
        impl Dependency for RcRefCellBox {
            fn impls(&self) -> Vec<Implementation> {
                vec![
                    $(
                        Implementation::$tra(self.0.clone())
                    ),+
                ]
            }
            fn as_dependent(&self) -> ::std::rc::Rc<::std::cell::RefCell<Dependent>> {
                self.0.clone()
            }
            fn as_base(&self) -> ::std::rc::Rc<::std::cell::RefCell<$base>> {
                self.0.clone()
            }
            fn dep_id(&self) -> &'static str {
                stringify!($imp)
            }
        }
    }
}

#[macro_export]
macro_rules! indep_default_new_sync {
    ($imp:ident) => {
        pub fn new_dep() -> Box<Dependency> {
            Box::new(RcRefCellBox(::std::rc::Rc::new(::std::cell::RefCell::new(Box::new($imp::new())))))
        }
    }
}

#[macro_export]
macro_rules! indep_reqs_sync {
    ($imp:ty, [$($tra:ident: [$($id:ident),+]),+]) => {
        impl Dependent for Box<$imp> {
            fn set(&mut self, i: &Implementation, tags: &Vec<String>) {
                match i {
                    $(
                    &Implementation::$tra(ref tr) => {
                        $(
                        if tags.len() < 1 {
                            self.$id = Some(tr.clone());
                            trace!("{} is set to {} as {}", i, stringify!($imp), stringify!($id));
                        } else {
                            for tag in tags {
                                if tag != "" && tag == stringify!($id) {
                                    self.$id = Some(tr.clone());
                                    trace!("{} is set to {} as {}", i, stringify!($imp), stringify!($id));
                                }
                            }
                        }
                        )+
                    }
                    ),+
                    _ => {}
                }
            }
        }
    };
    
    ($imp:ty, []) => {
        impl Dependent for Box<$imp> {
            #[allow(unused_variables)]
            fn set(&mut self, i: &Implementation, tag: &Vec<String>) {}
        }
    }
}

#[cfg(test)]
mod test {
	pub mod base {
        pub trait Base {
            fn init(&mut self);
        }
    }
    
    pub mod t1 {
        pub trait Trait1 {
            fn do1(&self);
        }
    }
    
    pub mod t2 {
        pub trait Trait2 {
            fn do2(&self) -> String;
        }
    }
    
    pub mod t3 {
        pub trait Trait3 {
            fn do3(&mut self);
        }
    }
    
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
        
        indep_reqs_sync!{Impl1, []}
        indep_impls_sync!{Impl1, Base, [Trait1,Trait2]}
        indep_default_new_sync!{Impl1}
    }
    
    pub mod i2 {
        use super::{Dependency,Dependent,Implementation};
        
        use super::t1::Trait1;
        use super::t2::Trait2;
        use super::base::Base;
        
        use std::rc::Rc;
        use std::cell::RefCell;
        
        struct Impl2 {
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
        
        indep_reqs_sync!{Impl2, [Trait1: [t1]]}
        indep_impls_sync!{Impl2, Base, [Trait2]}
        indep_default_new_sync!{Impl2}
    }
    
    pub mod i3 {
        use super::{Dependency,Dependent,Implementation};
    
        use super::t3::Trait3;
        use super::t2::Trait2;
        use super::t1::Trait1;
        use super::base::Base;
        
        use std::rc::Rc;
        use std::cell::RefCell;
        
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
        
        indep_reqs_sync!{Impl3, [Trait1: [t1_1], Trait2: [t2_1,t2_2]]}
        indep_impls_sync!{Impl3, Base, [Trait3]}
        indep_default_new_sync!{Impl3}
    }
    
    use self::t3::Trait3;
    use self::t2::Trait2;
    use self::t1::Trait1;
    use self::base::Base;
        
    indep_pool_sync!{Base, Trait1,Trait2,Trait3}    
    
    #[test]
    fn test_sync() {
        let mut pool = Pool::new();
        
        let t1 = i1::new_dep();
        let t2 = i2::new_dep();
        let t3 = i3::new_dep();
        
        pool.add_tagged(t1, vec!["t1_1".to_string()]);
        pool.add(t2);
        pool.add(t3);
        
        println!("Pool stat: {}", pool.stat());
    }
}