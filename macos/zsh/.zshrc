#!/usr/bin/env zsh

alias ll='ls -lhGA'

export HISTSIZE=1048576
setopt hist_ignore_all_dups
setopt hist_ignore_space

export LC_CTYPE='en_GB.UTF-8'
export LC_MESSAGES='en_GB.UTF-8'
export LANG='en_GB.UTF-8'

export PS1='%{%(?.%F{green}.%F{red})%}❯ %{%f%}'

# GCloud:
export PATH="$HOME/bin/google-cloud-sdk/bin:$PATH"

# Golang:
export GOPATH="$HOME/dev/golang"
export PATH="$GOPATH/bin:$PATH"

# Rust:
export PATH="$HOME/.cargo/bin:$PATH"

# Personal:
export PATH="$HOME/bin:$PATH"
