{ pkgs ? import <nixpkgs> {} }:
let
  rust = pkgs.rustChannels.nightly.rust.override {
      extensions = ["rust-src"];
  };
  rustNeovim = pkgs.neovim.override {
    configure = {
      plug.plugins = with pkgs.vimPlugins; [
        coc-nvim
      ];
      customRC = ''
        call coc#config('rust-analyzer', {
          \'serverPath': '${rust}/bin/rust-analyzer'
        \})
        nmap <silent> gd <Plug>(coc-definition)
      '';
    };
    viAlias = true;
    vimAlias = true;
  };

in

pkgs.mkShell {
  buildInputs = [
    rust
    rustNeovim   
  ];
}
