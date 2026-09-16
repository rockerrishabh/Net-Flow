fn main() {
    println!("cargo:rerun-if-changed=winmd/Microsoft.Windows.Widgets.winmd");
    println!("cargo:rerun-if-changed=Assets/app.ico");

    // Standard location for Windows SDK winmd
    let sdk_winmd =
        r"C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.26100.0\Windows.winmd";

    let mut args = vec!["--in", "winmd/Microsoft.Windows.Widgets.winmd"];

    if std::path::Path::new(sdk_winmd).exists() {
        args.push("--in");
        args.push(sdk_winmd);
    }

    args.extend(&[
        "--out",
        "src/bindings.rs",
        "--filter",
        "Microsoft.Windows.Widgets",
        "--implement",
    ]);

    windows_bindgen::bindgen(args);

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("Assets/app.ico");
        if let Err(e) = res.compile() {
            eprintln!("cargo:warning=Failed to compile Windows resource: {}", e);
        }
    }
}
