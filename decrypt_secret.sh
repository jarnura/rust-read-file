#!/bin/sh

# Decrypt the file
mkdir $HOME/test
# --batch to prevent interactive command
# --yes to assume "yes" for questions
gpg --quiet --batch --yes --decrypt --passphrase="$SECRET_PASSPHRASE" \
--output $HOME/test/auth.json my_secret.json.gpg