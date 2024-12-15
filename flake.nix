{
  description = "CHIP-8 emulator";

  outputs =
    { nixpkgs, ... }:
    let
      # Supported target
      system = "x86_64-linux";

      # Package set used
      pkgs = nixpkgs.legacyPackages.${system};

      # Building tools
      nativeBuildInputs = with pkgs; [
        # Package config
        pkg-config

        # Add support for mold the super fast linker
        mold
        # It requires the presence of clang to work
        clang
      ];

      # Libraries
      buildInputs = with pkgs; [
        # Keyboard input
        libxkbcommon

        # Wayland windowing protocol
        wayland

        # Vulkan graphics
        vulkan-headers
        vulkan-loader

        # Audio
        alsa-lib
      ];

    in
    {
      # Formatter
      formatter.${system} = pkgs.nixfmt-rfc-style;

      # Devellopement shell accessible with `nix develop`
      devShells.${system}.default = pkgs.mkShell {
        name = "chip-8";

        inherit nativeBuildInputs;

        inherit buildInputs;

        # Add dynamic libraries to loading path
        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";

        # Add support to various higher level audio systems (pulseaudio, jack, ...)
        ALSA_PLUGIN_DIR = "${pkgs.alsa-plugins}/lib/alsa-lib";
      };
    };
}
