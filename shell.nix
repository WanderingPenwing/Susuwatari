{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    pkg-config
    libdbusmenu
    dbus
    glib.dev 
    cargo
    rustc
    cairo
    gdk-pixbuf
    pango
    gtk3
    atk
	gcc
  ];
}
