mod collaborative_filtering;
mod object;
mod logger;
mod test;

use crate::logger::logger::Logger;

use crate::test::test::status;
use crate::test::test::test;

fn main() {
    Logger::inform_status(&status());

    if status() == false {
        // MAIN
    } else {
        test();
    }
}












