#!/bin/bash

set -eu

command_1="$1 list-all"
echo "Evaluating: $command_1"
eval $command_1

command_2="$1 list-all --lang python --lang rust"
echo "Evaluating: $command_2"
eval $command_2

command_3="$1 fetch --lang python"
echo "Evaluating: $command_3"
eval $command_3

command_4="$1 merge --lang python --lang rust"
echo "Evaluating: $command_4"
eval $command_4


