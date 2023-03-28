#!/bin/bash
for tape in $(find tapes/ -name '*.tape'); do
  ./vhs < "$tape"
done
