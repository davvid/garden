#!/bin/sh
if test -z "$1"
then
    echo 1>&2 error: subdirectory name is required
    exit 1
fi

# Create the test subdirectory
mkdir -p "$1" && (
    cd "./$1" &&
    mkdir -p repos &&
    # Create repos/example.git
    git init --bare repos/example.git && (
        # Create an empty commit
        cd ./repos/example.git &&
        tree=$(git write-tree) &&
        git commit-tree -m "$1" "$tree" >refs/heads/default &&
        git symbolic-ref HEAD refs/heads/default &&
        git commit-tree -m "$1 commit 2" -p "$(git rev-parse HEAD)" "$tree" >refs/heads/default &&
        git rev-parse HEAD >refs/heads/dev
    )
)

