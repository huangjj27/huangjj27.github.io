# 整数和它两倍（一倍半）之间的合数，可以整除整数的阶乘
## 形式化描述
\\[ \label{2.1} \tag{2.1}
    \forall m \in Z^+, m \gt 8, n \in Z^+, m \lt n \lt 2m; \quad n \not \in Primes \Leftrightarrow n \mid m!
\\]

## 证明
### 充分性
\\[ \label{2.1.1} \tag{2.1.1}
    n \not \in Primes \Rightarrow n \mid m!
\\]

因为 \\( n \\) 为合数，将其分为完全平方数和不完全平方数两种情况证明。

#### 不完全平方数
不妨设 \\( n = ab (a \gt b \ge 2, (a, b) = 1) \\)，则 \\(b \lt a \lt m \\)，证明如下：

若 \\( a \ge m \\)，

则 \\(n = ab \ge 2a \ge 2m \Leftrightarrow n \ge 2m \\)，

这与 \\( n \lt 2m \\) 矛盾！

故假设不成立，故 \\(2 \le b \lt a \lt m \\)。

又 \\( (a, b) = 1 \Rightarrow [a, b] = ab = n \\)

因此：\\( \exists C \in Z^+, m! = C \cdot ab = C \cdot n \Rightarrow n \mid m! \\)

#### 完全平方数
不妨设 \\( n = k^2 \\)，则 \\(2 \lt k \lt 2k \lt m \\)，证明如下：

若 \\( 2k \ge m \\)，则 \\( k \ge { m \over 2 } \\)，

则 \\(n = k^2 \ge { m^2 \over 4 } \ge { 8 / 4 } \cdot m = 2m \Leftrightarrow n \ge 2m \\)，

这与 \\( n \lt 2m \\) 矛盾！

故假设不成立，故 \\(2 \le k \lt 2k \lt m \\)。

因此：\\( \exists C \in Z^+, m! = C \cdot (k \cdot 2k) = C \cdot 2k^2 = 2C \cdot n \Rightarrow n \mid m! \\)

**综上，命题 \\( \ref{2.1.1} \\) 得证。**

### 必要性
\\[ \label{2.1.2} \tag{2.1.2}
    n \mid m! \Rightarrow n \not \in Primes
\\]

反证：假设 \\(n \in Primes \\)，则：

\\( \because n > m, n \in Primes, \\)

\\( \therefore n \nmid 2, \space n \nmid 3, \space n \nmid 4, \space \dots, \space n \mid m \Rightarrow n \nmid m!， \\)

这与条件 \\( n \mid m! \\) 矛盾！故假设不成立，命题 \\( \ref{2.1.2} \\) 得证。

**综上，命题 \\( \ref{2.1} \\) 得证。**

## 更强的结论
显然对 \\( \forall n \in (m, 3/2m) \Rightarrow n \in (m, 2n) \\)，故：
\\[ \label{2.2} \tag{2.2}
    \forall m \in Z^+, m \gt 8, n \in Z^+, m \lt n \lt { 3m \over 2 }; \quad n \not \in Primes \Leftrightarrow n \mid m!
\\]
