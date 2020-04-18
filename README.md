# Introduction

Tries to make it easy to run commands from Docker images. Your home is mounted inside the container, so you can work on your files.

# Requirements and installation

A working Docker installation (or podman-docker).

Right now we only support Linux (macOS and Windows have been tested superficially).

```
$ wget https://github.com/alexpdp7/cmdocker/releases/latest/download/cmdocker-(linux|macos) -O ~/.local/bin/cmdocker  # or some other directory in your $PATH you can write to
$ chmod +x ~/.local/bin/cmdocker
$ cmdocker add-wrapper busy_touch /bin/touch busybox
Creating "/home/user/.local/bin/busy_touch" as symlink to "/home/user/.local/bin/cmdocker"
$ busy_touch ~/foo bar  # will work with any absolute or relative path inside $HOME
```

# Windows notes

* You will need to add a writable directory to your `$Env:Path` and install as `cmdocker.exe`
* Your `$Env.USERPROFILE` will be mounted as `/home/user` inside the container, so absolute paths probably will not work.

# Examples

Run Python versions not packaged for your distro for quick testing:

```
$ cmdocker add-wrapper python3.5 /usr/local/bin/python python:3.5
$ python3.5
Python 3.5.9 (default, Mar 31 2020, 16:56:48) 
[GCC 8.3.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>>
```

Run maven targets without even installing a JDK:

```
$ cmdocker add-wrapper mvn /usr/bin/mvn maven:latest
$ mvn validate
```

Check quickly other distribution packages:

```
$ cmdocker add-wrapper dnf-c8 /usr/bin/dnf centos:8
$ dnf-c8 search jdk
...
java-11-openjdk-demo.x86_64 : OpenJDK Demos 11
...
```

Connect to databases with client versions not packaged in your distro:

```
$ cmdocker add-wrapper psql_12 /usr/bin/psql postgres:12
$ psql_12 -V
psql (PostgreSQL) 12.2 (Debian 12.2-2.pgdg100+1)
```
