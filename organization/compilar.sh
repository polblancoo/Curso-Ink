#!/usr/bin/env bash
#replica comandos en consola al ejecutarse
set -eu

echo "Que compilamos..?"
echo "1 - voting"
echo "2 - psp34"
echo "3 - master"
echo "4 - ToDOS"
read accion;

case $accion in
  1)  cargo +stable contract build --manifest-path votantes/Cargo.toml;;
  2)  cargo +stable contract build --manifest-path psp34_lop/Cargo.toml;;
  3)  cargo +stable contract build --manifest-path master/Cargo.toml;;
  4)  cargo +stable contract build --manifest-path master/Cargo.toml
	  cargo +stable contract build --manifest-path psp34_lop/Cargo.toml
	  cargo +stable contract build --manifest-path votantes/Cargo.toml;;
 
  *) echo "No es una opcion.";; 
esac

##cargo  contract build
