#!/bin/bash
(./malicious-ccd-party.x -N 3 -p 0 sha256_2 & ./malicious-ccd-party.x -N 3 -p 1 sha256_2 1>/dev/null 2>&1 & ./malicious-ccd-party.x -N 3 -p 2 sha256_2 1>/dev/null 2>&1 & wait) | grep Reg | cut -d ' ' -f 3 | sed s/^0x//
