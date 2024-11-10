package Twenty;
import java.io.IOException;
import java.util.List;

public interface WordExtractor {
    /**
     * Extracts words from the specified file.
     * 
     * @param filePath The path to the file to be read.
     * @return A list of words extracted from the file.
     * @throws IOException If there is an issue reading the file.
     */
    List<String> extractWords(String filePath) throws IOException;
}