fn main() {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();
}
