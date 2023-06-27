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

    public void subscribe(String topic) {
            System.out.println("INSIDE JAV CALL "+nativePtr);
            subscribe(nativePtr,topic);
        }

    public byte[] receive() {
             return receive(nativePtr);
          }

     private static native long newNodeSubscriber(long subPtr);

     private static native void subscribe(long subPtr,String topic);

     private static native byte[] receive(long subPtr);
}
