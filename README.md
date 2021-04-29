# Rust related hacks for the Sailfish OS SDK

This repository contains some hacks that I use when developing in Rust
with the Sailfish OS SDK.

## `sdk-poddocker`

A podman wrapper "script" written in Rust to fake docker outputs from
podman. It sort of works in a cgroups v2 based environemnt as it starts sshd
when the Sailfish SDK engine is started.

## Docker images

These docker images are based on [https://hub.docker.com/u/coderus](Coderus's images)
in which the Rust target is installed. This operation is sometimes tricky to be
done without error.
