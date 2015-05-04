require 'set'

def train(features)
  model = Hash.new { |hash, key| hash[key] = 1 }
  features.each { |f| model[f] += 1 }
  return model
end

$NWORDS = train(File.read('../big.txt').downcase.scan(/[a-z]+/))

$alphabet = 'abcdefghijklmnopqrstuvwxyz'

# all words at a distance of 1 edit from 'word'
def edits1(word)
  splits = (0...word.length).map { |i| [word[0...i], word[i..-1]] }
  deletes, replaces, transposes, inserts = [], [], [], []
  splits.each do |a, b|
    deletes << a + b[1..-1]
    transposes << a + b[1] + b[0] + b[2..-1] rescue nil
    $alphabet.each_char do |alph|
      replaces << a + alph + b[1..-1]
      inserts << a + alph + b
    end
  end
  $alphabet.each_char { |alph| inserts << word + alph }
  return Set.new(deletes + transposes + replaces + inserts)
end

# check if 'word' is in $NWORDS
# returns false is not, else it returns a positive number
def in_nwords(word)
  $NWORDS.fetch word rescue false
end

# all words at a distance of 2 edit from 'word'
def known_edits2(word)
  result = Set.new
  edits1(word).each { |e1| edits1(e1).each { |e2| result << e2 if in_nwords(e2) } }
  return result
end

# reject all w in words that do NOT lie in $NWORDS
def known(words)
  Set.new(words.reject { |w| !in_nwords(w) })
end

def correct(word)
  no_edits = known([word])
  one_edits = known(edits1(word))
  two_edits = known_edits2(word)
  candidates = no_edits.empty? ? one_edits.empty? ? two_edits.empty? ?
               [word] : two_edits : one_edits : no_edits
  return candidates.max { |a,b| $NWORDS.fetch(a) <=> $NWORDS.fetch(b) }
end

ARGV.each { |arg| puts correct(arg) }