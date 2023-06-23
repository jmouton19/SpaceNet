import com.example.Node;

public class ExampleNode3 {
    public static void main(String[] args) {
        Node node = new Node("test_cluster");
         String zid = node.getZid();
         System.out.println("Node zid: " + zid);
         node.leaveOnKey('q');
         node.join(69.0,69.0);

         while (true) {

         }
    }
}
