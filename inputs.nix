{pkgs}: {
  nativeBuildInputs = with pkgs; [
    # Core Dependencies
    cargo
    rustc
    # GUI Dependencies
    pkg-config
    gobject-introspection
    bun
  ];
  buildInputs = with pkgs; [
    # Core Dependencies
    sqlite
    # GUI Dependencies
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];
}
