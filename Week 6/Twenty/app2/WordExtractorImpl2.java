import java.io.*;
import java.util.*;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.stream.Collectors;

public class WordExtractorImpl2 implements WordExtractor {

    public WordExtractorImpl2() {}

    // Function to load stopwords from file and add default stopwords
    private Set<String> loadStopWords(String stopWordPath) throws IOException {
        Set<String> stopWords = new HashSet<>();
        
        // Add all single letters a to z as stopwords
        for (char c = 'a'; c <= 'z'; c++) {
            stopWords.add(String.valueOf(c));
        }

        // Read stopwords from file
        if (stopWordPath != null && !stopWordPath.isEmpty()) {
            List<String> fileStopWords = Files.readAllLines(Paths.get(stopWordPath));
            stopWords.addAll(fileStopWords.stream()
                                          .flatMap(line -> Arrays.stream(line.split(",")))
                                          .collect(Collectors.toSet()));
        }

        return stopWords;
    }

    // Function to process a line into words (splitting, trimming, and lowercasing)
    private List<String> processLine(String line) {
        return Arrays.stream(line.split("[\\W_]+"))
                     .map(word -> word.toLowerCase().trim())
                     .filter(word -> !word.isEmpty())
                     .collect(Collectors.toList());
    }

    // Function to filter words based on stopwords
    private List<String> filterStopWords(List<String> words, Set<String> stopWords) {
        return words.stream()
                    .filter(word -> !stopWords.contains(word))
                    .collect(Collectors.toList());
    }

    // Function to read and process the file
    private List<String> readAndProcessFile(String filePath) throws IOException {
        try (BufferedReader reader = new BufferedReader(new FileReader(filePath))) {
            return reader.lines()
                         .flatMap(line -> processLine(line).stream())
                         .collect(Collectors.toList());
        }
    }

    // Pipeline function for extracting words
    public List<String> extractWords(String filePath, String stopWordPath) throws IOException {
        // Load stopwords
        Set<String> stopWords = loadStopWords(stopWordPath);

        // Read and process the file
        List<String> words = readAndProcessFile(filePath);

        // Filter out stopwords
        return filterStopWords(words, stopWords);
    }
}