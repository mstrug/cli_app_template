use vergen::{Config, vergen, ShaKind};

fn main() {
    // Generate environment variables
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    if vergen(config.clone()).is_err() {
        println!("cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=0");
        println!("cargo:rustc-env=VERGEN_GIT_SHA_SHORT=0");
        *config.git_mut().enabled_mut() = false;  // building without .git folder case
        if vergen(config).is_err() {
            println!("cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=0");
        }
    }
}
