# 整数和它两倍之间的合数，可以整除整数的阶乘
## 形式化描述
\\[ \label{2} \tag{2}
    \forall m \in Z^+, m \gt 8, n \in Z^+, m \lt n \lt 2m; \quad n \not \in Primes \Rightarrow n \mid m!
\\]

## 证明
因为 \\( n \\) 为合数，将其分为完全平方数和不完全平方数两种情况证明。

### 不完全平方数
不妨设 \\( n = ab (a \gt b \ge 2, (a, b) = 1) \\)，则 \\(b \lt a \lt m \\)，证明如下：

若 \\( a \ge m \\)，

则 \\(n = ab \ge 2a \ge 2m \Leftrightarrow n \ge 2m \\)，

这与 \\( n \lt 2m \\) 矛盾！

故假设不成立，故 \\(2 \le b \lt a \lt m \\)。

又 \\( (a, b) = 1 \Rightarrow [a, b] = ab = n \\)

因此：\\( \exists C \in Z^+, m! = C \cdot ab = C \cdot n \Rightarrow n \mid m! \\)

### 完全平方数
不妨设 \\( n = k^2 \\)，则 \\(2 \lt k \lt 2k \lt m \\)，证明如下：

若 \\( 2k \ge m \\)，则 \\( k \ge { m \over 2 } \\)，

则 \\(n = k^2 \ge { m^2 \over 4 } \ge { 8 / 4 } \cdot m = 2m \Leftrightarrow n \ge 2m \\)，

这与 \\( n \lt 2m \\) 矛盾！

故假设不成立，故 \\(2 \le k \lt 2k \lt m \\)。

因此：\\( \exists C \in Z^+, m! = C \cdot (k \cdot 2k) = C \cdot 2k^2 = 2C \cdot n \Rightarrow n \mid m! \\)

** 综上，命题 \\( \ref{2} \\) 得证。**
