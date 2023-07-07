import com.example.Node;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.ObjectOutputStream;
import java.io.ObjectInputStream;
import java.io.Serializable;
import java.io.ByteArrayInputStream;

public class ExampleNode2 {

    public static class Benny implements java.io.Serializable {
            public static int age=21;
            public static String gf ="Begga";

    }

       public static byte[] serializeObject(Object obj) {
            ByteArrayOutputStream bos = new ByteArrayOutputStream();
            try (ObjectOutputStream oos = new ObjectOutputStream(bos)) {
                oos.writeObject(obj);
                oos.flush();
                return bos.toByteArray();
            } catch (IOException e) {
                e.printStackTrace();
            }
            return null;
        }

          public static Benny deserializeObject(byte[] bytes) {
                ByteArrayInputStream bis = new ByteArrayInputStream(bytes);
                try (ObjectInputStream ois = new ObjectInputStream(bis)) {
                    return (Benny) ois.readObject();
                } catch (IOException | ClassNotFoundException e) {
                    e.printStackTrace();
                }
                return null;
            }


    public static void main(String[] args) {
        Node node = new Node("network_1");
         String zid = node.getZid();
         System.out.println("Node zid: " + zid);
         node.leaveOnKey('q');
         node.join(69.0,69.0);


        Benny ben = new Benny();
        System.out.println(ben.age);
        byte[] serializedBytes = serializeObject(ben);
        System.out.println("Serialized bytes: " + serializedBytes);

        Benny deserializedBen = deserializeObject(serializedBytes);
        System.out.println("Deserialized Benny object: " + deserializedBen.age + ", " + deserializedBen.gf);

//
//         while(true){
//         if(node.getNeighbours().length!=0)
//
//             for (String neighbour: node.getNeighbours()){
//             System.out.println("NEIGHBOURS: " +neighbour);
//             }
//         }


         int i = 1;
            while (i <= 1){
                byte[] rawData = new byte[10];
                rawData[0]=(byte) 0x04;
                node.sendMessage(rawData, "pog");
                System.out.println(rawData[0]);
                i += 1;
            }

         while (true) {

         }
    }
}
