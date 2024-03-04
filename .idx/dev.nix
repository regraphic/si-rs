{ pkgs, ... }: {
    channel = "unstable";
    # We need rustup, then install the required toolchain
    packages = [
        pkgs.rustup
    ];

    idx.extensions = [
        "rust-lang.rust"
    ];

    idx.workspace.onCreate = {
        install-rust = "rustup install nightly";
    };
}