#!/bin/bash

# Script to run all the `api-collection` assertions.

VARS="tests/api-collection/props/local"

#
# Test `Meta`

hurl --test --variables-file "$VARS" --glob "tests/api-collection/meta/**/*.hurl"

#
# Test `User`

# Only run `query` assertions.
files=(
	"user"
	"users"
	"users_both_first_and_last"
	"users_first_3"
	"users_first_3_after_n"
	"users_invalid_cursor"
	"users_last_1_before_n"
	"users_last_3"
	"users_no_first_or_last"
)
for file in "${files[@]}"; do
	hurl --test --variables-file "$VARS" "tests/api-collection/user/${file}.hurl"
done
