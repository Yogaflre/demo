#lang racket

(define ? '())

; 定义计算长度函数
(define LEN
  (lambda (l)
    (cond ((null? l) 0)
          (else (+ 1 (LEN (cdr l)))))))

; 抽象匿名函数
; 计算长度为0
(lambda (l)
  (cond ((null? l) 0)
        (else (+ 1 (? (cdr l))))))
; 计算长度为1
(lambda (l)
  (cond ((null? l) 0)
        (else (+ 1 ((lambda (l)
                      (cond ((null? l) 0)
                            (else (+ 1 (? (cdr l))))))
                    (cdr l))))))
; 计算长度为2
(lambda (l)
  (cond ((null? l) 0)
        (else (+ 1 ((lambda (l)
                      (cond ((null? l) 0)
                            (else (+ 1 ((lambda (l)
                                          (cond ((null? l) 0)
                                                (else (+ 1 (? (cdr l))))))
                                        (cdr l))))))
                    (cdr l))))))
; ...


; 将计算长度的匿名函数?提取为参数len
; 计算长度为0
((lambda (len)
   (lambda (l)
     (cond ((null? l) 0)
           (else (+ 1 (len (cdr l)))))))
 ?)
; 计算长度为1
((lambda (len)
   (lambda (l)
     (cond ((null? l) 0)
           (else (+ 1 (len (cdr l)))))))
 ((lambda (len)
    (lambda (l)
      (cond ((null? l) 0)
            (else (+ 1 (len (cdr l)))))))
  ?))
; ...


; 将相同的代码抽象
; 计算长度为0
((lambda (mk-len)
   (mk-len ?))
 (lambda (len)
   (lambda (l)
     (cond ((null? l) 0)
           (else (+ 1 (len (cdr l))))))))
; 计算长度为1
((lambda (mk-len)
   (mk-len
    (mk-len ?)))
 (lambda (len)
   (lambda (l)
     (cond ((null? l) 0)
           (else (+ 1 (len (cdr l))))))))
; ...


; ?可以是任何函数，当然也可以是mk-len自身
((lambda (mk-len)
   (mk-len mk-len))
 (lambda (len)
   (lambda (l)
     (cond ((null? l) 0)
           (else (+ 1 (len (cdr l))))))))

; 上述函数只能计算长度为0的list，因为len需要接收一个list为入参，但现在入参为mk-len (一个函数)
; 将len换成一个可以计算list长度的函数：也就是mk-len自身！就得到了一个可以计算任意长度list的匿名函数
((lambda (mk-len)
   (mk-len mk-len))
 (lambda (mk-len)
   (lambda (l)
     (cond ((null? l) 0)
           (else (+ 1 ((mk-len mk-len) (cdr l))))))))


; 还原len函数，保证计算长度函数的完整性
((lambda (mk-len)
   (mk-len mk-len))
 (lambda (mk-len)
   ((lambda (len)
      (lambda (l)
        (cond ((null? l) 0)
              (else (+ 1 (len (cdr l)))))))
    (lambda (l)
      ((mk-len mk-len) l)))))

; 把len函数抽象出来当作入参
((lambda (le)
   ((lambda (mk-len)
      (mk-len mk-len))
    (lambda (mk-len)
      (le (lambda (l)
            ((mk-len mk-len) l))))))
 (lambda (len)
   (lambda (l)
     (cond ((null? l) 0)
           (else (+ 1 (len (cdr l))))))))


; 把通用递归函数抽象出来，精简下参数名，再起个名字？Y
(define Y
  (lambda (y)
    ((lambda (f) (f f))
     (lambda (f)
       (y (lambda (x)
            ((f f) x)))))))
