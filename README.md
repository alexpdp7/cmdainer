# Introduction

Tries to make it easy to run commands from container images. Your home is mounted inside the container, so you can work on your files.

# Requirements and installation

A working `docker` command (even podman-docker).

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
