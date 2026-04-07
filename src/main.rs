use limbo::kernel::build;


#[tokio::main]
async fn main() {
    build::parse_and_match_cli();
}

