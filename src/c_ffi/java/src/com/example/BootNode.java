package com.example;

public class BootNode {
    static {
        System.loadLibrary("space_net");
        System.loadLibrary("java_wrapper");
    }
    private final long nativePtr;

    public BootNode(String clusterName,boolean centralized_voronoi) {
        nativePtr = newBoot(clusterName,centralized_voronoi);
    }

    public String getZid() {
        return getZid(nativePtr);
    }

    public void free() {
            free(nativePtr);
        }

    private native String getZid(long nodePtr);

     private native void free(long nodePtr);

    private static native long newBoot(String clusterName,boolean centralized_voronoi);


}
