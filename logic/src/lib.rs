#[macro_use]
extern crate validator_derive;

#[derive(Clone)]
pub struct App<Database = ()> {
    pub db: Database,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
