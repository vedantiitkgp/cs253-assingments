import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.stream.Collectors;

public class WordFrequencyCalculatorImpl2 implements WordFrequencyCalculator {
    public WordFrequencyCalculatorImpl2(){}
    
    public List<Entry<String, Integer>> getTop25(List<String> words) {
        Map<String, Integer> wordFrequencies = new HashMap<>();

        // Count each word's occurrences
        for (String word : words) {
            wordFrequencies.put(word, wordFrequencies.getOrDefault(word, 0) + 1);
        }

        // Sort by frequency in descending order; if frequencies are the same, sort lexicographically
        return wordFrequencies.entrySet()
            .stream()
            .sorted((e1, e2) -> {
                int freqCompare = e2.getValue().compareTo(e1.getValue());
                if (freqCompare == 0) {
                    return e1.getKey().compareTo(e2.getKey());
                }
                return freqCompare;
            })
            .limit(25)
            .collect(Collectors.toList());
    }
}
