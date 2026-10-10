//! The questions of the core (MSG_SELECT_* etc.) and random valid responses to them
//! (as ygoai/actions.py random_response).

use ygo_core::carddb::CardDb;
use ygo_core::constants::*;
use ygo_core::playerop::is_declarable;

/// A small deterministic RNG (SplitMix64).
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    pub fn chance(&mut self, p: f64) -> bool {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64 * 1.0 < p
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
    pub fn sample(&mut self, n: usize, k: usize) -> Vec<usize> {
        let mut v: Vec<usize> = (0..n).collect();
        self.shuffle(&mut v);
        v.truncate(k);
        v.sort();
        v
    }
}

struct Reader<'a> {
    b: &'a [u8],
    p: usize,
}

impl Reader<'_> {
    fn u8(&mut self) -> u8 {
        let v = self.b[self.p];
        self.p += 1;
        v
    }
    fn u16(&mut self) -> u16 {
        let v = u16::from_le_bytes(self.b[self.p..self.p + 2].try_into().unwrap());
        self.p += 2;
        v
    }
    fn u32(&mut self) -> u32 {
        let v = u32::from_le_bytes(self.b[self.p..self.p + 4].try_into().unwrap());
        self.p += 4;
        v
    }
    fn u64(&mut self) -> u64 {
        let v = u64::from_le_bytes(self.b[self.p..self.p + 8].try_into().unwrap());
        self.p += 8;
        v
    }
    fn skip(&mut self, n: usize) {
        self.p += n;
    }
}

fn i32r(v: i32) -> Vec<u8> {
    v.to_le_bytes().to_vec()
}

fn select_indices(idx: &[usize]) -> Vec<u8> {
    let mut r = i32r(0);
    r.extend_from_slice(&(idx.len() as u32).to_le_bytes());
    for &i in idx {
        r.extend_from_slice(&(i as u32).to_le_bytes());
    }
    r
}

fn check_exact(params: &[u32], acc: i32) -> bool {
    fn rec(p: &[u32], i: usize, acc: i32) -> bool {
        if acc == 0 || i == p.len() {
            return false;
        }
        let o1 = (p[i] & 0xffff) as i32;
        let o2 = (p[i] >> 16) as i32;
        if i == p.len() - 1 {
            return acc == o1 || acc == o2;
        }
        (acc > o1 && rec(p, i + 1, acc - o1)) || (o2 > 0 && acc > o2 && rec(p, i + 1, acc - o2))
    }
    rec(params, 0, acc)
}

fn check_atleast(params: &[u32], acc: i32) -> bool {
    let (mut total, mut mx, mut mn) = (0i32, 0i32, 0x7fffffffi32);
    for &op in params {
        let o1 = (op & 0xffff) as i32;
        let o2 = (op >> 16) as i32;
        let ms = if o2 != 0 && o2 < o1 { o2 } else { o1 };
        total += ms;
        mx += if o2 > o1 { o2 } else { o1 };
        mn = mn.min(ms);
    }
    !(mx < acc || total - mn >= acc)
}

/// A random valid response to the question `msg` (the message with its type byte), or `None`
/// if the message is not a question.
pub fn random_response(msg: &[u8], rng: &mut Rng, db: &CardDb, codes: &[u32]) -> Option<Vec<u8>> {
    let typ = msg[0] as u32;
    let mut r = Reader { b: msg, p: 1 };
    let resp = match typ {
        MSG_SELECT_BATTLECMD => {
            r.u8();
            let nch = r.u32() as usize;
            r.skip(nch * 19);
            let nat = r.u32() as usize;
            r.skip(nat * 8);
            let (m2, ep) = (r.u8(), r.u8());
            let mut opts: Vec<i32> = vec![];
            opts.extend((0..nch).map(|i| ((i as i32) << 16) | 0));
            opts.extend((0..nat).map(|i| ((i as i32) << 16) | 1));
            if m2 != 0 {
                opts.push(2);
            }
            if ep != 0 {
                opts.push(3);
            }
            i32r(opts[rng.below(opts.len())])
        }
        MSG_SELECT_IDLECMD => {
            r.u8();
            let mut opts: Vec<i32> = vec![];
            for (kind, size) in [(0, 10), (1, 10), (2, 7), (3, 10), (4, 10), (5, 19)] {
                let n = r.u32() as usize;
                r.skip(n * size);
                opts.extend((0..n).map(|i| ((i as i32) << 16) | kind));
            }
            let (bp, ep, sh) = (r.u8(), r.u8(), r.u8());
            if bp != 0 {
                opts.push(6);
            }
            if ep != 0 {
                opts.push(7);
            }
            if sh != 0 {
                opts.push(8);
            }
            i32r(opts[rng.below(opts.len())])
        }
        MSG_SELECT_EFFECTYN | MSG_SELECT_YESNO => i32r(rng.below(2) as i32),
        MSG_SELECT_OPTION | MSG_ANNOUNCE_NUMBER => {
            r.u8();
            let n = r.u8() as usize;
            i32r(rng.below(n) as i32)
        }
        MSG_SELECT_CARD => {
            r.u8();
            let cancelable = r.u8() != 0;
            let (min, max, n) = (r.u32() as usize, r.u32() as usize, r.u32() as usize);
            if cancelable && rng.chance(0.05) {
                return Some(i32r(-1));
            }
            let hi = max.min(n);
            let k = if hi >= min {
                let lo = if hi >= 1 { min.max(1) } else { 0 };
                lo + rng.below(hi - lo + 1)
            } else {
                min
            };
            select_indices(&rng.sample(n, k))
        }
        MSG_SELECT_TRIBUTE => {
            r.u8();
            let cancelable = r.u8() != 0;
            let (min, max, n) = (r.u32() as usize, r.u32() as usize, r.u32() as usize);
            let mut params = vec![];
            for _ in 0..n {
                r.skip(4 + 1 + 1 + 4);
                params.push(r.u8() as usize);
            }
            if cancelable && rng.chance(0.05) {
                return Some(i32r(-1));
            }
            let mut order: Vec<usize> = (0..n).collect();
            rng.shuffle(&mut order);
            let mut chosen = vec![];
            let mut total = 0;
            for i in order {
                if total >= min && !chosen.is_empty() {
                    break;
                }
                chosen.push(i);
                total += params[i];
            }
            chosen.truncate(max.max(1));
            chosen.sort();
            select_indices(&chosen)
        }
        MSG_SELECT_SUM => {
            r.u8();
            let mode = r.u8();
            let (acc, min, max) = (r.u32() as i32, r.u32() as usize, r.u32() as usize);
            let nm = r.u32() as usize;
            let mut must = vec![];
            for _ in 0..nm {
                r.skip(14);
                must.push(r.u32());
            }
            let n = r.u32() as usize;
            let mut params = vec![];
            for _ in 0..n {
                r.skip(14);
                params.push(r.u32());
            }
            let check = |sel: &[usize]| {
                let mut all = must.clone();
                all.extend(sel.iter().map(|&i| params[i]));
                if mode == 0 {
                    min <= sel.len() && sel.len() <= max && check_exact(&all, acc)
                } else {
                    check_atleast(&all, acc)
                }
            };
            let mut order: Vec<usize> = (0..n).collect();
            rng.shuffle(&mut order);
            let mut steps = 0;
            fn dfs(order: &[usize], start: usize, chosen: &mut Vec<usize>, steps: &mut usize, check: &dyn Fn(&[usize]) -> bool, dead: &dyn Fn(&[usize]) -> bool) -> Option<Vec<usize>> {
                *steps += 1;
                if *steps > 20000 {
                    return None;
                }
                if !chosen.is_empty() && check(chosen) {
                    return Some(chosen.clone());
                }
                // no superset of an overshooting selection can be valid
                if dead(chosen) {
                    return None;
                }
                for k in start..order.len() {
                    chosen.push(order[k]);
                    if let Some(r) = dfs(order, k + 1, chosen, steps, check, dead) {
                        return Some(r);
                    }
                    chosen.pop();
                }
                None
            }
            // "at least" mode: once the sum minus its smallest part reaches acc, adding cards only grows it
            let dead = |sel: &[usize]| {
                if mode == 0 {
                    return false;
                }
                let (mut total, mut mn) = (0i32, i32::MAX);
                for &op in must.iter().chain(sel.iter().map(|&i| &params[i])) {
                    let (o1, o2) = ((op & 0xffff) as i32, (op >> 16) as i32);
                    let ms = if o2 != 0 && o2 < o1 { o2 } else { o1 };
                    total += ms;
                    mn = mn.min(ms);
                }
                total - mn >= acc
            };
            let mut sel = dfs(&order, 0, &mut vec![], &mut steps, &check, &dead).unwrap_or_default();
            sel.sort();
            select_indices(&sel)
        }
        MSG_SELECT_UNSELECT_CARD => {
            r.u8();
            let finishable = r.u8() != 0;
            let cancelable = r.u8() != 0;
            r.u32();
            r.u32();
            let n1 = r.u32() as usize;
            r.skip(n1 * 14);
            let n2 = r.u32() as usize;
            if (finishable || cancelable) && (n1 == 0 || rng.chance(0.5)) {
                return Some(i32r(-1));
            }
            let idx = if n1 > 0 { rng.below(n1) } else { n1 + rng.below(n2) };
            let mut v = i32r(1);
            v.extend(i32r(idx as i32));
            v
        }
        MSG_SELECT_CHAIN => {
            r.u8();
            r.u8();
            let forced = r.u8() != 0;
            r.u32();
            r.u32();
            let n = r.u32() as usize;
            if forced {
                i32r(rng.below(n) as i32)
            } else {
                i32r(rng.below(n + 1) as i32 - 1)
            }
        }
        MSG_SELECT_PLACE | MSG_SELECT_DISFIELD => {
            let player = r.u8();
            let count = r.u8() as usize;
            let flag = r.u32();
            let mut places = vec![];
            for own in [true, false] {
                let p = if own { player } else { 1 - player };
                for (loc, nseq, shift) in [(LOCATION_MZONE, 7, 0), (LOCATION_SZONE, 8, 8)] {
                    for seq in 0..nseq {
                        let bit = 1u32 << (seq + shift + if own { 0 } else { 16 });
                        if flag & bit == 0 {
                            places.push((p, loc as u8, seq as u8));
                        }
                    }
                }
            }
            let k = count.min(places.len());
            let mut v = vec![];
            let mut idx: Vec<usize> = (0..places.len()).collect();
            rng.shuffle(&mut idx);
            for &i in &idx[..k] {
                let (p, l, s) = places[i];
                v.extend([p, l, s]);
            }
            v
        }
        MSG_SELECT_POSITION => {
            r.u8();
            r.u32();
            let positions = r.u8() as u32;
            let opts: Vec<u32> = [POS_FACEUP_ATTACK, POS_FACEDOWN_ATTACK, POS_FACEUP_DEFENSE, POS_FACEDOWN_DEFENSE].into_iter().filter(|p| positions & p != 0).collect();
            i32r(opts[rng.below(opts.len())] as i32)
        }
        MSG_SELECT_COUNTER => {
            r.u8();
            r.u16();
            let mut remaining = r.u16();
            let n = r.u32() as usize;
            let mut have = vec![];
            for _ in 0..n {
                r.skip(7);
                have.push(r.u16());
            }
            let mut counts = vec![0u16; n];
            while remaining > 0 {
                let cands: Vec<usize> = (0..n).filter(|&i| counts[i] < have[i]).collect();
                let i = cands[rng.below(cands.len())];
                counts[i] += 1;
                remaining -= 1;
            }
            counts.iter().flat_map(|c| (*c as i16).to_le_bytes()).collect()
        }
        MSG_SORT_CARD | MSG_SORT_CHAIN => {
            r.u8();
            let n = r.u32() as usize;
            let mut order: Vec<usize> = (0..n).collect();
            rng.shuffle(&mut order);
            order.iter().map(|&o| o as u8).collect()
        }
        MSG_ANNOUNCE_RACE | MSG_ANNOUNCE_ATTRIB => {
            r.u8();
            let count = r.u8() as usize;
            let avail = if typ == MSG_ANNOUNCE_RACE { r.u64() } else { r.u32() as u64 };
            let bits: Vec<u64> = (0..64).map(|i| 1u64 << i).filter(|b| avail & b != 0).collect();
            let pick = rng.sample(bits.len(), count.min(bits.len()));
            let mask: u64 = pick.iter().map(|&i| bits[i]).sum();
            if typ == MSG_ANNOUNCE_RACE { mask.to_le_bytes().to_vec() } else { (mask as u32).to_le_bytes().to_vec() }
        }
        MSG_ANNOUNCE_CARD => {
            r.u8();
            let n = r.u8() as usize;
            let ops: Vec<u64> = (0..n).map(|_| r.u64()).collect();
            let ok: Vec<u32> = codes.iter().copied().filter(|c| is_declarable(&db.cards[c], &ops)).collect();
            i32r(if ok.is_empty() { 0 } else { ok[rng.below(ok.len())] as i32 })
        }
        MSG_ROCK_PAPER_SCISSORS => i32r(1 + rng.below(3) as i32),
        _ => return None,
    };
    Some(resp)
}
