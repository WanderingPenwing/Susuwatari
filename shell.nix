{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    pkg-config
    libdbusmenu
    dbus
    cargo
    rustc
  ];
}
