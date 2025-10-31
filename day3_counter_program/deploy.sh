#!/bin/bash

#----------------------- Solana Program Build and Deploy script --------------------

PROGRAM_NAME = "day3_counter_program"
PROGRAM_PATH = "target/deploy/${PROGRAM_NAME}.so"
KEYPAIR_PATH = "$HOME/.config/solana/id.json"

echo "Building Solana Program...."
cargo build-sbf

if [$? -ne 0]; then
echo "Build Failed!!!!!"
exit 1
fi

echo "Deploying program....."
solana program deploy "$PROGRAM_PATH"  --keypair "$KEYPAIR_PATH"

if [$? -ne 0]; then
echo "Deploy failed!!!!!"
exit 1
fi

echo "Program Successfully Deployed......"

