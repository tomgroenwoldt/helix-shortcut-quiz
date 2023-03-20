#!/bin/bash
for tape in tapes/*.tape; do
  ./vhs < "$tape" &
done
