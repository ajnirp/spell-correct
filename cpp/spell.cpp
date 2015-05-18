#include <iostream>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <utility>
#include <map>
#include <string>
#include <vector>
#include <regex>

using namespace std;

void remove_duplicates(vector<string> &words) {
    sort(words.begin(), words.end());
    auto last = unique(words.begin(), words.end());
    words.erase(last, words.end());
}

vector<string> edits1(const string &word) {
    vector<string> result;
    const string alphabet = "abcdefghijklmnopqrstuvwxyz";
    for (int i = 0 ; i <= word.length() ; i++) {
        const string first = word.substr(0, i);
        const string second = word.substr(i);
        if (second.length() > 0) {
            const string third = second.substr(1);
            // delete
            result.push_back(first + third);
            // replace
            for (char c : alphabet)
                result.push_back(first + c + third);
        }
        // insert
        for (char c : alphabet)
            result.push_back(first + c + second);
        // transpose
        if (second.length() > 1)
            result.push_back(first + second[1] + second[0] + second.substr(2));
    }
    remove_duplicates(result);
    return result;
}

vector<string> known_edits2(const string &word, map<string, int> &model) {
    vector<string> result;
    for (string s1 : edits1(word))
        for (string s2 : edits1(s1))
            if (model.find(s2) != model.end())
                result.push_back(s2);
    remove_duplicates(result);
    return result;
}

void known(vector<string> &words, map<string, int> &model, vector<string> &result) {
    result.clear();
    for (string word : words)
        if (model.find(word) != model.end())
            result.push_back(word);
}

bool get_count(const string &a, map<string, int> &model) {
    if (model.find(a) == model.end())
        return 1;
    return model[a];
}

string find_best_candidate(vector<string> &candidates, map<string, int> &model) {
    int max_value = -1;
    string best_candidate;
    for (string cand : candidates) {
        int count = 1;
        if (model.find(cand) != model.end())
            count = model[cand];
        if (count > max_value) {
            max_value = count;
            best_candidate = cand;
        }
    }
    return best_candidate;
}

const string correct(const string &word, map<string, int> &model) {
    if (model.find(word) != model.end())
        return word;
    vector<string> candidates;
    vector<string> e1 = edits1(word);
    known(e1, model, candidates);
    if (candidates.size() > 0)
        return find_best_candidate(candidates, model);
    candidates.clear();
    candidates = known_edits2(word, model);
    if (candidates.size() > 0)
        return find_best_candidate(candidates, model);
    return word;
}

// Speed bottleneck
map<string, int> get_words_and_train(const string &contents) {
    regex word_regex("[a-z]+");
    auto match_begin = sregex_iterator(contents.begin(), contents.end(), word_regex);
    auto match_end = sregex_iterator();
    map<string, int> result;
    for (auto i = match_begin; i != match_end; ++i) {
        string word = i->str();
        if (result.find(word) == result.end())
            result[word] = 1;
        else
            result[word]++;
    }
    return result;
}

int main(int argc, char const *argv[]) {
    ifstream file("../big.txt");
    string contents((istreambuf_iterator<char>(file)), (istreambuf_iterator<char>()));
    transform(contents.begin(), contents.end(), contents.begin(), ::tolower);
    map<string, int> nwords = get_words_and_train(contents);
    for (int i = 1 ; i < argc ; i++)
        cout << correct(argv[i], nwords) << endl;
    return 0;
}