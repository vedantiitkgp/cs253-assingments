import java.util.List;
import java.util.Scanner;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.Set;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class WordExtractorImpl1 implements WordExtractor {
    public WordExtractorImpl1() {}

    public List<String> extractWords(String filePath, String stopWordPath) throws IOException {
        // Initialize stopwords set
        Set<String> stopWords = new HashSet<>();
        
        // Add all single letters a to z as stopwords
        for (char c = 'a'; c <= 'z'; c++) {
            stopWords.add(String.valueOf(c));
        }
        
        // Read additional stopwords from file
        Scanner f = new Scanner(new File(stopWordPath), "UTF-8");
        try {
            f.useDelimiter(",");
            while (f.hasNext()) {
                stopWords.add(f.next());
            }
        } finally {
            f.close();
        }

        // Read and process the main file
        List<String> words = new ArrayList<>();
        String content = new String(Files.readAllBytes(Paths.get(filePath)));
        System.out.println(stopWords.size());
        // Split content into words and filter out stopwords
        for (String word : content.split("[\\W_]+")) {
            String lowercaseWord = word.toLowerCase().trim();
            if (!lowercaseWord.isEmpty() && !stopWords.contains(lowercaseWord)) {
                words.add(lowercaseWord);
            }
        }
        
        return words;
    }
}