use api_lib::models::filer::Model as Filer;
use filer_status_lib::FilerStatus;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let cik = String::to_string(&args[1]);

    let f = Filer { cik };

    let mut fs = FilerStatus::new(f);

    fs.set_is_active();
    println!("{:?}", fs);
    // TODO save in database
}
