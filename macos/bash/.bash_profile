#!/bin/bash

export HISTCONTROL=ignoreboth
export HISTFILESIZE=1048576

# nano will be used on platforms where micro is not installed:
export EDITOR=micro
#export EDITOR=nano

# TODO check if the date and currency formatting is ok:
export LC_CTYPE='en_GB.UTF-8'
export LC_MESSAGES='en_GB.UTF-8'
export LANG='en_GB.UTF-8'

alias ll='ls -lhGA'
#alias ssha='ssh -A'

# golang:
export GOPATH="$HOME/dev/go"

# useful when more machines on the network, prints username and host:
#export PS1='\[$( if [[ $? -eq 0 ]] ; then echo -n "\033[0;32m" ; else echo -n "\033[0;31m" ; fi )\]\u@\h \w❯ \[\033[0m\]'
# useful with tmux or single-user workstation:
#export PS1='\[$( if [[ $? -eq 0 ]] ; then echo -n "\033[0;32m" ; else echo -n "\033[0;31m" ; fi )\]\w❯ \[\033[0m\]'
# don't print current dir if tmux already has it:
export PS1='\[$( if [[ $? -eq 0 ]] ; then echo -n "\033[0;32m" ; else echo -n "\033[0;31m" ; fi )\]❯ \[\033[0m\]'

# add ~/bin and cargo to PATH:
export PATH="$HOME/bin:$HOME/.cargo/bin:$HOME/bin/google-cloud-sdk/bin:$PATH"
