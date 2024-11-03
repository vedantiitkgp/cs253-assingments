datatype Pair = Pair(term: string, count: int)

method Main() {
    AnalyzeTextFrequency("White tigers live mostly in India\nWild lions live mostly in Africa\nSome acquaintance or other, my dear, I suppose; I am sure I do not\nknow.");
}

method AnalyzeTextFrequency(text: string) 
requires text != ""
{
    var normalizedText := NormalizeText(text);
    var stopWords := GetStopWords();
    var tokens := TokenizeText(normalizedText);
    var filteredTokens := FilterStopWords(tokens, stopWords);
    var termFrequency := CalculateFrequency(filteredTokens);
    DisplayTop25(termFrequency);
}

method NormalizeText(inputText: string) returns (normalizedText: string)
requires inputText != ""
ensures forall i :: 0 <= i < |normalizedText| ==> (normalizedText[i]==' ' || 'a' <= normalizedText[i] <= 'z')
{
    normalizedText := "";
    for i := 0 to |inputText| - 1 // Corrected range
        invariant forall k :: 0 <= k < |normalizedText| ==> (normalizedText[k]==' ' || 'a' <= normalizedText[k] <= 'z')
    {
        var c := inputText[i];
        if 'A' <= c <= 'Z' {
            normalizedText := normalizedText + [((c - 'A') + 'a') as char];
        } else if 'a' <= c <= 'z' || c == ' ' {
            normalizedText := normalizedText + [c];
        }
    }
}

method TokenizeText(text: string) returns (tokens: seq<string>)
requires forall i :: 0 <= i < |text| ==> (text[i]==' ' || 'a' <= text[i] <= 'z')
ensures |tokens| >= 0
{
    tokens := [];
    var currentToken: string := "";
    for i := 0 to |text| - 1 // Corrected range
        invariant forall t :: t in tokens ==> |t| > 0
    {
        if text[i] == ' ' {
            if |currentToken| > 0 {
                tokens := tokens + [currentToken];
                currentToken := "";
            }
        } else {
            currentToken := currentToken + [text[i]];
        }
    }
    if |currentToken| > 0 {
        tokens := tokens + [currentToken];
    }
}


method GetStopWords() returns (stopWords: set<string>)
    // ensures forall w :: w in stopWords ==> |w| > 0
{
    stopWords := {"a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z",
                  "able","about","across","after","all","almost","also","am","among","an","and","any","are","as","at",
                  "be","because","been","but","by","can","cannot","could","dear","did","do","does","either","else","ever",
                  "every","for","from","get","got","had","has","have","he","her","hers","him","his","how","however","i",
                  "if","in","into","is","it","its","just","least","let","like","likely","may","me","might","most","must",
                  "my","neither","no","nor","not","of","off","often","on","only","or","other","our","own","rather","said",
                  "say","says","she","should","since","so","some","than","that","the","their","them","then","there","these",
                  "they","this","tis","to","too","twas","us","wants","was","we","were","what","when","where","which","while",
                  "who","whom","why","will","with","would","yet","you","your"};

    // Assert that all words have length greater than 
}

method FilterStopWords(tokens: seq<string>, stopWords: set<string>) returns (filteredTokens: seq<string>)
// requires forall t :: t in tokens ==> |t| > 0
// requires forall w :: w in stopWords ==> |w| > 0
// ensures forall t :: t in filteredTokens ==> t !in stopWords && t in tokens
{
    filteredTokens := [];
    for i := 0 to |tokens|
        invariant forall t :: t in filteredTokens ==> t !in stopWords && t in tokens
    {
        if tokens[i] !in stopWords {
            filteredTokens := filteredTokens + [tokens[i]];
        }
    }
}

method CalculateFrequency(words: seq<string>) returns (freqMap: seq<Pair>)
requires |words| >= 0
{
    freqMap := [];
    
    var j := 0;
    while j < |words| decreases |words|-j
        invariant j>=0
    {
        var i := 0;
        var found := 0;
        var word := words[j];
        while i < |freqMap| decreases |freqMap|-i
        {
            if freqMap[i].term == word {
                found := 1;
                freqMap := freqMap[..i] + [Pair(word,freqMap[i].count + 1)] + freqMap[i+1..];
                break;
            }    
            i := i+1;
        }
        if found == 0 {
            freqMap := freqMap + [Pair(word,1)];
        }
        j := j+1;
    }
    var i:=0;
    while i < |freqMap| decreases |freqMap| -i
        invariant 0 <= i <= |freqMap|
    {
        var j:= i+1;
        //var maxV:=freqMap[i].value;
        var maxIndex:=i;
        while j < |freqMap| decreases |freqMap| -j
            invariant i < j <= |freqMap| && i <= maxIndex < |freqMap| 
            invariant maxIndex>i ==> freqMap[maxIndex].count >= freqMap[i].count
        {
            if freqMap[maxIndex].count < freqMap[j].count
            {
                //maxV := freqMap[j].value;
                maxIndex := j;
            }
            j:=j+1;
        }
        if maxIndex != i
        {
            freqMap := freqMap[..i] + [freqMap[maxIndex]] + freqMap[i+1..maxIndex] + [freqMap[i]] + freqMap[maxIndex+1..];
        }
        i := i+1;
    }
}

function method IntToString(n: int): string
requires n >= 0
decreases n
{
    if n == 0 then "0"
    else 
        var digit := n % 10;
        var rest := n / 10;
        if rest == 0 then [DigitToChar(digit)]
        else IntToString(rest) + [DigitToChar(digit)]
}

function method DigitToChar(d: int): char
requires 0 <= d <= 9
{
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][d]
}

method DisplayTop25(wordFreqs: seq<Pair>) 
requires |wordFreqs| >= 0
// requires forall i :: 0 <= i < |wordFreqs| ==> wordFreqs[i].count > 0
{
    var i := 0;
    while i < |wordFreqs| && i < 25 
        decreases |wordFreqs| - i
        invariant 0 <= i <= |wordFreqs|
    {
        print wordFreqs[i].term + " - " + IntToString(wordFreqs[i].count) + "\n";
        i := i + 1;
    }
}