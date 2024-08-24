import java.nio.file.Path;
import java.nio.file.Paths;
import java.io.FileReader;
import java.io.BufferedReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;

public class santaList{
        

  public static ArrayList<String> readFile(BufferedReader reader) throws IOException{
    ArrayList<String> readList = new ArrayList<>();
    String tmp;

    while ((tmp = reader.readLine()) != null){
      readList.add(tmp);
    }
    return readList;
  }

  public static void main(String[] args){
    int maxElf = 0; 
    ArrayList<String> lines = new ArrayList<>();
    ArrayList<Integer> elfList = new ArrayList<>();

    // get current directory
    String cwd = System.getProperty("user.dir");
    System.out.println(cwd);
    // assume that the input file is called 'santaList.txt', concatenate onto the cwd
    String pCwd = String.format("%s/santaList.txt", cwd);

    try (FileReader fr = new FileReader(pCwd);
        BufferedReader bf = new BufferedReader(fr))
    {
      lines = readFile(bf);
    } 
    catch(IOException e ){
      System.out.println(String.format("There was an error: %s", e.toString()));
    }
    
    int currTotal = 0;
    for(String word : lines){
      if ("".equals(word)){
        elfList.add(currTotal);     
        currTotal = 0;
      }
      else{
        currTotal += Integer.valueOf(word); 
      }
    }

    Collections.sort(elfList, Collections.reverseOrder());
  
    System.out.println(String.format("STACKED ELF: %d ", elfList.get(0)));
    int locSum = 0;
    
    for (int i = 0; i < 3; i++){
      locSum += Integer.valueOf(elfList.get(i));
    }
    System.out.println(String.format("Stacked ELVES: $%d: ", locSum));
  }
}
