#!/usr/bin/env bash
mspdebug -v 3300 --fet-force-id MSP430FR2476 -C mspdebug.cfg tilib "prog $1 & run"