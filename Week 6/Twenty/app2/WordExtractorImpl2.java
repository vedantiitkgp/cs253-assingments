package Twenty;
import java.io.*;
import java.util.*;

public class WordExtractorImpl2 implements WordExtractor {
    @Override
    public List<String> extractWords(String filePath) throws IOException {
        List<String> words = new ArrayList<>();
        // Different word extraction strategy
        try (BufferedReader reader = new BufferedReader(new FileReader(filePath))) {
            String line;
            while ((line = reader.readLine()) != null) {
                String[] wordArray = line.split("[\\W_]+");
                for (String word : wordArray) {
                    words.add(word.toLowerCase());
                }
            }
        }
        return words;
    }
}