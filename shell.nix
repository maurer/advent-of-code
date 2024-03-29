{ pkgs ? import <nixpkgs> {} }:
let
  rust = pkgs.rustChannels.nightly.rust.override {
      extensions = ["rust-src" "clippy-preview"];
  };
  rustNeovim = pkgs.neovim.override {
    configure = {
      plug.plugins = with pkgs.vimPlugins; [
        coc-nvim
      ];
      customRC = ''
        call coc#config('rust-analyzer', {
          \'serverPath': '${rust}/bin/rust-analyzer',
          \'cargo.loadOutDirsFromCheck': v:true,
          \'procMacro.enable': v:true
        \})
	call coc#config('codeLens', {
	  \'enable': v:true
	\})
        nmap <silent> gd <Plug>(coc-definition)
	nnoremap <leader>cl :<C-u>call CocActionAsync('codeLensAction')<CR>
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
