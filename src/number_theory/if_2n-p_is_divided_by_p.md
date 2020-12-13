# 若质数p不能整除偶数2n，则p不整除2n-p
## 命题形式化描述
\\[\label{1} \tag{1}
    \forall p \in Primes, p \lt 2n(n \in Z^+), p \nmid 2n \Rightarrow p \nmid 2n - p
\\]

## 一般化命题与反证法 [^1]
我们可以抽出一个更一般的命题：对任意正整数 \\(a, b (a \lt b)\\)，若 \\( a \nmid b\\)，则 \\( a \nmid b - a \\):
\\[\label{1.0} \tag{1.0}
    \forall a, b \in Z^+ (a \lt b), a \nmid b \Rightarrow a \mid b - a
\\]

### 反证法
假设原结论不成立，即 \\( a \mid b - a \\) 成立，则：

\\( \because a \mid b - a, a \mid a \\)

\\(  \therefore a \mid (b - a) + a \\) （整除的线性性质），即 \\( a \mid b \\),

这与假设 \\( a \nmid b \\) 矛盾！故假设不成立，原结论成立，即 \\( a\mid b - a \\),

故命题 \\( \ref{1.0} \\) 成立。

### 代入条件

令 \\( a := p, b = 2n \\), 则显然：

\\[
    p \in Primes, p \lt 2n \Rightarrow p \nmid 2n \Rightarrow p \nmid 2n - p
\\]

即命题 \\( \ref{1} \\) 得证。

---

(以下内容为作者之前思考过的另外一个比较迂回的证明方法，为后续叙述的引理）
## 一整数若不同时整除互质两数，则不能整除后两数之差
### 形式化描述
\\[ \label{1.1} \tag{1.1}
    \forall x \in Z^+ \land x > 1, a \in Z^+ \land a > 1, b \in Z^+ \land b > 1; \quad
    (a, b) = 1,  x|a \lor x|b \Rightarrow x \nmid a-b
\\]

### 证明
不妨设 \\( a \gt b \\)。易知\\(x|a\\)与\\(x|b\\)不同时成立, 否则\\((a, b) \ge x\\), 与 \\((a, b) = 1\\) 矛盾.
分类讨论:
- \\(x|a, \, x \nmid b\\)

    \\( \because x|a \\)

    \\( \therefore \exists  m \in Z^+,  mx=a. \\)

    假设 \\( x|a-b \\),  即 \\( \exists k \in Z^+,  kx=a-b \Leftrightarrow b = a-kx = (m-k)x. \\)

    若 \\( m = k \\), 则 \\( b = 0 \\), 与 \\( b > 1 \\) 矛盾;

    若 \\( m \neq k \\), 则 \\( \exists (m-k) \in Z,  b = (m-k)x \Leftrightarrow x|b \\),  与假设 \\( x \nmid b \\) 矛盾!

    故假设 \\( x|a-b \\) 不成立, \\( x \nmid a-b. \\)

- \\(x \nmid a, \, x|b\\). 同理可得:
    若\\( x|a-b\\) ,  则 \\( a = b + kx = (m+k)x \Leftrightarrow x|a \\), 与 \\(x \nmid a\\) 矛盾! 故 \\(x \nmid a-b \\).

### 代入条件
令 \\( x := p, a := 2n, b := p \\)，显然有\\( p \mid p \\)。故，当 \\(p \nmid 2n \\)时， \\( p \nmid 2n - p \\)，即命题 \\( \ref{1} \\) 得证。


[^1]: 感谢中山大学数学专业的倪秉业师弟指出这个更简洁的证明方式！
