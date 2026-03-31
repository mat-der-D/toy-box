//! # 2^x + 3^y + 5 = z^3 の整数解探索
//!
//! ## アルゴリズムの概要
//!
//! z^3 = 2^x + 3^y + 5 が成り立つなら、任意の素数 p に対して
//!
//! ```text
//!   2^x + 3^y + 5 ≡ z^3 (mod p)
//! ```
//!
//! が必要条件として成り立つ。右辺は必ず p の**立方剰余**でなければならない。
//!
//! 一方、2^x (mod p) の値は x mod ord_p(2) にしか依存しない（周期性）。
//! これにより「どの (x mod ord_p(2), y mod ord_p(3)) が許されるか」を
//! 各素数で列挙し、中国剰余定理（CRT）で合成していくことで
//! 解の候補となる剰余類を段階的に絞り込める。
//!
//! ## 出力の読み方
//!
//! 最終的な candidates の各エントリ (r_x, r_y) は
//!
//! ```text
//!   解が存在するなら  x ≡ r_x (mod M_x)  かつ  y ≡ r_y (mod M_y)
//! ```
//!
//! を満たすことを意味する必要条件（十分条件ではない）。
//! M_x, M_y は使用した全素数の位数の LCM であり、最終行に表示される。

use rustc_hash::{FxHashMap, FxHashSet};

/// 候補集合: (x mod M_x, y mod M_y) のペアの集合
type Candidates = FxHashSet<(u64, u64)>;

// ── 算術ユーティリティ ──────────────────────────────────────────────────────

fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// n の素因数をすべて列挙する（重複なし、昇順）
fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

/// 拡張ユークリッドアルゴリズム: (g, x, y) を返す（a*x + b*y = g = gcd(a, b)）
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x, y) = extended_gcd(b, a % b);
    (g, y, x - (a / b) * y)
}

/// a の mod m における乗法的逆元。gcd(a, m) ≠ 1 なら None
fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    if m == 1 {
        return Some(0);
    }
    let (g, x, _) = extended_gcd(a as i64, m as i64);
    (g == 1).then(|| x.rem_euclid(m as i64) as u64)
}

// ── 数論 ───────────────────────────────────────────────────────────────────

/// オイラーのトーシェント関数 φ(n)
fn euler_phi(n: u64) -> u64 {
    let mut result = n;
    for p in prime_factors(n) {
        result = result / p * (p - 1);
    }
    result
}

/// base の mod m における乗法的位数
///
/// φ(m) の約数の中から最小の位数を求める（素数・合成数どちらにも対応）。
fn multiplicative_order(base: u64, m: u64) -> u64 {
    let phi = euler_phi(m);
    let mut order = phi;
    for q in prime_factors(phi) {
        while order % q == 0 && pow_mod(base, order / q, m) == 1 {
            order /= q;
        }
    }
    order
}

/// mod p の立方剰余全体: { z³ mod p | z ∈ 0..p }
fn cubic_residues(p: u64) -> FxHashSet<u64> {
    (0..p).map(|z| pow_mod(z, 3, p)).collect()
}

// ── コアアルゴリズム ────────────────────────────────────────────────────────

/// mod p で 2^x + 3^y + 5 が立方剰余になる (x mod ox, y mod oy) の集合を返す
fn allowed_mod_p(p: u64) -> (u64, u64, Candidates) {
    let ox = multiplicative_order(2, p);
    let oy = multiplicative_order(3, p);

    let xs: Vec<u64> = (0..ox).map(|x| pow_mod(2, x, p)).collect();
    let ys: Vec<u64> = (0..oy).map(|y| pow_mod(3, y, p)).collect();
    let cs = cubic_residues(p);

    let mut ok = Candidates::default();
    for (xi, &u) in xs.iter().enumerate() {
        for (yi, &v) in ys.iter().enumerate() {
            if cs.contains(&((u + v + 5) % p)) {
                ok.insert((xi as u64, yi as u64));
            }
        }
    }

    (ox, oy, ok)
}

/// 中国剰余定理: x ≡ a (mod m) かつ x ≡ b (mod n) の解を返す（sympy.ntheory.modular.crt に対応）
///
/// gcd(m, n) が (b-a) を割り切らない場合は解なし（None）。
/// 解が存在する場合、結果は mod lcm(m, n) で一意。
fn merge_congruences(a: u64, m: u64, b: u64, n: u64) -> Option<u64> {
    let g = gcd(m, n);
    let diff = b as i128 - a as i128;
    if diff % g as i128 != 0 {
        return None;
    }
    let n_reduced = n / g;
    let inv = mod_inverse(m / g, n_reduced)?;
    let t = ((diff / g as i128).rem_euclid(n_reduced as i128) as u64 * inv) % n_reduced;
    let combined_mod = lcm(m, n);
    Some(((a as u128 + m as u128 * t as u128) % combined_mod as u128) as u64)
}

/// x >= 6 のとき 2^x ≡ 0 (mod 64) を利用した y の絞り込み
///
/// 2^x の寄与がゼロに固定されるため、条件は (0 + 3^y + 5) が mod 64 の立方剰余かどうかのみ。
/// ox = 1 を返すことで intersect に「x は無制約」と伝える。
fn allowed_y_mod64_x_ge6() -> (u64, u64, Candidates) {
    let m = 64u64;
    let oy = multiplicative_order(3, m);
    let cs = cubic_residues(m);
    let mut ok = Candidates::default();
    for y in 0..oy {
        if cs.contains(&((pow_mod(3, y, m) + 5) % m)) {
            ok.insert((0, y));
        }
    }
    (1, oy, ok)
}

/// y >= 2 のとき 3^y ≡ 0 (mod 9) を利用した x の絞り込み
///
/// 3^y の寄与がゼロに固定されるため、条件は (2^x + 0 + 5) が mod 9 の立方剰余かどうかのみ。
/// oy = 1 を返すことで intersect に「y は無制約」と伝える。
fn allowed_x_mod9_y_ge2() -> (u64, u64, Candidates) {
    let m = 9u64;
    let ox = multiplicative_order(2, m);
    let cs = cubic_residues(m);
    let mut ok = Candidates::default();
    for x in 0..ox {
        if cs.contains(&((pow_mod(2, x, m) + 5) % m)) {
            ok.insert((x, 0));
        }
    }
    (ox, 1, ok)
}

/// 現在の候補集合に新しい条件を CRT で合成し、整合する候補のみ残す
///
/// ```text
/// 入力:
///   candidates  … 「x ≡ big_x (mod mx), y ≡ big_y (mod my)」という既存の候補集合
///   (ox, oy, ok) … 新たな素数から得た「x ≡ x0 (mod ox), y ≡ y0 (mod oy)」という条件
///
/// 処理:
///   既存の各候補 (big_x, big_y) と新条件の各ペア (x0, y0) に対して
///   CRT で連立合同式を解く。解が存在するものだけを新しい候補として残す。
///
/// 出力:
///   「x ≡ ? (mod lcm(mx, ox)), y ≡ ? (mod lcm(my, oy))」に更新された候補集合
/// ```
fn intersect(
    candidates: Candidates,
    mx: u64,
    my: u64,
    ox: u64,
    oy: u64,
    ok: &Candidates,
) -> (Candidates, u64, u64) {
    let nx = lcm(mx, ox);
    let ny = lcm(my, oy);

    // ok の各エントリを x → [y, ...] に整理して内側ループを効率化
    let mut ys_by_x: FxHashMap<u64, Vec<u64>> = FxHashMap::default();
    for &(x, y) in ok {
        ys_by_x.entry(x).or_default().push(y);
    }

    // CRT 結果をキャッシュして再計算を避ける
    let mut x_cache: FxHashMap<(u64, u64), Option<u64>> = FxHashMap::default();
    let mut y_cache: FxHashMap<(u64, u64), Option<u64>> = FxHashMap::default();
    let mut out = Candidates::default();

    for &(big_x, big_y) in &candidates {
        for (&x, ys) in &ys_by_x {
            let x2 = *x_cache
                .entry((big_x, x))
                .or_insert_with(|| merge_congruences(big_x, mx, x, ox));
            let Some(x2) = x2 else { continue };

            for &y in ys {
                let y2 = *y_cache
                    .entry((big_y, y))
                    .or_insert_with(|| merge_congruences(big_y, my, y, oy));
                let Some(y2) = y2 else { continue };

                out.insert((x2 % nx, y2 % ny));
            }
        }
    }

    (out, nx, ny)
}

fn main() {
    // ── フェーズ 1: 素数による立方剰余スクリーニング ─────────────────────────
    //
    // p ≡ 1 (mod 3) の素数では立方剰余が p の 1/3 しかないため、
    // 2^x + 3^y + 5 ≡ z^3 (mod p) という条件が有効な絞り込みになる。
    // 各素数の条件を CRT で合成し、候補の剰余類を段階的に減らしていく。
    let primes: &[u64] = &[
        7, 13, 19, 31, 37, 43, 61, 67, 73, 97, 109, 127, 181, 193, 199, 211, 241, 331, 337, 379,
        397, 421, 433, 463, 541, 577, 661, 673, 991, 2113, 2311,
    ];

    let mut candidates: Candidates = [(0u64, 0u64)].into_iter().collect();
    let (mut mx, mut my) = (1u64, 1u64);

    for &p in primes {
        let (ox, oy, ok) = allowed_mod_p(p);
        (candidates, mx, my) = intersect(candidates, mx, my, ox, oy, &ok);
        println!("p={p:6}  mod=({mx},{my})  candidates={}", candidates.len());
    }

    // ── フェーズ 2: x >= 6, y >= 2 の制約を合成数で反映 ─────────────────────
    //
    // x >= 6 のとき 2^6 = 64 より 2^x ≡ 0 (mod 64)。
    // よって mod 64 での条件は「3^y + 5 が立方剰余 mod 64」に帰着し、
    // y の剰余類のみを絞り込める（x は無制約: ox = 1）。
    let (ox, oy, ok) = allowed_y_mod64_x_ge6();
    (candidates, mx, my) = intersect(candidates, mx, my, ox, oy, &ok);
    println!(
        "mod64(x>=6)  mod=({mx},{my})  candidates={}",
        candidates.len()
    );

    // y >= 2 のとき 3^2 = 9 より 3^y ≡ 0 (mod 9)。
    // よって mod 9 での条件は「2^x + 5 が立方剰余 mod 9」に帰着し、
    // x の剰余類のみを絞り込める（y は無制約: oy = 1）。
    let (ox, oy, ok) = allowed_x_mod9_y_ge2();
    (candidates, mx, my) = intersect(candidates, mx, my, ox, oy, &ok);
    println!(
        "mod9 (y>=2)  mod=({mx},{my})  candidates={}",
        candidates.len()
    );

    // ── 結果出力 ──────────────────────────────────────────────────────────────
    //
    // 各行 (r_x, r_y) は「解が存在するなら x ≡ r_x (mod M_x), y ≡ r_y (mod M_y)」
    // という必要条件を表す。
    println!();
    let mut result: Vec<(u64, u64)> = candidates.into_iter().collect();
    result.sort();
    for (rx, ry) in result {
        println!("x ≡ {rx:6} (mod {mx}),  y ≡ {ry:6} (mod {my})");
    }
}
