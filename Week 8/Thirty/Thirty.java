import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.concurrent.*;
import java.util.regex.*;
import java.util.stream.*;

public class Thirty {
    // Shared data spaces
    private static final BlockingQueue<String> wordSpace = new LinkedBlockingQueue<>();
    private static final BlockingQueue<Map<String, Integer>> freqSpace = new LinkedBlockingQueue<>();

    // Set of stopwords
    private static final Set<String> stopWords = new HashSet<>();

    public static void main(String[] args) throws IOException, InterruptedException {
        if (args.length < 1) {
            System.out.println("Usage: java Thirty <input_file>");
            return;
        }

        // Load stop words
        stopWords.addAll(Arrays.asList(
            new String(Files.readAllBytes(Paths.get("../../stop_words.txt"))).split(",")
        ));

        // Populate the word space
        Pattern wordPattern = Pattern.compile("[a-z]{2,}");
        String fileContent = new String(Files.readAllBytes(Paths.get(args[0]))).toLowerCase();
        Matcher matcher = wordPattern.matcher(fileContent);
        while (matcher.find()) {
            wordSpace.put(matcher.group());
        }

        // Create and start worker threads
        int numWorkers = 5;
        List<Thread> workers = new ArrayList<>();
        for (int i = 0; i < numWorkers; i++) {
            Thread worker = new Thread(Thirty::processWords);
            workers.add(worker);
            worker.start();
        }

        // Wait for workers to finish
        for (Thread worker : workers) {
            worker.join();
        }

        // Merge results from frequency space
        Map<String, Integer> wordFreqs = new HashMap<>();
        while (!freqSpace.isEmpty()) {
            Map<String, Integer> partialFreqs = freqSpace.poll();
            if (partialFreqs != null) {
                for (Map.Entry<String, Integer> entry : partialFreqs.entrySet()) {
                    wordFreqs.merge(entry.getKey(), entry.getValue(), Integer::sum);
                }
            }
        }

        // Print top 25 words
        wordFreqs.entrySet().stream()
            .sorted((e1, e2) -> e2.getValue().compareTo(e1.getValue()))
            .limit(25)
            .forEach(entry -> System.out.println(entry.getKey() + " - " + entry.getValue()));
    }

    // Worker function to process words and store partial results in freqSpace
    private static void processWords() {
        Map<String, Integer> wordFreqs = new HashMap<>();
        try {
            while (true) {
                String word = wordSpace.poll(1, TimeUnit.SECONDS); // Wait for a word
                if (word == null) break; // Exit when no more words

                if (!stopWords.contains(word)) {
                    wordFreqs.merge(word, 1, Integer::sum);
                }
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
        // Add partial results to frequency space
        freqSpace.add(wordFreqs);
    }
}
