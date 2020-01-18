#!/bin/sh

docker run -i --security-opt seccomp=default.json --cap-add sys_admin --privileged=true profiler 
