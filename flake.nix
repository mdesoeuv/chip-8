{
  outputs = { nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    buildInputs = with pkgs; [
      # Keyboard input
      libxkbcommon
      # WINIT_UNIX_BACKEND=wayland
      wayland

      # Vulkan
      vulkan-headers
      vulkan-loader
    ];
    in {
      devShells.${system}.default = pkgs.mkShell {
        name = "chip-8";
        inherit buildInputs;
        shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath buildInputs}
        '';
      };
    };
}
