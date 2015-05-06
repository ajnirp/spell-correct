package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
)

type pair struct {
	f string
	s string
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

// Thanks, Rosetta Code!
// http://rosettacode.org/wiki/Remove_duplicate_elements#Go
func remove_duplicates(list []string) []string {
	unique_set := make(map[string]bool, len(list))
	for _, word := range list {
		unique_set[word] = true
	}
	result := make([]string, 0, len(unique_set))
	for word := range unique_set {
		result = append(result, word)
	}
	return result
}

func train(words []string) map[string]int {
	model := map[string]int{}
	for _, word := range words {
		model[word] += 1
	}
	return model
}

func edits1(word string) []string {
	alphabet := "abcdefghijklmnopqrstuvwxyz"
	var splits []pair
	for i := 0; i < len(word) + 1; i++ {
		new_pair := pair{f: word[:i], s: word[i:]}
		splits = append(splits, new_pair)
	}
	var deletes, transposes, replaces, inserts []string
	for _, p := range splits {
		a := p.f
		b := p.s
		if len(b) > 0 {
			deletes = append(deletes, a + b[1:])
		for _, c := range alphabet {
				replaces = append(replaces, a + string(c) + b[1:])
			}
		}

		if len(b) > 1 {
			transposes = append(transposes, a + string(b[1]) + string(b[0]) + b[2:])
		}
		for _, c := range alphabet {
			inserts = append(inserts, a + string(c) + b)
		}
	}
	var result []string
	result = append(result, deletes...)
	result = append(result, transposes...)
	result = append(result, replaces...)
	result = append(result, inserts...)
	return remove_duplicates(result)
}

func known_edits2(word string, nwords map[string]int) []string {
	var result []string
	for _, e1 := range edits1(word) {
		for _, e2 := range edits1(e1) {
			if _, ok := nwords[e2]; ok {
				result = append(result, e2)
			}
		}
	}
	return remove_duplicates(result)
}

func known(words []string, nwords map[string]int) []string {
	var result []string
	for _, word := range words {
		if _, ok := nwords[word]; ok {
			result = append(result, word)
		}
	}
	return remove_duplicates(result)
}

func correct(word string, nwords map[string]int) string {
	candidates := known([]string{word}, nwords)
	if len(candidates) == 0 {
		candidates = known(edits1(word), nwords)
	}
	if len(candidates) == 0 {
		candidates = known_edits2(word, nwords)
	}
	if len(candidates) == 0 {
		candidates = []string{word}
	}
	return max_count(candidates, nwords)
}

func max_count(candidates []string, nwords map[string]int) string {
	best_cand := candidates[0]
	max_value := -1
	for _, cand := range candidates {
		count, ok := nwords[cand]
		if !ok {
			count = 1
		}
		if count > max_value {
			best_cand = cand
			max_value = count
		}
	}
	return best_cand
}

func main() {
	bytes, err := ioutil.ReadFile("../big.txt")
	text := string(bytes)
	check(err)
	word_r, _ := regexp.Compile("[a-z]+")
	words := word_r.FindAllString(text, -1)
	_ = train(words)
	nwords := train(words)
	for _, arg := range os.Args[1:] {
		fmt.Print(correct(arg, nwords), "\n")
	}
}