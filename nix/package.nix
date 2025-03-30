{inputs, ...}: {
  imports = [inputs.flake-parts.flakeModules.easyOverlay];

  perSystem = {
    config,
    pkgs,
    ...
  }: {
    overlayAttrs = {
      default = config.packages;
      zellij-ultra-compact-bar = config.packages.default;
    };
    packages.default = pkgs.callPackage ./mk-zellij-plugin.nix {};
  };
}
