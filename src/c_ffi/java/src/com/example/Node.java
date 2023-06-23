package com.example;

public class Node {
    static {
        System.loadLibrary("space_net");
        System.loadLibrary("java_wrapper");
    }

    public enum NodeStatus {
        Online,
        Leaving,
        Joining,
        Offline
    }
    private final long nativePtr;

    public Node(String clusterName) {
        nativePtr = newNode(clusterName);
    }

    public String getZid() {
        return getZid(nativePtr);
    }

     public long getPointer() {
            return nativePtr;
     }

     public String closestNeighbour(double x, double y) {
            return closestNeighbour(nativePtr,x,y);
        }

     public void join(double x,double y) {
                 join(nativePtr,x,y);
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

    private native void join(long nodePtr,double x, double y);

    private native void leave(long nodePtr);

//     private native void send_message(long nodePtr);

    private native void leaveOnKey(long nodePtr, char key);

    private static native long newNode(String clusterName);

    private native int getStatus(long nodePtr);

    private native int isNeighbour(long nodePtr, String zid);

    private native int isInPolygon(long nodePtr,double x, double y);

    private native String closestNeighbour(long nodePtr,double x, double y);


}
