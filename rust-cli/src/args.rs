use clap::{
    Parser,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct SomeArgs{
    /// The first arguement
    pub first_arg: String,
    /// The second argument
    pub second_arg: String, 
    /// the third argument
    pub thrid_arg: String,
}
