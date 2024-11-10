import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.stream.Collectors;

public class WordFrequencyCalculatorImpl2 implements WordFrequencyCalculator {

    public WordFrequencyCalculatorImpl2() {}

    // Function to count the frequency of each word
    private Map<String, Integer> countWordFrequencies(List<String> words) {
        return words.stream()
                    .collect(Collectors.toMap(word -> word, word -> 1, Integer::sum));
    }

    // Function to sort the word frequencies by frequency in descending order and lexicographically for ties
    private List<Entry<String, Integer>> sortByFrequency(Map<String, Integer> wordFrequencies) {
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

    // Pipeline function to get the top 25 most frequent words
    public List<Entry<String, Integer>> getTop25(List<String> words) {
        // Apply the pipeline: count frequencies -> sort by frequency -> get top 25
        return sortByFrequency(countWordFrequencies(words));
    }
}
