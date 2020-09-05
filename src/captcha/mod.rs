use std::{collections::HashMap, cell::RefCell};
pub mod solvers;

//use solvers::blocking;

pub use solvers::re_caps;

pub trait CapSolvable{

    fn get_type(&self) -> &CapTypes;

    fn solve(&self) -> Result<String,String>;

    //fn set_settings(&mut self, settings: HashMap<&'a str,&'a str>);
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum CapTypes{
    RC2,
    RC3
}

pub struct CapConn<'a>{
    conn: HashMap<CapTypes, &'a RefCell<dyn CapSolvable>>,
}


impl<'a> CapConn<'a>{

    pub fn new() -> Self{
        Self{
            conn: HashMap::new()
        }
    }

    pub fn add(&mut self, name: CapTypes, solver: &'a RefCell<dyn CapSolvable>, overwrite: bool) 
        -> Result<(), &str>{
            if solver.borrow().get_type() != &name{
                return Err("incorrect captcha type")
            }
            let ptr = self.conn.entry(name).or_insert(solver);
            if overwrite{
                *ptr = solver;
            }
            Ok(())
    }

    pub fn solve(&self, name: CapTypes) -> Result<String, String>{
        if let Some(val) = self.conn.get(&name){
            match val.borrow().solve(){
                Ok(val) => Ok(val),
                Err(err) => Err(err)
            }
        }else{
            Err(String::from("that type is not initilazed"))
        }
    }
}