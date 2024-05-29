extern crate winres;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();

        res.set_icon("icon.ico").set_language(0x0409);

        if cfg!(feature = "advanced") {
            res.set_manifest_file("manifest_adv.xml");
        } else {
            res.set_manifest_file("manifest.xml");
        }

        if let Err(e) = res.compile() {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
