{
  description = "CHIP-8 emulator";

  inputs = {
    # Formatter for entire projects
    treefmt.url = "github:numtide/treefmt-nix";

    # Nix packaging for rust's cargo projects
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    { nixpkgs, treefmt, crane, ... }:
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

        # Package wrapper
        makeWrapper
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

      # Create rust build tools
      craneLib = crane.mkLib pkgs;

      # Filter project for sources
      src = craneLib.cleanCargoSource ./.;

      # Inputs for craneLib.build
      craneBuildInputs = {
        inherit src;

        # Build tools
        inherit nativeBuildInputs;

        # Dependencies 
        inherit buildInputs;

        # Execution environment variables
        inherit LD_LIBRARY_PATH;
        inherit ALSA_PLUGIN_DIR;
      };

      # Cache-build dependencies
      craneLibArtifacts = craneLib.buildDepsOnly craneBuildInputs;

      # Chip-8 package without wrapping
      chip-8 = craneLib.buildPackage (craneBuildInputs // {
        inherit craneLibArtifacts;
        postInstall = ''
            wrapProgram $out/bin/chip-8 \
            	--set ALSA_PLUGIN_DIR ${pkgs.alsa-plugins}/lib/alsa-lib \
            	--suffix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
        '';
      });

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

      # Dynamic libraries to loading path
      LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";

      # Support to various higher level audio systems (pulseaudio, jack, ...)
      ALSA_PLUGIN_DIR = "${pkgs.alsa-plugins}/lib/alsa-lib";

    in
    {
      # Formatter wrapping all formatters
      formatter.${system} = treefmt-config.build.wrapper;

      # Developement shell accessible with `nix develop`
      devShells.${system}.default = craneLib.devShell {
        name = "chip-8";

        # Import build inputs from chip-8
        inputsFrom = [ chip-8 ];

        # Execution environment variables
        inherit LD_LIBRARY_PATH;
        inherit ALSA_PLUGIN_DIR;

        # Provide the appropriate LSP server
        packages = [ pkgs.rust-analyzer ];
      };

      # Packaged chip-8
      packages.${system}.default = chip-8;
    };
}
