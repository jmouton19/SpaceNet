import com.example.Node;
import com.example.NodeSubscriber;

public class ExampleNode3 {
    public static void main(String[] args) {
        Node node = new Node("network_1");
         String zid = node.getZid();
         System.out.println("Node zid: " + zid);
         node.leaveOnKey('q');
         node.join(69.0,69.0);

         NodeSubscriber sub= new NodeSubscriber(node);
         sub.subscribe("pog");

         while (true) {
               try {
                    Thread.sleep(1000);
               } catch (InterruptedException e) {
                    e.printStackTrace();
               }
               byte[] output = sub.receive();
               if(output.length!=0)
                System.out.println(output[0]+", length "+output.length);
         }
    }
}
