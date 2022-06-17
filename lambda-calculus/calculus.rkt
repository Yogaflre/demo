#lang racket

; preparation
; currying
(define plus-normal
  (lambda (x y)
    (+ x y)))
(define plus-currying
  (lambda (x)
    (lambda (y)
      (+ x y))))
; (plus-currying 1) -> (lambda (y) (+ 1 y))
; ((plus-currying 1) 2) -> (+ 1 2)




; number (church number)
(define COUNT
  (lambda (x) (+ 1 x)))

(define zero
  (lambda (f x) x))

(define INCR
  (lambda (n f x)
    (f (n f x))))

(define one
  (lambda (f x)
    (INCR zero f x)))
; (lambda (f x) (f (zero f x)))
; (lambda (f x) (f ((lambda (f x) x) f x)))
; (lambda (f x) (f x))
(define two
  (lambda (f x)
    (INCR one f x)))
; (lambda (f x) (f (one f x)))
; (lambda (f x) (f ((lambda (f x) (f x)) f x)))
; (lambda (f x) (f (f x)))
(define three
  (lambda (f x)
    (INCR two f x)))
; (lambda (f x) (f (two f x)))
; (lambda (f x) (f ((lambda (f x) (f (f x))) f x)))
; (lambda (f x) (f (f (f x))))
; (three COUNT 0)
; ...

λn.λf.λx.n (λg.λh.h (g f)) (λu.x) (λu.u)
; FIXME
(define DECR
  (lambda (n f x)
    ((n
      (lambda (g h)
        (h (g f)))
      (lambda (u) x))
     (lambda (u) u))))

(DECR two COUNT 0)




; operation
(define ADD
  (lambda (m n)
    (lambda (f x)
      (m f (n f x)))))
; (ADD one two)
; (lambda (f x) (one f (two f x)))
; (lambda (f x) ((lambda (f x) (f x)) f ((lambda (f x) (f (f x))) f x)))
; (lambda (f x) ((lambda (f x) (f x)) f (f (f x))))
; (lambda (f x) (f (f (f x))))
; ((ADD one two) COUNT 0)




; condition
; boolean
(define TRUE (lambda (x y) x))
(define FALSE (lambda (x y) y))

(define AND
  (lambda (x y)
    (x y FALSE)))
; (AND TRUE FALSE)
; (TRUE FALSE FALSE)
; ((lambda (x y) x) (lambda (x y) y) (lambda (x y) y))
; (lambda (x y) y)

(define OR
  (lambda (x y)
    (x TRUE y)))

(define NOT
  (lambda (x)
    (x FALSE TRUE)))

(define XOR
  (lambda (x y)
    (x (y TRUE FALSE) y)))
; (XOR TRUE TRUE)
; (TRUE (FALSE TURE FALSE) TRUE)
; FALSE

; (XOR FALSE TRUE)
; (FALSE (TRUE TRUE FALSE) TRUE)
; TRUE


(define IF
  (lambda (cond-expr true-expr false-expr)
    (cond-expr true-expr false-expr)))




; recursive
