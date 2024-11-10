import java.io.*;
import java.lang.reflect.*;
import java.util.*;

public class Seventeen {
    /*
     * The main function
     */
    public static void main(String[] args) throws Exception {
        // Prompt user for class name and display introspective information
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter the class name (e.g., DataStorageManager): ");
        String className = scanner.nextLine();
        displayClassInfo(className);
        System.out.println("\n------------------------------------------------------------\n");
        
        // Run the WordFrequencyController
        new WordFrequencyController(args[0]).run();
    }
    
    /*
     * Function to display class fields, methods, superclasses, and interfaces using reflection.
     */
    public static void displayClassInfo(String className) {
        try {
            Class<?> cls = Class.forName(className);
            
            System.out.println("Class: " + cls.getName());

            // Display fields
            System.out.println("Fields:");
            for (Field field : cls.getDeclaredFields()) {
                System.out.println(" - " + field.getName() + " : " + field.getType().getName());
            }

            // Display methods
            System.out.println("Methods:");
            for (Method method : cls.getDeclaredMethods()) {
                System.out.println(" - " + method.getName() + "()");
            }

            // Display superclass
            System.out.println("Superclass: " + cls.getSuperclass().getName());

            // Display interfaces
            System.out.println("Interfaces:");
            for (Class<?> iface : cls.getInterfaces()) {
                System.out.println(" - " + iface.getName());
            }
        } catch (ClassNotFoundException e) {
            System.out.println("Class not found: " + className);
        }
    }
}

/*
 * Abstract class with introspection capabilities.
 */
abstract class TFExercise {
    public String getInfo() {
        return this.getClass().getName();
    }
}

/*
 * Controller class that coordinates other components.
 */
class WordFrequencyController extends TFExercise {
    private DataStorageManager storageManager;
    private StopWordManager stopWordManager;
    private WordFrequencyManager wordFreqManager;

    public WordFrequencyController(String pathToFile) throws IOException {
        this.storageManager = new DataStorageManager(pathToFile);
        this.stopWordManager = new StopWordManager();
        this.wordFreqManager = new WordFrequencyManager();
    }

    public void run() {
        try {
            // Use reflection to invoke methods on each component
            Method getWordsMethod = DataStorageManager.class.getMethod("getWords");
            List<String> words = (List<String>) getWordsMethod.invoke(storageManager);

            Method isStopWordMethod = StopWordManager.class.getMethod("isStopWord", String.class);
            Method incrementCountMethod = WordFrequencyManager.class.getMethod("incrementCount", String.class);
            Method sortedMethod = WordFrequencyManager.class.getMethod("sorted");

            for (String word : words) {
                if (!(boolean) isStopWordMethod.invoke(stopWordManager, word)) {
                    incrementCountMethod.invoke(wordFreqManager, word);
                }
            }

            List<WordFrequencyPair> sortedWords = (List<WordFrequencyPair>) sortedMethod.invoke(wordFreqManager);
            int numWordsPrinted = 0;
            for (WordFrequencyPair pair : sortedWords) {
                System.out.println(pair.getWord() + " - " + pair.getFrequency());
                numWordsPrinted++;
                if (numWordsPrinted >= 25) break;
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}

/** Models the contents of the file. */
class DataStorageManager extends TFExercise {
    private List<String> words;

    public DataStorageManager(String pathToFile) throws IOException {
        this.words = new ArrayList<>();
        
        Scanner f = new Scanner(new File(pathToFile), "UTF-8");
        try {
            f.useDelimiter("[\\W_]+");
            while (f.hasNext()) {
                this.words.add(f.next().toLowerCase());
            }
        } finally {
            f.close();
        }
    }
    
    public List<String> getWords() {
        return this.words;
    }
    
    public String getInfo() {
        return super.getInfo() + ": My major data structure is a " + this.words.getClass().getName();
    }
}

/** Models the stop word filter. */
class StopWordManager extends TFExercise {
    private Set<String> stopWords;

    public StopWordManager() throws IOException {
        this.stopWords = new HashSet<>();
        
        Scanner f = new Scanner(new File("../stop_words.txt"), "UTF-8");
        try {
            f.useDelimiter(",");
            while (f.hasNext()) {
                this.stopWords.add(f.next());
            }
        } finally {
            f.close();
        }
        
        // Add single-letter words
        for (char c = 'a'; c <= 'z'; c++) {
            this.stopWords.add("" + c);
        }
    }

    public boolean isStopWord(String word) {
        return this.stopWords.contains(word);
    }

    public String getInfo() {
        return super.getInfo() + ": My major data structure is a " + this.stopWords.getClass().getName();
    }
}

/** Keeps the word frequency data. */
class WordFrequencyManager extends TFExercise {
    private Map<String, MutableInteger> wordFreqs;

    public WordFrequencyManager() {
        this.wordFreqs = new HashMap<>();
    }

    public void incrementCount(String word) {
        MutableInteger count = this.wordFreqs.get(word);
        if (count == null) {
            this.wordFreqs.put(word, new MutableInteger(1));
        } else {
            count.setValue(count.getValue() + 1);
        }
    }

    public List<WordFrequencyPair> sorted() {
        List<WordFrequencyPair> pairs = new ArrayList<>();
        for (Map.Entry<String, MutableInteger> entry : wordFreqs.entrySet()) {
            pairs.add(new WordFrequencyPair(entry.getKey(), entry.getValue().getValue()));
        }
        pairs.sort(Collections.reverseOrder());
        return pairs;
    }

    public String getInfo() {
        return super.getInfo() + ": My major data structure is a " + this.wordFreqs.getClass().getName();
    }
}

class MutableInteger {
    private int value;

    public MutableInteger(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public void setValue(int value) {
        this.value = value;
    }
}

class WordFrequencyPair implements Comparable<WordFrequencyPair> {
    private String word;
    private int frequency;

    public WordFrequencyPair(String word, int frequency) {
        this.word = word;
        this.frequency = frequency;
    }

    public String getWord() {
        return word;
    }

    public int getFrequency() {
        return frequency;
    }

    public int compareTo(WordFrequencyPair other) {
        return Integer.compare(this.frequency, other.frequency);
    }
}