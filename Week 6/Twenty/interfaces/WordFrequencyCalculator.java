package Twenty;
import java.util.List;
import java.util.Map;

public interface WordFrequencyCalculator {
    /**
     * Calculates the top 25 words by frequency from a given list of words.
     *
     * @param words A list of words to calculate frequencies from.
     * @return A list of word-frequency pairs (each entry represented as a Map.Entry),
     *         sorted by frequency in descending order.
     */
    List<Map.Entry<String, Integer>> getTop25(List<String> words);
}
