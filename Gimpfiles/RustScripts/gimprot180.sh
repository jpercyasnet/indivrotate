#!/bin/sh
echo ""$@""
gimp -i -b "(Gimp-Rotate-180 \"$@\")" -b "(gimp-quit 0)"

