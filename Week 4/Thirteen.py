import sys, re, operator, string

# Auxiliary functions that can't be lambdas
def extract_words(obj, path_to_file):
    print(f"Trying to open text file: {path_to_file}")  # Debugging print
    try:
        with open(path_to_file) as f:
            obj['data'] = f.read()
    except FileNotFoundError:
        print(f"Error: File '{path_to_file}' not found.")
        sys.exit(1)
    
    pattern = re.compile(r'[\W_]+')
    data_str = ''.join(pattern.sub(' ', obj['data']).lower())
    obj['data'] = data_str.split()

def load_stop_words(obj):
    stop_words_path = '../stop_words.txt'
    print(f"Trying to open stop words file: {stop_words_path}")  # Debugging print
    try:
        with open(stop_words_path) as f:
            obj['stop_words'] = f.read().split(',')
    except FileNotFoundError:
        print(f"Error: Stop words file '{stop_words_path}' not found.")
        sys.exit(1)
    
    obj['stop_words'].extend(list(string.ascii_lowercase))

def increment_count(obj, w):
    obj['freqs'][w] = 1 if w not in obj['freqs'] else obj['freqs'][w] + 1

def create_object(initial_data):
    this = initial_data

    def get_attr(name):
        return this[name]

    def set_attr(name, value):
        this[name] = value

    return {'get': get_attr, 'set': set_attr, **this}

data_storage_obj = create_object({
    'data': [],
    'init': lambda path_to_file: extract_words(data_storage_obj, path_to_file),
    'words': lambda: data_storage_obj['get']('data')
})

stop_words_obj = create_object({
    'stop_words': [],
    'init': lambda: load_stop_words(stop_words_obj),
    'is_stop_word': lambda word: word in stop_words_obj['get']('stop_words')
})

word_freqs_obj = create_object({
    'freqs': {},
    'increment_count': lambda w: increment_count(word_freqs_obj, w),
    'sorted': lambda: sorted(word_freqs_obj['get']('freqs').items(), key=operator.itemgetter(1), reverse=True),
    'top25': lambda: [
        print(w, '-', c) for (w, c) in word_freqs_obj['get']('sorted')()[0:25]
    ]
})

# Initialize data and stop words
data_storage_obj['init'](sys.argv[1])
print(f"Loaded {len(data_storage_obj['words']())} words.")
stop_words_obj['init']()
print(f"Loaded {len(stop_words_obj['get']('stop_words'))} stop words.")

# Count word frequencies, ignoring stop words
for w in data_storage_obj['words']():
    if not stop_words_obj['is_stop_word'](w):
        word_freqs_obj['increment_count'](w)
        print(f"Counting: {w}")

# Call the top25 method to print the top 25 word frequencies
word_freqs_obj['top25']()
print("Top 25 printed.")