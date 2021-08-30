#!/bin/bash

source $(dirname $0)/../../../prepare
file_path=$1

cargo fmt -- $file_path
