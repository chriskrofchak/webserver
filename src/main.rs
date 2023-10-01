mod server;
use server::RustWS;


fn main() {
    let rustws = RustWS::new();
    rustws.listen();
}
