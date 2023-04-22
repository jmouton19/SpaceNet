public class BootNode {
    static {
        System.loadLibrary("java_wrapper");
    }
    private final long nativePtr;

    public BootNode(String clusterName) {
        nativePtr = newBoot(clusterName);
    }

    public String getZid() {
        return getZid(nativePtr);
    }

    public void run() {
            run(nativePtr);
        }

    private native String getZid(long nodePtr);

    private native void run(long nodePtr);

    private static native long newBoot(String clusterName);


}
