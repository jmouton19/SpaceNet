import com.example.Node;

public class ExampleNode {
    public static void main(String[] args) {
        Node node = new Node("test_cluster",69.0,69.0);
         String zid = node.getZid();
         System.out.println("Node zid: " + zid);
         node.leaveOnKey('q');
         node.join();

         while (true) {

         }
    }
}
