{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    buildInputs = [
        pkgs.cargo
        pkgs.rustc
        pkgs.fftw
        pkgs.fftwFloat
    ];
    shellHook = with pkgs; ''
       export LD_LIBRARY_PATH=${fftw.out}/lib:${fftwFloat.out}/lib
    '';
}
