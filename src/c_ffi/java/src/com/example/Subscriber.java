package com.example;

public class Subscriber {
    static {
        System.loadLibrary("space_net");
        System.loadLibrary("java_wrapper");
    }
    private final long nativePtr;

    public Subscriber() {
        nativePtr = newSubscriber();
    }

    public void subscribe(String topic) {
            subscribe(nativePtr,topic);
        }

    public long receive() {
             return receive(nativePtr);
          }

     private static native long newSubscriber();

     private static native void subscribe(long subPtr,String topic);

     private static native long receive(long subPtr);
}
