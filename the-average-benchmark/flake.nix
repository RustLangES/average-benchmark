{
  description = "A flake for a Rust project using Cargo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShell = pkgs.mkShell {
        buildInputs = [
          pkgs.rustup # Rustup para gestionar Rust
          pkgs.lldb # Debugger LLDB
          pkgs.rust-analyzer # Soporte de Rust en VS Code
          pkgs.vscode-extensions.vadimcn.vscode-lldb # Extensión CodeLLDB
          pkgs.libllvm # Dependencia de LLDB en NixOS
        ];

        shellHook = ''
          rustup show active-toolchain > /dev/null 2>&1 || rustup default stable
        '';
      };
    });
}
