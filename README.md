# Git public keys

## Motivation

Overall goal of this service is to sync authorized keys using a git repository. This allows for a
couple things. One, you could have a dynamic list of authorized users on a shared machine and manage
the users via pull requests. Two, for personal use this allows very quick and easy authorization to
a machine. And if you need to later add another key, you can do so very easily.

## Repository Structure

The repositories that are used to hold public keys are expected in one of 2 formats. Either it
contains a `public_keys` folder which holds a series of public key files in the format
`<file-name>.pub`. Or it contains an `authorized_keys` file which holds a public key on each line of
the file.
