#!/bin/bash

if /usr/local/bin/tmux has-session 1>/dev/null 2>/dev/null
then
  exec /usr/local/bin/tmux -2 -u attach-session
else
  exec /usr/local/bin/tmux -2 -u new-session
fi
