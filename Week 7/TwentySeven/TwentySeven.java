import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.function.Supplier;
import java.util.stream.Collectors;

public class TwentySeven {

    // Column class represents each "column" in the spreadsheet
    static class Column<T> {
        private T data;
        private Supplier<T> formula;

        public Column(T data, Supplier<T> formula) {
            this.data = data;
            this.formula = formula;
        }

        public T getData() {
            return data;
        }

        public void setData(T data) {
            this.data = data;
        }

        public void update() {
            if (formula != null) {
                data = formula.get();
            }
        }
    }

    // Method to load stop words
    private static Set<String> loadStopWords(String stopWordsFilePath) throws IOException {
        return new HashSet<>(Arrays.asList(
            new String(Files.readAllBytes(Paths.get(stopWordsFilePath))).split(",")
        ));
    }

    // Method to process and display the top 25 words from a file
    private static void processFile(String inputFilePath, Set<String> stopWords, Column<List<String>> allWords,
                                    Column<Set<String>> stopWordsColumn, Column<List<String>> nonStopWords,
                                    Column<Set<String>> uniqueWords, Column<Map<String, Integer>> counts,
                                    Column<List<Map.Entry<String, Integer>>> sortedData) throws IOException {

        // Load all words from the input file
        allWords.setData(Files.readAllLines(Paths.get(inputFilePath)).stream()
            .flatMap(line -> Arrays.stream(line.toLowerCase().split("[^a-z]+")))
            .filter(word -> word.length() >= 2)
            .collect(Collectors.toList()));

        // Set the stop words column
        stopWordsColumn.setData(stopWords);

        // Update the columns
        allWords.update();
        stopWordsColumn.update();
        nonStopWords.update();
        uniqueWords.update();
        counts.update();
        sortedData.update();

        // Print the top 25 words
        sortedData.getData().stream().limit(25).forEach(entry ->
            System.out.println(entry.getKey() + " - " + entry.getValue())
        );
    }

    public static void main(String[] args) throws IOException {
        // File paths for stop words
        String stopWordsFilePath = "../../stop_words.txt";

        // Define columns (data + formulas)
        Column<List<String>> allWords = new Column<>(null, null);
        Column<Set<String>> stopWordsColumn = new Column<>(null, null);
        Column<List<String>> nonStopWords = new Column<>(null, () ->
            allWords.getData().stream()
                .filter(word -> !stopWordsColumn.getData().contains(word))
                .collect(Collectors.toList())
        );
        Column<Set<String>> uniqueWords = new Column<>(null, () ->
            new HashSet<>(nonStopWords.getData())
        );
        Column<Map<String, Integer>> counts = new Column<>(null, () -> {
            Map<String, Integer> wordCounts = new HashMap<>();
            nonStopWords.getData().forEach(word -> wordCounts.merge(word, 1, Integer::sum));
            return wordCounts;
        });
        Column<List<Map.Entry<String, Integer>>> sortedData = new Column<>(null, () ->
            counts.getData().entrySet().stream()
                .sorted((e1, e2) -> e2.getValue().compareTo(e1.getValue()))
                .collect(Collectors.toList())
        );

        // Load stop words into a set
        Set<String> stopWords = loadStopWords(stopWordsFilePath);

        // Create a Scanner object to read user input
        Scanner scanner = new Scanner(System.in);

        // Run in an interactive loop
        while (true) {
            System.out.print("Enter the path to a text file to process (or type 'exit' to quit): ");
            String inputFilePath = scanner.nextLine().trim();

            if ("exit".equalsIgnoreCase(inputFilePath)) {
                System.out.println("Exiting the program...");
                break;
            }

            try {
                // Process the file and display the top 25 words
                processFile(inputFilePath, stopWords, allWords, stopWordsColumn, nonStopWords, uniqueWords, counts, sortedData);
            } catch (IOException e) {
                System.err.println("Error reading file: " + e.getMessage());
            }
        }

        scanner.close();
    }
}