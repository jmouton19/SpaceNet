import com.example.BootNode;

public class ExampleBoot {
    public static void main(String[] args) {
        BootNode boot_node = new BootNode("test_cluster",true);
         String zid = boot_node.getZid();
         System.out.println("Boot node zid: " + zid);

         while (true) {
            //boot_node.run();
         }
    }
}
