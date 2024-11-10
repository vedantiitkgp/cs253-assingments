package Twenty;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;

public class Twenty {
    public static void main(String[] args) throws Exception {
        if (args.length != 1) {
            System.out.println("Usage: java Main <path_to_text_file>");
            return;
        }

        // Load plugins based on the configuration file
        Properties config = new Properties();
        try (InputStream input = new FileInputStream("config.ini")) {
            config.load(input);
        }

        String wordExtractorClass = config.getProperty("Plugins.words");
        String wordFreqCalculatorClass = config.getProperty("Plugins.frequencies");

        // Dynamically load the classes
        WordExtractor wordExtractor = (WordExtractor) Class.forName(wordExtractorClass).getDeclaredConstructor().newInstance();
        WordFrequencyCalculator wordFreqCalculator = (WordFrequencyCalculator) Class.forName(wordFreqCalculatorClass).getDeclaredConstructor().newInstance();

        // Extract words and calculate frequencies
        List<String> words = wordExtractor.extractWords(args[0]);
        List<Map.Entry<String, Integer>> wordFreqs = wordFreqCalculator.getTop25(words);

        // Display top 25 words
        for (Map.Entry<String, Integer> entry : wordFreqs) {
            System.out.println(entry.getKey() + " - " + entry.getValue());
        }
    }
}
