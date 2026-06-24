#!/usr/bin/env bash

if [[ -n $SOLANA_ZIG_VERSION ]]; then
  solana_zig_version="$SOLANA_ZIG_VERSION"
else
  solana_zig_version="v1.52.0"
fi
solana_zig_release_url="https://github.com/joncinque/solana-zig-bootstrap/releases/download/solana-$solana_zig_version"

output_dir="$1"
if [[ -z $output_dir ]]; then
  output_dir="solana-zig"
fi
output_dir="$(mkdir -p "$output_dir"; cd "$output_dir"; pwd)"
cd $output_dir

arch=$(uname -m)
if [[ "$arch" == "arm64" ]]; then
  arch="aarch64"
fi
case $(uname -s | cut -c1-7) in
"Linux")
  os="linux"
  abi="musl"
  ;;
"Darwin")
  os="macos"
  abi="none"
  ;;
"Windows" | "MINGW64")
  os="windows"
  abi="gnu"
  ;;
*)
  echo "install-solana-zig.sh: Unknown OS $(uname -s)" >&2
  exit 1
  ;;
esac

solana_zig_tar=zig-$arch-$os-$abi.tar.bz2
url="$solana_zig_release_url/$solana_zig_tar"
echo "Downloading $url"
curl --proto '=https' --tlsv1.2 -SfOL "$url"
echo "Unpacking $solana_zig_tar"
tar -xjf $solana_zig_tar
rm $solana_zig_tar

solana_zig_dir="zig-$arch-$os-$abi-baseline"
mv "$solana_zig_dir"/* .
rmdir $solana_zig_dir
echo "solana-zig compiler available at $output_dir"
