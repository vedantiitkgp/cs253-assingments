import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.stream.Collectors;

public class WordFrequencyCalculatorImpl1 implements WordFrequencyCalculator {
    public WordFrequencyCalculatorImpl1(){
    }
    
    public List<Entry<String, Integer>> getTop25(List<String> words) {
        Map<String, Integer> wordFrequencies = new HashMap<>();

        // Count each word's occurrences
        for (String word : words) {
            wordFrequencies.put(word, wordFrequencies.getOrDefault(word, 0) + 1);
        }

        // Sort by frequency in descending order and return the top 25
        return wordFrequencies.entrySet()
            .stream()
            .sorted((e1, e2) -> e2.getValue().compareTo(e1.getValue()))
            .limit(25)
            .collect(Collectors.toList());
    }
}