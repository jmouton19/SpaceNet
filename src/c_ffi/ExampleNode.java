
public class ExampleNode {
    public static void main(String[] args) {
        Node node = new Node("test_cluster");
         String zid = node.getZid(node.nativePtr);
         System.out.println("Node zid: " + zid);
    }
}
