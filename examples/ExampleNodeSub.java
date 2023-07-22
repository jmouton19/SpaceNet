import com.example.Node;
import com.example.Subscriber;
import com.example.PayloadMessage;

public class ExampleNodeSub {
    public static void main(String[] args) {
        Node node = new Node("network_1");
         String zid = node.getZid();
         System.out.println("Node zid: " + zid);
         node.leaveOnKey('q');
         node.join(69.0,69.0);

         Subscriber sub= new Subscriber();
         String topic= node.getCluster()+"/test";
         sub.subscribe(topic);

         while (true) {
               try {
                    Thread.sleep(1000);
               } catch (InterruptedException e) {
                    e.printStackTrace();
               }

               PayloadMessage message = new PayloadMessage(sub.receive());
               if(message.getPayload().length!=0)
                System.out.println(message.getPayload()[0]+", length "+message.getPayload().length+" topic "+message.getTopic());
               message.flush();
         }
    }
}
