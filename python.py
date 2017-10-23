#! /usr/bin/env python2

import sys

w = [l.split(' ', 1) for l in sys.stdin]
pref = [x[0] for x in w]
suff = [x[1].rstrip() for x in w]

for p in pref:
    print "\n".join([(p + s) for s in suff])
