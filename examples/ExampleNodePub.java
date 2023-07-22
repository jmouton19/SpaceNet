import com.example.Node;
import com.example.Subscriber;

public class ExampleNodePub {
    public static void main(String[] args) {
        Node node = new Node("network_1");
         String zid = node.getZid();
         System.out.println("Node zid: " + zid);
         node.leaveOnKey('q');
         node.join(69.0,69.0);

         int i = 1;
            while (i <= 5){
                byte[] rawData = new byte[10];
                rawData[0]=(byte)i;
                String topic=node.getCluster()+"/test";
                node.sendMessage(rawData, topic);
                System.out.println(rawData[0]);
                i += 1;
            }

         while (true) {

         }
    }
}
