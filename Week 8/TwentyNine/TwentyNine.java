import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.regex.*;
import java.util.stream.*;

abstract class ActiveThing implements Runnable {
    private Thread thread;
    protected BlockingQueue<List<Object>> queue = new LinkedBlockingQueue<>();
    protected boolean alive = true;

    public ActiveThing() {
        thread = new Thread(this);
        thread.start();
    }

    public void send(List<Object> message) {
        queue.add(message);
    }

    public void stop() {
        alive = false;
        queue.add(List.of("stop")); // Add a "stop" message to ensure graceful termination
        thread.interrupt();
    }

    public void join() throws InterruptedException {
        thread.join();
    }

    @Override
    public void run() {
        while (alive) {
            try {
                List<Object> message = queue.take();
                dispatch(message);
            } catch (InterruptedException e) {
                // Graceful exit
                alive = false;
            }
        }
    }

    protected abstract void dispatch(List<Object> message);
}

class DataStorageManager extends ActiveThing {
    private String data = "";
    private StopWordManager stopWordManager;

    @Override
    protected void dispatch(List<Object> message) {
        String command = (String) message.get(0);
        switch (command) {
            case "init" -> init((String) message.get(1), (StopWordManager) message.get(2));
            case "process" -> process((WordFrequencyController) message.get(1));
            case "stop" -> alive = false;
            default -> throw new IllegalArgumentException("Unknown message: " + command);
        }
    }

    private void init(String filePath, StopWordManager stopWordManager) {
        this.stopWordManager = stopWordManager;
        try {
            data = Files.readString(Path.of(filePath))
                        .replaceAll("[\\W_]+", " ")
                        .toLowerCase();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private void process(WordFrequencyController controller) {
        for (String word : data.split("\\s+")) {
            stopWordManager.send(List.of("filter", word));
        }
        stopWordManager.send(List.of("top25", controller));
    }
}

class StopWordManager extends ActiveThing {
    private Set<String> stopWords = new HashSet<>();
    private WordFrequencyManager wordFreqManager;

    @Override
    protected void dispatch(List<Object> message) {
        String command = (String) message.get(0);
        switch (command) {
            case "init" -> init((WordFrequencyManager) message.get(1));
            case "filter" -> filter((String) message.get(1));
            case "top25" -> wordFreqManager.send(message);
            case "stop" -> alive = false;
            default -> throw new IllegalArgumentException("Unknown message: " + command);
        }
    }

    private void init(WordFrequencyManager wordFreqManager) {
        this.wordFreqManager = wordFreqManager;
        try {
            String content = Files.readString(Path.of("../../stop_words.txt"));
            stopWords.addAll(Arrays.asList(content.split(",")));
            stopWords.addAll(IntStream.rangeClosed('a', 'z').mapToObj(c -> String.valueOf((char) c)).toList());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private void filter(String word) {
        if (!stopWords.contains(word)) {
            wordFreqManager.send(List.of("word", word));
        }
    }
}

class WordFrequencyManager extends ActiveThing {
    private final Map<String, Integer> wordFreqs = new HashMap<>();

    @Override
    protected void dispatch(List<Object> message) {
        String command = (String) message.get(0);
        switch (command) {
            case "word" -> incrementCount((String) message.get(1));
            case "top25" -> top25((WordFrequencyController) message.get(1));
            case "stop" -> alive = false;
            default -> throw new IllegalArgumentException("Unknown message: " + command);
        }
    }

    private void incrementCount(String word) {
        wordFreqs.merge(word, 1, Integer::sum);
    }

    private void top25(WordFrequencyController controller) {
        List<Map.Entry<String, Integer>> sorted =
            wordFreqs.entrySet()
                     .stream()
                     .sorted((a, b) -> b.getValue() - a.getValue())
                     .limit(25)
                     .toList();
        controller.send(List.of("top25", sorted));
    }
}

class WordFrequencyController extends ActiveThing {
    private DataStorageManager storageManager;

    @Override
    protected void dispatch(List<Object> message) {
        String command = (String) message.get(0);
        switch (command) {
            case "run" -> runController((DataStorageManager) message.get(1));
            case "top25" -> display((List<Map.Entry<String, Integer>>) message.get(1));
            case "stop" -> alive = false;
            default -> throw new IllegalArgumentException("Unknown message: " + command);
        }
    }

    private void runController(DataStorageManager storageManager) {
        this.storageManager = storageManager;
        storageManager.send(List.of("process", this));
    }

    private void display(List<Map.Entry<String, Integer>> wordFreqs) {
        wordFreqs.forEach(entry -> System.out.println(entry.getKey() + " - " + entry.getValue()));
        stop();
        storageManager.stop();
    }
}

public class TwentyNine {
    public static void main(String[] args) throws InterruptedException {
        if (args.length < 1) {
            System.out.println("Usage: java TwentyNine <input_file>");
            return;
        }

        WordFrequencyManager wordFreqManager = new WordFrequencyManager();
        StopWordManager stopWordManager = new StopWordManager();
        DataStorageManager storageManager = new DataStorageManager();
        WordFrequencyController controller = new WordFrequencyController();

        stopWordManager.send(List.of("init", wordFreqManager));
        storageManager.send(List.of("init", args[0], stopWordManager));
        controller.send(List.of("run", storageManager));

         // Wait for controller to finish
         controller.join();

         // Stop all threads
         wordFreqManager.stop();
         stopWordManager.stop();
         storageManager.stop();

        // Wait for all threads to finish
        for (ActiveThing thing : List.of(wordFreqManager, stopWordManager, storageManager, controller)) {
            thing.join();
        }
    }
}