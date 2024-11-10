import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.stream.Collectors;

// Capsule for counting word frequencies
class WordCounter {
    private Map<String, Integer> wordFrequencies;

    public WordCounter() {
        this.wordFrequencies = new HashMap<>();
    }

    public void countWords(List<String> words) {
        for (String word : words) {
            wordFrequencies.put(word, wordFrequencies.getOrDefault(word, 0) + 1);
        }
    }

    public Map<String, Integer> getWordFrequencies() {
        return wordFrequencies;
    }
}

// Capsule for sorting word frequencies
class WordFrequencySorter {
    public List<Entry<String, Integer>> sort(Map<String, Integer> wordFrequencies) {
        return wordFrequencies.entrySet()
            .stream()
            .sorted((e1, e2) -> e2.getValue().compareTo(e1.getValue())) // Sort in descending order
            .limit(25) // Limit to top 25
            .collect(Collectors.toList());
    }
}

// Capsule for coordinating the word frequency calculation
public class WordFrequencyCalculatorImpl1 implements WordFrequencyCalculator {
    private WordCounter wordCounter;
    private WordFrequencySorter wordFrequencySorter;

    public WordFrequencyCalculatorImpl1() {
        this.wordCounter = new WordCounter();
        this.wordFrequencySorter = new WordFrequencySorter();
    }

    public List<Entry<String, Integer>> getTop25(List<String> words) {
        // Dispatch message to count words
        wordCounter.countWords(words);
        
        // Dispatch message to sort the word frequencies and get top 25
        return wordFrequencySorter.sort(wordCounter.getWordFrequencies());
    }
}
