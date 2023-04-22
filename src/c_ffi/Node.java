public class Node {
    static {
        System.loadLibrary("java_wrapper");
    }
    private final long nativePtr;

    public Node(String clusterName) {
        nativePtr = newNode(clusterName);
    }

    public String getZid() {
        return getZid(nativePtr);
    }

     public void run() {
                 run(nativePtr);
            }

    public native String getZid(long nodePtr);

    private native void run(long nodePtr);

    private static native long newNode(String clusterName);


}
