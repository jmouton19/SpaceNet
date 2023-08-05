#!/bin/bash
export JAVA_HOME=/lib/jvm/java-1.8.0-openjdk-1.8.0.362.b09-2.fc38.x86_64
export LD_LIBRARY_PATH=target/release
mkdir src/c_ffi/java/bin/
javac -h src/c_ffi -d src/c_ffi/java/bin/ src/c_ffi/java/src/com/example/*.java
javac -cp src/c_ffi/java/bin examples/*.java
gcc -shared -fPIC -I${JAVA_HOME}/include -I${JAVA_HOME}/include/linux src/c_ffi/java_wrapper.c -Wl,-rpath,'$ORIGIN' -Ltarget/release -lspace_net -o target/release/libjava_wrapper.so

