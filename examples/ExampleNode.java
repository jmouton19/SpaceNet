
public class ExampleNode {
    public static void main(String[] args) {
        Node node = new Node("test_cluster");
         String zid = node.getZid();
         System.out.println("Node zid: " + zid);
         node.leaveOnKey('q');

         while (node.getStatus()!=Node.NodeStatus.Offline) {
         node.run();
         }
    }
}
