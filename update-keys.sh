#!/bin/bash

CONFIG_URL="$HOME/.config/git-public-keys/config.json"

set -x

function get-branch {
    jq -r '.target_branch' $CONFIG_URL
}

function get-repo-url {
    if [ -f "$CONFIG_URL" ]; then
        jq -r ".git_url" "$CONFIG_URL"
    else
        echo Could not load config file! >&2
        cat >&2 << EOF
This tool requires a config file in the following format:
{
    "git_url": "git@github.com:XtremeCookiez/git-public-keys.git",
    "target_branch": "main"
}
EOF
        exit 1
    fi
}

REPO_URL="$(get-repo-url)"
BRANCH="$(get-repo-url)"

mkdir /tmp/git-public-keys

git clone --branch "" "$REPO_URL" "/tmp/git-public-keys"

