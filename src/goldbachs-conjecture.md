# 用初等数论探索哥德巴赫猜想
> 在探索哥德巴赫猜想在初等数论框架内证明方式, 并由此发现一些显而易见的有趣结论: 若一个偶数2n能够拆分为两个奇素数的和的形式, 并且如果两个奇素数不相等, 那么这两个素数中较小的一个p(易知\\( p \lt n \\))必然不能整除n.

## 哥德巴赫猜想
关于历史研究历程一类资料, 请参考[wiki].

本文将哥德巴赫猜想简单地描述为:
> 给定任意整数\\(n(n > 1)\\), 以及不超过n的所有素数的集合\\( P = \\{ p \mid Prime(p) \land p \lt n \\} \\). 设\\(p\\)为集合\\( P \\)中的一个元素, 猜想\\(p\\)对应的整数\\(2n-p\\)所组成的集合\\(2n-P\\)中, 必然存在素数元素.

[wiki]: https://en.wikipedia.org/wiki/Goldbach's_conjecture

## 引理
### (一) P中存在不整除n的元素
引理1:
\\[ \label{1.1} \tag{1.1}
    \exists p \in P, p \nmid n
\\]

证明(反证法):

假设命题\\( \ref{1.1} \\) 的反命题:
\\[\label{1.2} \tag{1.2}
    \forall p \in P, p \mid n, 即\forall p \in P, \exists m \in Z, mp = n
\\]
成立, 由伯特兰-切比雪夫定理([Bertrand's postulate])可知:
\\[
    \exists p_0, {n \over 2} \lt p_0 \lt n
\\]

由假设\\( \ref{1.2} \\) 可得:
\\[{mp_0 \over 2} \lt p_0 \Rightarrow m \lt 2\\].

又\\(m \in Z\\), \\(p \gt 0, n \gt 0 \Rightarrow m = {n \over 2} \gt 0\\), 所以\\(m = 1\\), 即\\(mp_0 = p_0 = n\\), 这与\\(p_0 \gt n\\) 矛盾!

所以命题\\( \ref{1.2} \\)不成立. 故命题\\( \ref{1.1} \\)成立.

[Bertrand's postulate]: https://en.wikipedia.org/wiki/Bertrand%27s_postulate

### (二) 给定正整数 \\(x\\) 若整除互质数对\\( a, b \\)之一，则 \\(x\\) 不整除 \\(a-b\\)
\\[ \label{2} \tag{2}
    (a, b) = 1, x \gt 1, a \gt 1, b \gt 1, x|a \lor x|b \Rightarrow x \nmid a-b
\\]

证明:

易知\\(x|a\\)与\\(x|b\\)不同时成立, 否则\\((a, b) \ge x\\), 与 \\((a, b) = 1\\) 矛盾.
分类讨论:
- \\(x|a, \, x \nmid b\\)

    \\( \because x|a \\)

    \\( \therefore \exists m \ge 1, m \in Z,  mx=a. \\)

    假设 \\( x|a-b \\),  即 \\( \exists k \in Z,  kx=a-b \Leftrightarrow b = a-kx = (m-k)x. \\)

    若 \\( m = k \\), 则 \\( b = 0 \\), 与 \\( b > 1 \\) 矛盾;

    若 \\( m \neq k \\), 则 \\( \exists (m-k) \in Z,  b = (m-k)x \Leftrightarrow x|b \\),  与假设 \\( x \nmid b \\) 矛盾!

    故假设 \\( x|a-b \\) 不成立, \\( x \nmid a-b. \\)

- \\(x \nmid a, \, x|b\\). 同理可得:
    若\\( x|a-b\\) ,  则 \\( a = b + kx = (m+k)x \Leftrightarrow x|a \\), 与 \\(x \nmid a\\) 矛盾! 故 \\(x \nmid a-b \\).

## 探索哥德巴赫猜想
分类讨论:
- 若n为素数, 显然 \\( 2n-n = n \\) 亦为素数, 哥德巴赫猜想成立.
- 若n为合数, 则 \\( n \ge 4 \\). 由[引理(一)](#一-p中存在不整除n的元素), 将集合P以能否整除n划分为以下子集: \\( S = \lbrace s \mid s|n \rbrace,  T = \lbrace t \mid t \nmid n \rbrace\\)

    容易发现以下结论: \\( \forall s \in S,  \because s|n, s|s,  \therefore s|2n-s \\), 即若一个素数是n的素因子, 那么对应的整数 \\( 2n-s \\) 为合数. 故**符合哥德巴赫猜想的数对 \\( p \\)与 \\( 2n-p \\)必然满足 \\( p \nmid n \\)(封面结论)**

    到这里, 我们可以得到一个哥德巴赫猜想的等价命题:
    > 对给定合数\\(n\\), 及小于\\(n\\)且不整除\\(n\\)的素数集合\\( T = \lbrace t \mid Prime(t) \land t \gt n \land t \nmid n \rbrace \\), 在集合\\(T\\)对应的整数集\\(2n-T = \lbrace 2n - t \mid Prime(t) \land t \gt n \land t \nmid n \rbrace \\)中是否存在素数

## 进一步探究
### 推论一
\\[ \label{3} \tag{3}
    \forall s \in S, t \in T,  s \nmid 2n-t
\\]
而由[引理(二)]可得:

对于任意前述s, t, 有:
1. \\( Prime(s), Prime(t) \Rightarrow s \nmid t \\)
2. \\( s|n \Rightarrow s|2n \\)

故: \\( \forall s \in S, t \in T,  s \nmid 2n-t \\)
也就是, 至少集合 \\( 2n-T \\) 的元素不会被 \\( S \\) 中的元素整除， 命题 \\( \ref{3} \\) 证毕。

### 推论二
对于\\(T\\)的子集 \\( T_{\gt {n \over 2}} = T_1 = \lbrace t| t \in T, t \gt {n \over 2} \rbrace \\), 具有以下性质:

\\[ \label{4} \tag{4}
    \forall t_1, t_2 \in T_1,  t_1 \nmid 2n-t_2
\\]

证明如下:

假设 \\( \exists t_1, t_2,  t_1 \nmid 2n-t_2 \\), 即 \\( \exists m \in Z^+, mt_1 = 2n - t_2 \\).

\\( \because t_1, t_2为奇数, 2n为偶数 \\)
\\( \therefore m为奇数. \\)

分类讨论:
1. 当 \\( m = 1 \\) 时, t_1 + t_2 = 2n.

    \\( \because t_1 != t_2, t_1 \le n, t_2 \le n, \therefore t_1 + t_2 \lt 2n \\), 矛盾!

2. 当 \\( m \ge 3 \\)时:

    \\( \because t_1 \gt {n \over 2},  t_2 \gt {n \over 2}, \therefore mt_1 + t_2 \gt 2n \\)
, 矛盾!

故假设不成立, 命题 \\( \ref{4} \\) 证毕.

[引理(二)]: #二-给定正整数a-b--1-x-gt-1-a-gt-1-b-gt-1-若xa或xb-rightarrow-x-nmid-a-b
