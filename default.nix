with import <nixpkgs> { };

let
    py_pkgs = python3Packages;
in
pkgs.mkShell rec {
    name = "avin_dev_shell";
    venvDir = "src_py/.venv"; # python

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
        py_pkgs.ruff
        openssl
    ];

    nativeBuildInputs = [
        pkg-config
    ];

    postVenvCreation = ''
        unset SOURCE_DATE_EPOCH
        pip install -r src_py/requirements.txt
    '';

    postShellHook = ''
        unset SOURCE_DATE_EPOCH
    '';
}
