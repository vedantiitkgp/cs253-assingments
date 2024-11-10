import java.io.*;
import java.util.*;
import java.nio.file.Files;
import java.nio.file.Paths;

public class WordExtractorImpl2 implements WordExtractor {
    public WordExtractorImpl2() {}
    
    public List<String> extractWords(String filePath, String stopWordPath) throws IOException {
        Set<String> stopWords = new HashSet<>();
        
        // Add all single letters a to z as stopwords
        for (char c = 'a'; c <= 'z'; c++) {
            stopWords.add(String.valueOf(c));
        }
        
        // Read stopwords from file
        if (stopWordPath != null && !stopWordPath.isEmpty()) {
            Scanner f = new Scanner(new File(stopWordPath), "UTF-8");
            try {
                f.useDelimiter(",");
                while (f.hasNext()) {
                    stopWords.add(f.next());
                }
            } finally {
                f.close();
            }
        }

        List<String> words = new ArrayList<>();
        
        // Process the main file line by line
        try (BufferedReader reader = new BufferedReader(new FileReader(filePath))) {
            String line;
            while ((line = reader.readLine()) != null) {
                String[] wordArray = line.split("[\\W_]+");
                for (String word : wordArray) {
                    String lowercaseWord = word.toLowerCase().trim();
                    if (!lowercaseWord.isEmpty() && !stopWords.contains(lowercaseWord)) {
                        words.add(lowercaseWord);
                    }
                }
            }
        }
        
        return words;
    }
}