import com.example.BootNode;

public class ExampleBoot {
    public static void main(String[] args) {
        BootNode boot_node = new BootNode("network_1",true);
         String zid = boot_node.getZid();
         System.out.println("Boot node zid: " + zid);

         while (true) {
         }
    }
}
