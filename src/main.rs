mod tpfs;

fn main() {
    match tpfs::get_root() {
        Some(d) => println!("Hello, {}!", d.display()),
        None => println!("There are no password storage!"),
    }

    match tpfs::get_recipients() {
        Some(l) => {
            for r in l {
                println!("{}", r.display())
            }
        }
        None => println!("No recipients"),
    }
}
