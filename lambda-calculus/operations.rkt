#lang racket

(provide (all-defined-out))

(define INCR
  (lambda (n)
    (lambda (f x)
      (f (n f x)))))

(define DECR
  (lambda (n)
    (lambda (f x)
      ((n (lambda (g)
            (lambda (h)
              (h (g f))))
          (lambda (u) x))
       (lambda (u) u)))))

(define ADD
  (lambda (m n)
    (m INCR n)))
; (ADD one two)
; (lambda (f x) (one f (two f x)))
; (lambda (f x) ((lambda (f x) (f x)) f ((lambda (f x) (f (f x))) f x)))
; (lambda (f x) ((lambda (f x) (f x)) f (f (f x))))
; (lambda (f x) (f (f (f x))))

(define SUB
  (lambda (m n)
    (n DECR m)))
