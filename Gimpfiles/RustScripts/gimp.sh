#!/bin/sh
echo ""$@""
gimp -i -b "(Gimp-Rotate-No \"$@\")" -b "(gimp-quit 0)"

