package Twenty;
import java.util.List;
import java.util.ArrayList;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class WordExtractorImpl1 implements WordExtractor {
    @Override
    public List<String> extractWords(String filePath) throws IOException {
        List<String> words = new ArrayList<>();
        String content = new String(Files.readAllBytes(Paths.get(filePath)));
        for (String word : content.split("[\\W_]+")) {
            words.add(word.toLowerCase());
        }
        return words;
    }
}