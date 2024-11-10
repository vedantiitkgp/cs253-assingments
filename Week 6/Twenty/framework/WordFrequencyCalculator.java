import java.util.List;
import java.util.Map;

public interface WordFrequencyCalculator {
    List<Map.Entry<String, Integer>> getTop25(List<String> words);
}
