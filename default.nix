with import <nixpkgs> { };

let
    py_pkgs = python3Packages;
in
pkgs.mkShell rec {
    name = "avin_dev_shell";
    venvDir = "avin_data/.venv"; # python

    libPath = with pkgs; lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ];
    LD_LIBRARY_PATH = libPath;

    buildInputs = [
        py_pkgs.python
        py_pkgs.venvShellHook
        py_pkgs.numpy
        py_pkgs.polars
        openssl
    ];

    nativeBuildInputs = [
        pkg-config
    ];

    postVenvCreation = ''
        unset SOURCE_DATE_EPOCH
        pip install -r avin_data/requirements.txt
    '';

    postShellHook = ''
        unset SOURCE_DATE_EPOCH
    '';
}
