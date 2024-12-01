import numpy as np
import argparse
import re

def apply_leet(characters, mapping):
    # Apply the mapping using np.vectorize for efficiency
    convert = np.vectorize(lambda ch: mapping.get(ch, ch))  # Default to character itself if not in mapping
    return convert(characters)

def apply_anti_leet(text, mapping):
    # Sort anti-Leet mappings by descending length of keys
    sorted_mapping = sorted(mapping.items(), key=lambda item: -len(item[0]))
    regex = re.compile("|".join(re.escape(k) for k, _ in sorted_mapping))
    
    # Replace leet sequences using the regex
    return regex.sub(lambda match: mapping[match.group(0)], text)

# Function to process the text
def process_text(file_path):
    # Read the file and convert to a character array
    with open(file_path, 'r') as f:
        text = f.read()
    characters = np.array(list(text))

    # Normalize: Replace non-alphabetic characters with spaces, convert to uppercase
    characters[~np.char.isalpha(characters)] = ' '
    characters = np.char.upper(characters)

    # Define leet mapping
    leet_mapping = {
        'A': '4', 'B': '8', 'C': '<', 'D': '|)', 'E': '3', 'F': '7', 
        'G': '6', 'H': '#', 'I': '1','J': '9', 'K':'1<', 'L': '|', 
        'M': '44', 'N': '/\/','O': '0', 'P': '|o', 'Q': 'O_', 'R': '12',
        'S': '5', 'T': '7', 'U': '|_|', 'V': 'V', 'W': '\/\/', 'X': '%', 
        'Y': '`/', 'Z': '2', '0': 'O', '1': '|', '2': 'Z', '3': 'E', '4': 
        'A', '5': 'S', '6': 'G', '7': 'T', '8': 'B', '9': 'J'
    }
    anti_leet_mapping = {v: k for k, v in leet_mapping.items()}

    # Apply leet mapping
    characters = apply_leet(characters, leet_mapping)

    # Find spaces and split into words
    sp = np.where(characters == ' ')[0]
    sp2 = np.repeat(sp, 2)
    w_ranges = np.reshape(sp2[1:-1], (-1, 2))
    w_ranges = w_ranges[np.where(w_ranges[:, 1] - w_ranges[:, 0] > 2)]  # Ignore spaces

    words = np.array([''.join(characters[r[0]:r[1]]).strip() for r in w_ranges])

    # Filter out words smaller than 2 characters
    words = words[np.char.str_len(words) >= 2]

    # Load stop words
    try:
        stop_words = np.array(list(set(open('../stop_words.txt').read().split(','))))
        words = words[~np.isin(words, stop_words)]
    except FileNotFoundError:
        print("Stop words file not found. Proceeding without filtering stop words.")

    # Generate 2-grams
    if len(words) < 2:
        print("Not enough words for generating 2-grams.")
        return

    # Creating two grams through np array functions
    two_grams = np.char.add(np.char.add(words[:-1], ' '), words[1:])

    # Count occurrences of 2-grams
    uniq, counts = np.unique(two_grams, return_counts=True)
    freq_sorted = sorted(zip(uniq, counts), key=lambda t: t[1], reverse=True)

    # Print top 5 most frequent 2-grams
    for two_gram, count in freq_sorted[:5]:
        current_two_gram_decoded = apply_anti_leet(two_gram, anti_leet_mapping)
        print(f"{two_gram} - {count} (Anti Leet: {current_two_gram_decoded})")

if __name__ == "__main__":
    # Parse command-line arguments
    parser = argparse.ArgumentParser(description="Process a text file to find the top 5 2-grams.")
    parser.add_argument("file_path", type=str, help="Path to the input text file")
    args = parser.parse_args()

    # Call the function with the provided file path
    process_text(args.file_path)