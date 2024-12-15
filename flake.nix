{
  description = "CHIP-8 emulator";

  inputs.treefmt.url = "github:numtide/treefmt-nix";

  outputs =
    { nixpkgs, treefmt, ... }:
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

      # Contains every formatter config
      treefmt-module = {
        programs = {
          # Code
          nixfmt.enable = true; # Nix
          rustfmt.enable = true; # Rust

          # Docs
          mdformat.enable = true; # Markdown

          # Configs
          yamlfmt.enable = true; # YAML
          toml-sort.enable = true; # TOML
        };

        # Exclude chip-8 roms
        settings.global.excludes = [ "*.ch8" ];
      };

      # Evaluate module into a config
      treefmt-config = (treefmt.lib.evalModule pkgs treefmt-module).config;

    in
    {
      # Formatter wrapping all formatters
      formatter.${system} = treefmt-config.build.wrapper;

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
