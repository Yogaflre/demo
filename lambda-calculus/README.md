# Lambda 演算

## 基本表达式 (function)
- 函数定义：(lambda (x) (body))
    - lambda：函数标识
    - x.：入参（只接受一个入参）
    - body：函数体（返回计算结果）
- 函数标识符：用于标识函数中的某个参数名。只有在所有标识符都是绑定的情况下才合法
    - 自由标识符(❎)：(lambda (x) (plus x y))   // (x 是绑定标识符，plus / y 是自由标识符)
    - 绑定标识符(✅)：(lambda (x y) (x y))      // (x / y 都是绑定标识符)
- 函数调用：将函数定义放在参数前 ((lambda (x) (body)) y)

## 运算法则
### Alpha转换
变量名称不重要，我们可以同时替换所有相同的变量名称
(lambda (a) (a + 1)) == (lambda (b) (b + 1))
### Beta规约
在函数调用的过程中，可以将标识符用参数值替换
((lambda (z) (lambda (x) (x + z))) (x + 2))
(lambda (x) (x + x + 2))
3 + 3 + 2
