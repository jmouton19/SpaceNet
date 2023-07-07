package com.example;

public class PayloadMessage {
    static {
        System.loadLibrary("space_net");
        System.loadLibrary("java_wrapper");
    }
    private final long nativePtr;

      public PayloadMessage(long ptr) {
            nativePtr = ptr;
        }
    public byte[] getPayload() {
            return getPayload(nativePtr);
        }

    public String getSenderId() {
          return getSenderId(nativePtr);
     }

    public String getTopic() {
          return getTopic(nativePtr);
    }

    public void flush() {
         flush(nativePtr);
    }

    private native byte[] getPayload(long nodePtr);
    private native String getSenderId(long nodePtr);
    private native String getTopic(long nodePtr);
    private native void flush(long nodePtr);
}
