fn main() {
    // Link to SDL2
    let lib_path = std::env::var("SDL2_LIB_PATH")
        .expect("Set SDL2_LIB_PATH env variable");
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=SDL2");

    // Link to SDL2_image
    let img_lib_path = std::env::var("SDL2_IMAGE_LIB_PATH")
        .expect("Set SDL2_IMAGE_LIB_PATH env variable");
    println!("cargo:rustc-link-search=native={}", img_lib_path);
    println!("cargo:rustc-link-lib=SDL2_image");
}
