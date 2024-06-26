ARCHIVED:

I no longer use this.
Check [whalebrew](https://github.com/whalebrew/whalebrew) for a similar project to run commands from container images.
With [Distrobox](https://distrobox.it/) and [Toolbx](https://github.com/containers/toolbox), you can use a different distro that might contain the package you need.
[ubpkg](https://github.com/alexpdp7/ubpkg/) might help you obtain upstream binaries if available.

# Introduction

Tries to make it easy to run commands from container images. Your home is mounted inside the container, so you can work on your files.

This used to be called cmdocker. See below for migrating your configuration.

# Requirements and installation

A working `podman` or `docker` command.

When using `podman` on Linux, rootless operation must be configured, see https://github.com/containers/podman/blob/main/docs/tutorials/rootless_tutorial.md .
In particular, if you installed Podman using a package manager, you probably just need to configure `/etc/subuid` and `/etc/subgid` as described in that document.
Briefly, check the `/etc/subuid` and `/etc/subgid` files in your system.
If they do not exist or they do not contain an entry for your user, then run:

```
$ sudo usermod --add-subuids 100000-165535 --add-subgids 100000-165535 YOUR_USERNAME
```

Right now we only support Linux (macOS and Windows have been tested superficially).

## Linux

```
$ wget https://github.com/alexpdp7/cmdainer/releases/latest/download/cmdainer-linux -O ~/.local/bin/cmdainer  # or some other directory in your $PATH you can write to
$ chmod +x ~/.local/bin/cmdainer
```

## macOS

```
% curl --location https://github.com/alexpdp7/cmdainer/releases/latest/download/cmdainer-macos > /usr/local/bin/cmdainer
% chmod +x /usr/local/bin/cmdainer
% xattr -d com.apple.quarantine /usr/local/bin/cmdainer  # you might need this if macOS complains that the binary is not trusted
```

## Windows notes

* You will need to add a writable directory to your `$Env:Path` and install as `cmdainer.exe`
* Your `$Env.USERPROFILE` will be mounted as `/home/user` inside the container, so absolute paths probably will not work.

# Usage

```
$ cmdainer add-wrapper busy_touch /bin/touch busybox
Creating "/home/user/.local/bin/busy_touch" as symlink to "/home/user/.local/bin/cmdainer"
$ busy_touch ~/foo bar  # will work with any absolute or relative path inside $HOME
```

You can use a further argument to specify the architecture to use.

In theory, Podman for macOS ARM-based machines already ships configured with qemu to run x86-64 images.
See https://github.com/containers/podman/issues/11458#issuecomment-1257268091 for details.

# Troubleshooting

This command uses [env_logger](https://github.com/rust-cli/env_logger/), so you can configure logging by using the `RUST_LOG` environment variable.
In Bash, prefix a wrapper with `RUST_LOG=debug ` to print debugging information.

# Migrating from cmdocker

```
$ mv ~/.config/cmdocker/ ~/.config/cmdainer
$ mv ~/.config/cmdainer/cmdocker.toml ~/.config/cmdainer/cmdainer.toml
```

And update all symlinks to `cmdocker` to `cmdainer`.

# Examples

Run Python versions not packaged for your distro for quick testing:

```
$ cmdainer add-wrapper python3.5 /usr/local/bin/python python:3.5
$ python3.5
Python 3.5.9 (default, Mar 31 2020, 16:56:48) 
[GCC 8.3.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>>
```

Run maven targets without even installing a JDK:

```
$ cmdainer add-wrapper mvn /usr/bin/mvn maven:latest
$ mvn validate
```

Connect to databases with client versions not packaged in your distro:

```
$ cmdainer add-wrapper psql_12 /usr/bin/psql postgres:12
$ psql_12 -V
psql (PostgreSQL) 12.2 (Debian 12.2-2.pgdg100+1)
```

Use the latest asciidoctor without RubyGems:

```
$ cmdainer add-wrapper asciidoctor /usr/bin/asciidoctor asciidoctor/docker-asciidoctor
$ asciidoctor ...
```
