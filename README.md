# Git public keys

## Motivation

Overall goal of this service is to sync authorized keys using a git repository. This allows for a
couple things. One, you could have a dynamic list of authorized users on a shared machine and manage
the users via pull requests. Two, for personal use this allows very quick and easy authorization to
a machine. And if you need to later add another key, you can do so very easily.
