extern crate nix;
extern crate mach;

pub fn try_it_out() {
    println!("Trying out an experiment");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
