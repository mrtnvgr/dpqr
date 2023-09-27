#!/bin/sh

base_url="https://dp.unixis.fun"

qrencode -t SVG --foreground=214fe2 "${base_url}/intro.html" -o 1.svg
qrencode -t SVG --foreground=214fe2 "${base_url}/forte.html" -o 2.svg
