#!/bin/bash

echo "Running socat ..."
socat TCP-LISTEN:9999,reuseaddr,fork TCP:localhost:8080 &

echo "Running NetMath ..."
/app/dist/netmath
