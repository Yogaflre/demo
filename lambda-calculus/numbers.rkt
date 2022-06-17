#lang racket

(require "operations.rkt")
(provide (all-defined-out))

; Test func
(define COUNT
  (lambda (x) (+ 1 x)))



; church number
(define zero
  (lambda (f x) x))

(define one (INCR zero))
; (lambda (f x) (f (zero f x)))
; (lambda (f x) (f ((lambda (f x) x) f x)))
; (lambda (f x) (f x))
(define two (INCR one))
; (lambda (f x) (f (one f x)))
; (lambda (f x) (f ((lambda (f x) (f x)) f x)))
; (lambda (f x) (f (f x)))
(define three (INCR two))
; (lambda (f x) (f (two f x)))
; (lambda (f x) (f ((lambda (f x) (f (f x))) f x)))
; (lambda (f x) (f (f (f x))))
; (three COUNT 0)
; ...

