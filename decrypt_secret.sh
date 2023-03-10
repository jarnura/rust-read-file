#!/bin/sh

# Decrypt the file
mkdir $HOME/target/

mkdir $HOME/target/test
# --batch to prevent interactive command
# --yes to assume "yes" for questions
gpg --quiet --batch --yes --decrypt --passphrase="$SECRET_PASSPHRASE" \
--output $HOME/target/test/auth.json my_secret.json.gpg

gpg --quiet --batch --yes --decrypt --passphrase="$SECRET_PASSPHRASE" \
--output $HOME/target/test/auth_toml.toml auth.toml.gpg