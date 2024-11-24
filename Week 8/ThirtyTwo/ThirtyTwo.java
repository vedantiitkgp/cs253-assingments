import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;
import java.util.regex.*;
import java.util.stream.*;

public class ThirtyTwo {

    // Method to read the file content from the given path
    private static String readFile(String path) throws IOException {
        return Files.readString(Paths.get(path)); // Reads the entire file as a string
    }

    // Method to partition the input data into chunks of nLines (for parallel processing)
    private static List<String> partition(String data, int nLines) {
        List<String> lines = Arrays.asList(data.split("\n")); // Splits the data into lines
        List<String> chunks = new ArrayList<>();
        // Divide the lines into chunks of nLines
        for (int i = 0; i < lines.size(); i += nLines) {
            chunks.add(String.join("\n", lines.subList(i, Math.min(lines.size(), i + nLines)))); // Add each chunk of lines
        }
        return chunks; // Return the list of chunks
    }

    // Method to split the input text into words, filtering out stop words and non-alphabetic characters
    private static List<Map.Entry<String, Integer>> splitWords(String data, Set<String> stopWords) {
        // Define a regex pattern to remove all non-alphabetic characters
        Pattern pattern = Pattern.compile("[\\W_]+");
        return Arrays.stream(pattern.matcher(data).replaceAll(" ").toLowerCase().split(" ")) // Clean and split the text into words
                .filter(word -> !stopWords.contains(word) && !word.isEmpty()) // Filter out stop words and empty strings
                .map(word -> Map.entry(word, 1)) // Map each word to an entry with a count of 1
                .collect(Collectors.toList()); // Collect into a list of entries
    }

    // Method to regroup the word entries into 5 groups based on the first character of each word (a-e, f-j, etc.)
    private static Map<String, List<Map.Entry<String, Integer>>> regroup(List<List<Map.Entry<String, Integer>>> pairsList) {
        Map<String, List<Map.Entry<String, Integer>>> grouped = new ConcurrentHashMap<>();

        // 32.3: Grouping words alphabetically into five ranges for parallel counting
        pairsList.stream()
                .flatMap(List::stream) // Flatten the list of lists into a single stream of word-frequency pairs
                .forEach(pair -> {
                    String key = getGroupKey(pair.getKey()); // Get the group key based on the first character of the word
                    // Add the word to the corresponding group (with thread safety using synchronizedList)
                    grouped.computeIfAbsent(key, k -> Collections.synchronizedList(new ArrayList<>())).add(pair);
                });

        return grouped; // Return the grouped word entries
    }

    // Method to determine the group key based on the first letter of the word
    private static String getGroupKey(String word) {
        char c = word.charAt(0);
        if (c >= 'a' && c <= 'e') return "a-e";
        else if (c >= 'f' && c <= 'j') return "f-j";
        else if (c >= 'k' && c <= 'o') return "k-o";
        else if (c >= 'p' && c <= 't') return "p-t";
        else return "u-z"; // Words starting with u, v, w, x, y, z
    }

    // Method to count the frequency of words in a list of word entries (aggregating counts)
    private static List<Map.Entry<String, Integer>> countWords(List<Map.Entry<String, Integer>> pairs) {
        return pairs.stream()
                .collect(Collectors.groupingBy(Map.Entry::getKey, Collectors.summingInt(Map.Entry::getValue))) // Group by word and sum counts
                .entrySet()
                .stream() // Convert the map entries back to a stream
                .collect(Collectors.toList()); // Collect the entries into a list
    }

    // Method to sort the list of word-frequency pairs in descending order of frequency
    private static List<Map.Entry<String, Integer>> sort(List<Map.Entry<String, Integer>> wordFreqs) {
        return wordFreqs.stream()
                .sorted((a, b) -> b.getValue().compareTo(a.getValue())) // Sort by frequency in descending order
                .collect(Collectors.toList()); // Collect sorted results into a list
    }

    public static void main(String[] args) throws IOException {
        if (args.length < 1) {
            System.out.println("Usage: java ThirtyTwo <file_path>"); // If no file path is provided, print usage info
            return;
        }

        // Read the input data from the provided file
        String data = readFile(args[0]);

        // Read the stop words from the stop_words.txt file
        Set<String> stopWords = new HashSet<>(Arrays.asList(
                Files.readString(Paths.get("../../stop_words.txt")).split(",")));

        // Add single-letter alphabetic characters as stop words
        stopWords.addAll("abcdefghijklmnopqrstuvwxyz".chars().mapToObj(c -> String.valueOf((char) c)).toList());

        // Partition the input data into chunks of 200 lines for parallel processing
        List<String> chunks = partition(data, 200);

        // Process each chunk to split words and count frequencies
        List<List<Map.Entry<String, Integer>>> splits = chunks.stream()
                .map(chunk -> splitWords(chunk, stopWords)) // Split the chunk into words and count them
                .collect(Collectors.toList()); // Collect results into a list of lists

        // Regroup words into 5 groups based on the first letter of each word
        Map<String, List<Map.Entry<String, Integer>>> grouped = regroup(splits);

        // For each group, count the words in parallel and combine the results
        List<Map.Entry<String, Integer>> wordFreqs = grouped.values().stream()
                .parallel() // Process groups in parallel
                .flatMap(group -> countWords(group).stream()) // Count words in each group
                .collect(Collectors.toList()); // Collect the word-frequency pairs into a list

        // Sort the word-frequency pairs by frequency in descending order
        List<Map.Entry<String, Integer>> sortedWordFreqs = sort(wordFreqs);

        sortedWordFreqs.stream().limit(25).forEach(entry ->
                System.out.println(entry.getKey() + " - " + entry.getValue())); // Print each word with its frequency
    }
}