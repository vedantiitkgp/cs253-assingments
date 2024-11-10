import java.net.MalformedURLException;
import java.net.URL;
import java.net.URLClassLoader;
import java.io.FileInputStream;
import java.io.IOException;
import java.util.Properties;
import java.util.Scanner;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.io.*;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

class Main {
    public static void main(String[] args) {
        System.out.println("Enter class name to use:");
        Scanner in = new Scanner(System.in);
        String name = in.nextLine();
        in.close();
        System.out.println("Loading and instantiating " + name + "...");

        // Load the configuration
        Properties properties = new Properties();
        try (FileInputStream fis = new FileInputStream("../config.properties")) {
            properties.load(fis);
        } catch (IOException e) {
            e.printStackTrace();
            return;
        }

        // Get the URL for the class from the config file
        String classUrlStr = properties.getProperty(name);
        System.out.println(classUrlStr);
        if (classUrlStr == null) {
            System.out.println("Class URL not found in configuration for: " + name);
            return;
        }

        // Create the URLClassLoader
        URL classUrl;
        try {
            classUrl = new URL(classUrlStr);
        } catch (MalformedURLException e) {
            e.printStackTrace();
            return;
        }
        System.out.println(classUrlStr);
        URL[] classUrls = {classUrl};
        URLClassLoader cloader = new URLClassLoader(classUrls);

        // Load and instantiate the class
        try {
            System.out.println(name);
            if(name.equals("golf")){
               name = "1";
            }
            String wordExtractorClass = "WordExtractorImpl" + name;
            String wordFreqCalculatorClass = "WordFrequencyCalculatorImpl" + name;
            Class<?> wordExtractorcls = cloader.loadClass(wordExtractorClass);
            Class<?> wordFreqCalculatorcls = cloader.loadClass(wordFreqCalculatorClass);
            WordExtractor wordExtractor = (WordExtractor) wordExtractorcls.getDeclaredConstructor().newInstance();
            WordFrequencyCalculator wordFreqCalculator = (WordFrequencyCalculator) wordFreqCalculatorcls.getDeclaredConstructor().newInstance();
            List<String> words = wordExtractor.extractWords("../../../pride-and-prejudice.txt", "../../../stop_words.txt");
            List<Map.Entry<String, Integer>> wordFreqs = wordFreqCalculator.getTop25(words);
            // Display top 25 words
            for (Map.Entry<String, Integer> entry : wordFreqs) {
                System.out.println(entry.getKey() + " - " + entry.getValue());
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}