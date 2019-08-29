#!/bin/bash

export HISTCONTROL=ignoreboth
export HISTFILESIZE=1048576

# nano will be used on platforms where micro is not installed:
export EDITOR=micro
#export EDITOR=nano

# TODO does not really work as expected any way:
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
#export PS1='\[$( if [[ $? -eq 0 ]] ; then echo -n "\033[0;32m" ; else echo -n "\033[0;31m" ; fi )\]❯ \[\033[0m\]'
# set "title" to current cwd:
#export PS1='\[\033]0;\w\007\]\[$( if [[ $? -eq 0 ]] ; then echo -n "\033[0;32m" ; else echo -n "\033[0;31m" ; fi )\]❯ \[\033[0m\]'
##                                                                                                                    ^^^^^^^^^^^ non-visible reset colors back to normal
##                                                                                                                  ^^ actual visible prompt
##                                                                                                               ^^^ end of non-visible output capture
##                                                                                     ^^^^^^^^^^^^^^^^^^^ red prompt if previous command failed
##                                                         ^^^^^^^^^^^^^^^^^^^^ green prompt if previous command succeeded
##                                ^^^^^^^^^^^^^^^^^ checking previous exit code
##                           ^^^^ start of non-visible output capture
##          ^^^^^^^^^^^^^^^^^ non-visible set current shell title to cwd
# setting title to abbreviated cwd:
# TODO: this one is broken, feed line will have color of abbreviate-cwd exit code, which is almost always 0:
#export PS1='\[\033]0;$(abbreviate-cwd)\007\]\[$( if [[ $? -eq 0 ]] ; then echo -n "\033[0;32m" ; else echo -n "\033[0;31m" ; fi )\]❯ \[\033[0m\]'
#
# latest iTerm has fantastic status bar, don't need cwd any more:
export PS1='\[$( if [[ $? -eq 0 ]] ; then echo -n "\033[0;32m" ; else echo -n "\033[0;31m" ; fi )\]❯ \[\033[0m\]'

# add ~/bin and cargo to PATH:
export PATH="$HOME/bin:$HOME/.cargo/bin:$HOME/bin/google-cloud-sdk/bin:$PATH"
