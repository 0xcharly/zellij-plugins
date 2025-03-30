{inputs, ...}: {
  imports = [inputs.flake-parts.flakeModules.easyOverlay];

  perSystem = {
    config,
    pkgs,
    ...
  }: {
    overlayAttrs.zellij-plugins = config.packages;
    packages = {
      ultra-compact-bar = pkgs.callPackage ./mk-zellij-plugin.nix {};
    };
  };
}
