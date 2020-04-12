#!/bin/bash
if [ -d "/usr/src/app/tmp/pids/" ]; then rm -Rf /usr/src/app/tmp/pids/; fi
rails s