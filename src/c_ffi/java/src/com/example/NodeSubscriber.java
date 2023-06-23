package com.example;

public class NodeSubscriber {
    static {
        System.loadLibrary("space_net");
        System.loadLibrary("java_wrapper");
    }
    private final long nativePtr;

    public NodeSubscriber(Node node) {
        nativePtr = newNodeSubscriber(node.getPointer());
    }

     private static native long newNodeSubscriber(long nodePtr);
}
