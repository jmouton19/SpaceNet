package com.example;

public class Node {
    static {
        System.loadLibrary("java_wrapper");
    }

    public enum NodeStatus {
        Online,
        Leaving,
        Joining,
        Offline
    }
    private final long nativePtr;

    public Node(String clusterName,double x, double y) {
        nativePtr = newNode(clusterName,x,y);
    }

    public String getZid() {
        return getZid(nativePtr);
    }

     public void join() {
                 join(nativePtr);
            }

     public void leave() {
          leave(nativePtr);
     }

     public void leaveOnKey(char key) {
         leaveOnKey(nativePtr, key);
     }

     public NodeStatus getStatus() {
           int status =  getStatus(nativePtr);
           return NodeStatus.values()[status];
     }

     public int isNeighbour(String zid) {
            return isNeighbour(nativePtr,zid);
     }

     public int isInPolygon(double x, double y) {
            return isInPolygon(nativePtr,x,y);
     }

    private native String getZid(long nodePtr);

    private native void join(long nodePtr);

    private native void leave(long nodePtr);

    private native void leaveOnKey(long nodePtr, char key);

    private static native long newNode(String clusterName,double x, double y);

    private native int getStatus(long nodePtr);

    private native int isNeighbour(long nodePtr, String zid);

    private native int isInPolygon(long nodePtr,double x, double y);


}
