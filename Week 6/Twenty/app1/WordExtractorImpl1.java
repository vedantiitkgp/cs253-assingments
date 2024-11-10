import java.util.List;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.Set;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Scanner;

// Capsule for managing stopwords
class StopWordManager {
    private Set<String> stopWords;

    public StopWordManager(String stopWordPath) throws IOException {
        this.stopWords = new HashSet<>();

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
    }

    public boolean isStopWord(String word) {
        return stopWords.contains(word);
    }

    public int size() {
        return stopWords.size();
    }
}

// Capsule for reading file content
class FileReader {
    public String readFile(String filePath) throws IOException {
        return new String(Files.readAllBytes(Paths.get(filePath)));
    }
}

// Capsule for word extraction logic
public class WordExtractorImpl1 implements WordExtractor {
    private StopWordManager stopWordManager;
    private FileReader fileReader;

    public WordExtractorImpl1() {}

    // Override the method with two parameters, as expected by the interface
    @Override
    public List<String> extractWords(String filePath, String stopWordPath) throws IOException {
        // Initialize StopWordManager with the provided stop word path
        this.stopWordManager = new StopWordManager(stopWordPath);
        this.fileReader = new FileReader();

        // Read content from file
        String content = fileReader.readFile(filePath);

        List<String> words = new ArrayList<>();
        
        // Split content into words and filter out stopwords
        for (String word : content.split("[\\W_]+")) {
            String lowercaseWord = word.toLowerCase().trim();
            if (!lowercaseWord.isEmpty() && !stopWordManager.isStopWord(lowercaseWord)) {
                words.add(lowercaseWord);
            }
        }

        return words;
    }
}