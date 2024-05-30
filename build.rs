extern crate winresource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();

        res.set_icon("resources/icon.ico").set_language(0x0409);

        if cfg!(feature = "advanced") {
            res.set_manifest_file("resources/manifest_adv.xml");
        } else {
            res.set_manifest_file("resources/manifest.xml");
        }

        if let Err(e) = res.compile() {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
