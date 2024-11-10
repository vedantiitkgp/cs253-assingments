import java.io.IOException;
import java.util.List;

public interface WordExtractor {
    List<String> extractWords(String filePath, String stopWordPath) throws IOException;
}
  