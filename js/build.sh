#!/bin/bash

# Must be run through `npm run build` so `concurrently` 
# will be available.

if [[ -z "$SKIP_JS" ]]; then
  concurrently "cd js/core && npm run build" "cd js/editor && npm run build"
fi