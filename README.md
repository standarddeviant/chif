
# `chif` is short for chiffonade

## What does it do?
Currently, this tool provides a list of all bazel targets affected by each modified file since the current git branch was created from the tracking branch.

## What's in a name?
Chiffonade is a technique for slicing basil to be easier to work with.

Similar to the culinary technique, this tool aims to make a different bazel easier to work with.

## Design and Goals
This is an experimental tool written in rust because I like rust. I want to learn more about basil while writing some rust.

This tool spawns processes and parses the output with regex. 
This is not ideal, but is 'good enough' for a proof-of-concept demonstration.
