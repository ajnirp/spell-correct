#lang racket

;;; Running instructions

;; To run this file, open it in DrRacket and press Ctrl+R
;; At the REPL, type (correct "word-you-want-to-correct") and hit enter

; read all [a-z]+ from the file named 'text'
(define (words text)
  (map (compose string-downcase bytes->string/utf-8)
       (regexp-match* #rx"[a-zA-Z]+" (open-input-file text))))

; initialise an empty hash of known words
(define nwords (make-hash))

; populate the hash with counts of words
; note that this is a simple hash, no defaultdict behaviour
(define (train features nwords)
  (for ((word features))
    (if (hash-has-key? nwords word)
        (hash-set! nwords word (+ 1 (hash-ref nwords word)))
        (hash-set! nwords word 1))))

; read the file and populate the hash
(train (words "../big.txt") nwords)

(define alphabet "abcdefghijklmnopqrstuvwxyz")

(define (splits word)
  (for/list ((i (range (+ 1 (string-length word)))))
    (cons (substring word 0 i) (substring word i))))

; all words at an edit distance of 1 from 'word'
; may or may not be in nwords, though
(define (edits-1 word)
  (let* ((spl (splits word))
         (firsts (map car spl))
         (seconds (map cdr spl)))
    (define deletes ; a + b[1:] if len(b) > 0
      (for/set ((a firsts) (b seconds) #:unless (equal? b ""))
               (string-append a (substring b 1))))
    (define transposes ; a + b[1] + b[0] + b[2:] if len(b) > 1
      (for/set ((a firsts) (b seconds) #:when (> (string-length b) 1))
               (string-append a (string (string-ref b 1))
                              (string (string-ref b 0)) (substring b 2))))
    (define replaces ; a + c + b[1:] if len(b) > 0
      (for*/set ((pair spl) (c alphabet) #:unless (equal? (cdr pair) ""))
                (string-append (car pair) (string c) (substring (cdr pair) 1))))
    (define inserts ; a + c + b
      (for*/set ((pair spl) (c alphabet))
                (string-append (car pair) (string c) (cdr pair))))
    (set-union deletes transposes replaces inserts)))

; return the count if the word is in the hash, else return #f
(define (in-nwords word) (if (hash-has-key? nwords word) (hash-ref nwords word) #f))

; return 1 if the word is not in the hash, else return its count
(define (get-count word) (let ((count-if-present (in-nwords word))) (if count-if-present count-if-present 1)))

; all words at an edit distance of 2 from the original
; guaranteed to be present in nwords
(define (known-edits-2 word)
  (let ((result (for*/set ((e1 (edits-1 word))
                           (e2 (edits-1 e1))
                           #:when (in-nwords e2))
                          e2)))
    (if (zero? (set-count result)) #f result)))

; remove all words from 'words' that do not lie in nwords
; words is a set
(define (known words)
  (let ((result (list->set (filter (λ (w) (in-nwords w)) (set->list words)))))
    (if (zero? (set-count result)) #f result)))

; correct the word
; the maximisation is done via a left fold that accumulates the maximum of the
; cdr's of the list elements seen so far
; each pair is a ("word" . frequency) pair
(define (correct word)
  (let* ((word-l (set word))
         (candidates (or (known word-l)
                         (known (edits-1 word))
                         (known-edits-2 word)
                         word-l)))
    (car
     (foldr (λ (curr acc) (if (> (cdr curr) (cdr acc)) curr acc))
            '(#t . 0) (set-map candidates (λ (cand) (cons cand (get-count cand))))))))

(display "Enter a word: ")
(correct (symbol->string (read)))