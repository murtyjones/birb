use api_lib::models::filer::Model as Filer;
use api_lib::models::filer::Model as Filer;
use filer_status_lib::FilerStatus;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let cik = &args[0];

    let f = Filer {
        cik
    };

    let fs = FilerStatus::new(f);

    fs.set_is_active();
    println!("{}", fs);
    // TODO save in database
}