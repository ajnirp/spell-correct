# straightforward port, probably not idiomatic or as concise as possible

import re, tables, os, sequtils, sets

var alph = "abcdefghijklmnopqrstuvwxyz"

proc splits(word: string): seq[array[2, string]] =
  var length = len(word)
  var res: seq[array[2, string]]
  if length == 0:
    res = @[["", ""]]
    return res
  else:
    res = @[["", word], [word, ""]]
  for i in 1..length-1:
    res.add([word[0..i-1], word[i..length-1]])
  return res

proc edits1(word: string): seq[string] =
  var splits = splits(word)
  var filt0 = filter(splits) do (x: array[2, string]) -> bool : len(x[1]) > 0
  var filt1 = filter(splits) do (x: array[2, string]) -> bool : len(x[1]) > 1
  var deletes = filt0.mapIt(string, it[0] & it[1][1..it[1].high])
  var transposes = filt1.mapIt(string, it[0] & it[1][1..1] & it[1][0..0] & it[1][2..it[1].high])
  var replaces: seq[string] = @[]
  for pair in filt0:
    for c in alph:
      replaces.add(pair[0] & c & pair[1][1..pair[1].high])
  var inserts: seq[string] = @[]
  for pair in splits:
    for c in alph:
      inserts.add(pair[0] & c & pair[1])
  var temp_set = initSet[string]()
  for s in concat(deletes, transposes, replaces, inserts): temp_set.incl(s)
  var res: seq[string] = @[]
  for edit in temp_set: res.add(edit)
  return res

proc known(words: seq[string], nwords: CountTable[string]): seq[string] =
  return filter(words) do (x: string) -> bool : nwords.hasKey(x)

proc known_edits2(word: string, nwords: CountTable[string]): seq[string] =
  var temp_set = initSet[string]()
  for ed1 in edits1(word):
    for ed2 in edits1(ed1):
      if nwords.hasKey(ed2):
        temp_set.incl(ed2)
  var res: seq[string] = @[]
  for edit in temp_set: res.add(edit)
  return res

proc max_frequency_word(candidates: seq[string], nwords: CountTable[string]): string =
  var max_value = -1
  var res: string
  for cand in candidates:
    var count = 1
    if nwords.hasKey(cand):
      count = nwords[cand]
    if count > max_value:
      max_value = count
      res = cand
  return res

proc correct(word: string, nwords: CountTable[string]): string =
  var candidates = known(@[word], nwords)
  if len(candidates) == 0:
    candidates = known(edits1(word), nwords)
  if len(candidates) == 0:
    candidates = known_edits2(word, nwords)
  if len(candidates) == 0:
    candidates = @[word]
  return max_frequency_word(candidates, nwords)

var words = readFile("../big.txt").findAll(re"[a-z]+", 0)
var nwords = initCountTable[string]()
for word in words: nwords.inc(word)

for arg in os.commandLineParams():
  echo correct(arg, nwords)