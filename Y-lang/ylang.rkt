#lang racket

; context
(define env0 '())

(define ext-env
  (lambda (k v env)
    (cons (cons k v) env)))

(define lookup-env
  (lambda (k env)
    (cond 
      ((null? env) #f)
      (else 
        (cond 
          ((eq? k (caar env)) (cdar env))
          (else (lookup-env k (cdr env))))))))

; 闭包和静态方法不同的地方就在于，闭包需要囊括上下文
(struct Closure (f env))



; 语法特性
; 变量
(define handle-var
  (lambda (x env)
    (let ((v (lookup-env x env)))
         (cond
           ((not v) (error "undefined variable" x))
           (else v)))))
; 数字
(define handle-num
  (lambda x
    (cond
      ((pair? x) (car x))
      (else x))))
; 函数
; 绑定
; 调用
; 算数表达式
(define handle-math
  (lambda (op x y)
    (match op
      ('+ (+ x y))
      ('- (- x y))
      ('* (* x y))
      ('/ (/ x y))
      (else (error "unsupported operation" op)))))

(define interp
  (lambda (expr env)
    (match expr
      ((? symbol? x) (handle-var x env))
      ((? number? x) (handle-num x))
      ((,op ,x ,y) (handle-math op x y))
      (else (error "unsupported expression" expr)))))

(define Ylang
  (lambda expr
    (interp expr env0)))

; test
(define env1 (ext-env 'x '1 env0))
(define env2 (ext-env 'y '2 env1))
(interp 'y env2)
(interp '100 env2)
(interp '(+ 2 3) env2)
