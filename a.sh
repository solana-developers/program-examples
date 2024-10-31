#!/bin/bash -xe
build() {
declare -a ProjectDirs=(
            "basics/account-data/native/program"
            "basics/checking-accounts/native/program"
            "basics/close-account/native/program"
            "basics/counter/native/program"
            "basics/create-account/native/program"
            "basics/hello-solana/native/program"
            "basics/pda-rent-payer/native/program"
            "basics/processing-instructions/native/program"
            "basics/program-derived-addresses/native/program"
            "basics/realloc/native/program"
            "basics/rent/native/program"
            "basics/repository-layout/native/program"
            "basics/transfer-sol/native/program"
          )
          for projectDir in "${ProjectDirs[@]}"; do
            echo "
            ********
            Building $projectDir
            ********"
            cd $projectDir
            if cargo-build-sbf --verbose; then
              echo "Build succeeded for $projectDir."
            else
              failed=true
              failed_builds+=($projectDir)
              echo "Build failed for $projectDir. Continuing with the next program."
            fi
            cd - > /dev/null
          done
}

run() {
          solana -V
          rustc -V
          declare -a ProjectDirs=(
            #"basics/account-data/native/"
            #"basics/checking-accounts/native/"
            #"basics/close-account/native/"
            #"basics/counter/native/"
            #"basics/create-account/native/"
            "basics/hello-solana/native/"
            #"basics/pda-rent-payer/native/"
            #"basics/processing-instructions/native/"
            #"basics/program-derived-addresses/native/"
            #"basics/rent/native/"
            #"basics/repository-layout/native/"
            #"basics/transfer-sol/native/"
          )
          for projectDir in "${ProjectDirs[@]}"; do
            echo "
            ********
            Testing $projectDir
            ********"
            cd $projectDir
            pnpm install --frozen-lockfile
            if (cargo build-sbf --manifest-path=./program/Cargo.toml --sbf-out-dir=./tests/fixtures && pnpm test); then
              echo "Tests succeeded for $projectDir."
            else
              failed=true
              failed_tests+=($projectDir)
              echo "Tests failed for $projectDir. Continuing with the next program."
            fi
            cd - > /dev/null
          done
}

run
