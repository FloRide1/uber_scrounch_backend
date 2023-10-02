with (import <nixpkgs> { });

mkShell { buildInputs = [ diesel-cli postgresql openssl pkg-config ]; }
