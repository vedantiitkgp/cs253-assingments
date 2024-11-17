import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

// gets lines -> convert them to words using split -> filter stop words -> increment count
public class TwentyEight {

    // A generator-like method for lines (using Streams)
    private static Stream<String> lines(String filename) throws IOException {
        return Files.lines(Paths.get(filename));  // Stream of lines from file
    }

    // A generator-like method to split a line into words and stream them
    private static Stream<String> wordsFromLine(String line) {
        return Arrays.stream(line.toLowerCase().split("[^a-z0-9]+"))  // Split line into words
            .filter(word -> !word.isEmpty());  // Filter out empty strings
    }

    // A method to yield words from the entire file by processing lines
    private static Stream<String> allWords(String filename) throws IOException {
        return lines(filename)  // Stream of lines
            .flatMap(TwentyEight::wordsFromLine);  // Flatten each line's words into a single stream
    }

    // A method to filter out stop words and return a stream of non-stop words
    private static Stream<String> nonStopWords(String filename) throws IOException {
        // Load stop words from the file
        Set<String> stopWords = new HashSet<>(Arrays.asList(
            new String(Files.readAllBytes(Paths.get("../../stop_words.txt")))
                .toLowerCase().split(",")));
        stopWords.addAll(Arrays.asList("a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
            "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"));

        // Stream non-stop words from all words
        return allWords(filename)
            .filter(word -> !stopWords.contains(word));  // Filter out stop words
    }

    // A method to count word frequencies and return a stream of sorted word frequencies
    private static Stream<Map.Entry<String, Integer>> countAndSort(String filename) throws IOException {
        // Create a map to store word frequencies
        Map<String, Integer> freqs = new HashMap<>();
        
        // Stream over non-stop words and count frequencies
        nonStopWords(filename)
            .forEach(word -> freqs.put(word, freqs.getOrDefault(word, 0) + 1));

        // Sort and return the frequency entries
        return freqs.entrySet().stream()
            .sorted((e1, e2) -> Integer.compare(e2.getValue(), e1.getValue()));  // Sort by frequency descending
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Usage: java TwentyEight <filename>");
            return;
        }

        String filename = args[0];
        try {
            // Stream word frequencies and print the top 25
            countAndSort(filename)
                .limit(25)  // Limit to the top 25 most frequent words
                .forEach(entry -> System.out.println(entry.getKey() + " - " + entry.getValue()));
        } catch (IOException e) {
            System.err.println("Error reading file: " + e.getMessage());
        }
    }
}