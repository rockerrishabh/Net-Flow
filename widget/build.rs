fn main() {
    println!("cargo:rerun-if-changed=winmd/Microsoft.Windows.Widgets.winmd");
    println!("cargo:rerun-if-changed=Assets/app.ico");

    // Standard location for Windows SDK winmd (with env override support)
    let sdk_ver = std::env::var("NETFLOW_SDK_VERSION")
        .or_else(|_| std::env::var("WindowsSDKVersion"))
        .unwrap_or_else(|_| "10.0.26100.0".to_string());
    let sdk_ver_clean = sdk_ver.trim_matches(['\\', '/']);
    let sdk_winmd = format!(
        r"C:\Program Files (x86)\Windows Kits\10\UnionMetadata\{}\Windows.winmd",
        sdk_ver_clean
    );

    let mut args = vec!["--in", "winmd/Microsoft.Windows.Widgets.winmd"];

    if std::path::Path::new(&sdk_winmd).exists() {
        args.push("--in");
        args.push(&sdk_winmd);
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
