mod init;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "init" {
        init::init_repository();
    } else if args.len() > 1 && args[1] == "version" {
        println!("GVC Version 0.0.1-beta (protocol draft)");
    } else {
        println!(
            "GVC - GBS Version Control (mini VCS)\n\n\
            USAGE:\n  \
              gvc init         Initialize a new GVC repository\n  \
              gvc version      Show the version of GVC\n"
        );
    }
}
