#!/bin/sh
echo ""$@""
gimp -i -b "(Gimp-Rotate-Right \"$@\")" -b "(gimp-quit 0)"

