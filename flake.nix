{
  description = "begin-rust";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url     = "github:NixOS/nixpkgs";

 outputs = { self, nixpkgs, flake-utils }:
   flake-utils.lib.eachSystem [ "x86_64-darwin" "x86_64-linux" ] (system:
  let
    pkgs = import nixpkgs {
      inherit system;
    };
  in
  {
    devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
            rustup
            rustfmt
            rust-analyzer
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };

    checks = {
      build = self.devShell.${system};
    };
  });
}
