{
  description = "Development environment";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
  inputs.nixgl.url = "github:nix-community/nixGL";

  outputs =
    { self
    , flake-utils
    , nixpkgs
    , nixgl

    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        # pkgs = nixpkgs.legacyPackages.${system};
        pkgs = import nixpkgs {
            system = "${system}";
            overlays = [ nixgl.overlay ];
        };


      in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # See https://github.com/NixOS/nixpkgs/issues/59209.
            bashInteractive
            ripgrep
            cargo
            fzf
            libdrm
            libllvm
            zoxide
            gcc
            glfw
            SDL
            glew
            glm
            pkg-config
            libxkbcommon
            vulkan-tools
            vulkan-headers
            vulkan-loader
            wayland-scanner
            egl-wayland
            libglvnd
            mesa
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXinerama
            xorg.xinput
            xorg.libXi
            xwayland
            wayland
            xorg.xeyes
            xorg.xhost
            xorg.xorgserver
            xorg.xinit
            python312Packages.glad2

          ];
          buildInputs = with pkgs; [
          ];

        };
      }

    );
}
