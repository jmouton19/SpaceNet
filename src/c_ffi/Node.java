public class Node {
    static {
        System.loadLibrary("java_wrapper");
    }
    public long nativePtr;

    public Node(String clusterName) {
        nativePtr = newNode(clusterName);
    }
    public native String getZid(long nodePtr);

    private static native long newNode(String clusterName);
}
