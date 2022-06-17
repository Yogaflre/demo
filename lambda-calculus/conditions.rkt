#lang racket

(require "operations.rkt")
(provide (all-defined-out))

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

(define IS-ZERO?
  (lambda (n)
    (n (lambda (x) FALSE) TRUE)))
; (IS-ZERO? zero)
; ((lambda (f x) x) (lambda (x) FALSE) TRUE)
; TRUE

; (IS-ZERO? one)
; ((lambda (f x) (f x)) (lambda (x) FALSE) TRUE)
; ((lambda (x) FALSE) TRUE)
; FALSE


(define LEQ?
  (lambda (x y)
    (IS-ZERO? (SUB x y))))

(define EQ?
  (lambda (x y)
    (AND (LEQ? x y)
         (LEQ? y x))))

(define IF
  (lambda (cond-expr true-expr false-expr)
    (cond-expr true-expr false-expr)))
