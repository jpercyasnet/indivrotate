#!/bin/sh
echo ""$@""
gimp -i -b "(Gimp-Rotate-Left \"$@\")" -b "(gimp-quit 0)"

