use std::io::{StdinLock, Lines};

pub trait StdinExt {
    fn read_line(&mut self) -> String;

    fn skip_line(&mut self);
}

impl StdinExt for Lines<StdinLock<'_>> {
    fn read_line(&mut self) -> String {
        self.next().unwrap().unwrap()
    }

    fn skip_line(&mut self) {
        let _ = self.next().unwrap().unwrap();
    }

}