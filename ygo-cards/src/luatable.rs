//! The iteration order of Lua 5.4.8 tables with integer keys (`ltable.c`: array part, hash part
//! with `hashmod` main positions and `lastfree` collisions, rehash/resize/reinsert, `luaH_next`).
//! Scripts whose results depend on `pairs`/`next` order use this.

#[derive(Clone, Debug)]
struct Node<V> {
    /// `None`: a nil key (never used)
    key: Option<i64>,
    /// `None`: an empty value
    val: Option<V>,
    next: i64,
}

/// A Lua table with integer keys.
#[derive(Clone, Debug)]
pub struct LuaTable<V> {
    array: Vec<Option<V>>,
    /// The hash part (empty: the dummy node).
    node: Vec<Node<V>>,
    lastfree: usize,
}

impl<V: Clone> Default for LuaTable<V> {
    fn default() -> Self {
        LuaTable::new()
    }
}

fn ceillog2(x: u64) -> u32 {
    // luaO_ceillog2: ceil(log2(x)) for x >= 1
    let mut l = 0;
    let mut x = x - 1;
    while x >= 256 {
        l += 8;
        x >>= 8;
    }
    const LOG_2: [u8; 256] = {
        let mut t = [0u8; 256];
        let mut i = 1;
        while i < 256 {
            let mut v = i;
            let mut n = 0;
            while v > 0 {
                n += 1;
                v >>= 1;
            }
            t[i] = n;
            i += 1;
        }
        t
    };
    l + LOG_2[x as usize] as u32
}
/// `arrayindex`: the key as an array index (0 if it cannot be one)
fn arrayindex(k: i64) -> u64 {
    let u = k as u64;
    if u.wrapping_sub(1) < (1u64 << 31) { u } else { 0 }
}

impl<V: Clone> LuaTable<V> {
    /// `{}`
    pub fn new() -> Self {
        LuaTable { array: Vec::new(), node: Vec::new(), lastfree: 0 }
    }
    /// A table constructor with `asize` list items and `hsize` keyed items (`OP_NEWTABLE` sizes)
    pub fn with_sizes(asize: usize, hsize: usize) -> Self {
        let mut t = LuaTable::new();
        if asize > 0 || hsize > 0 {
            t.resize(asize, hsize);
        }
        t
    }
    fn sizenode(&self) -> usize {
        if self.node.is_empty() { 1 } else { self.node.len() }
    }
    fn isdummy(&self) -> bool {
        self.node.is_empty()
    }
    /// `hashint`: the main position of an integer key
    fn mainposition(&self, k: i64) -> usize {
        if self.isdummy() {
            return 0;
        }
        let m = ((self.sizenode() - 1) | 1) as u64;
        let ui = k as u64;
        if ui <= i32::MAX as u64 { (ui as i32 % m as i32) as usize } else { (ui % m) as usize }
    }
    /// The array slot or hash node of `k` (`luaH_getint`): `Ok(array index)`, `Err(Some(node))`,
    /// `Err(None)` when absent.
    fn find(&self, k: i64) -> Result<usize, Option<usize>> {
        if (k as u64).wrapping_sub(1) < self.array.len() as u64 {
            return Ok((k - 1) as usize);
        }
        if self.isdummy() {
            return Err(None);
        }
        let mut n = self.mainposition(k);
        loop {
            if self.node[n].key == Some(k) {
                return Err(Some(n));
            }
            let nx = self.node[n].next;
            if nx == 0 {
                return Err(None);
            }
            n = (n as i64 + nx) as usize;
        }
    }
    pub fn get(&self, k: i64) -> Option<&V> {
        match self.find(k) {
            Ok(i) => self.array[i].as_ref(),
            Err(Some(n)) => self.node[n].val.as_ref(),
            Err(None) => None,
        }
    }
    /// `t[k] = v` (`None`: nil)
    pub fn set(&mut self, k: i64, v: Option<V>) {
        match self.find(k) {
            Ok(i) => self.array[i] = v,
            Err(Some(n)) => self.node[n].val = v,
            Err(None) => self.newkey(k, v),
        }
    }
    fn getfreepos(&mut self) -> Option<usize> {
        if !self.isdummy() {
            while self.lastfree > 0 {
                self.lastfree -= 1;
                if self.node[self.lastfree].key.is_none() {
                    return Some(self.lastfree);
                }
            }
        }
        None
    }
    /// `luaH_newkey`
    fn newkey(&mut self, k: i64, v: Option<V>) {
        let Some(v) = v else { return };
        let mut mp = self.mainposition(k);
        if self.isdummy() || self.node[mp].val.is_some() {
            let Some(f) = self.getfreepos() else {
                self.rehash(k);
                self.set(k, Some(v));
                return;
            };
            let othern0 = self.mainposition(self.node[mp].key.expect("colliding node without a key"));
            if othern0 != mp {
                // the colliding node is out of its main position: move it to the free position
                let mut othern = othern0;
                while (othern as i64 + self.node[othern].next) as usize != mp {
                    othern = (othern as i64 + self.node[othern].next) as usize;
                }
                self.node[othern].next = f as i64 - othern as i64;
                self.node[f] = self.node[mp].clone();
                if self.node[mp].next != 0 {
                    self.node[f].next += mp as i64 - f as i64;
                    self.node[mp].next = 0;
                }
                self.node[mp].val = None;
            } else {
                // the new node goes into the free position
                if self.node[mp].next != 0 {
                    self.node[f].next = (mp as i64 + self.node[mp].next) - f as i64;
                }
                self.node[mp].next = f as i64 - mp as i64;
                mp = f;
            }
        }
        self.node[mp].key = Some(k);
        self.node[mp].val = Some(v);
    }
    fn countint(k: i64, nums: &mut [u32]) -> u32 {
        let a = arrayindex(k);
        if a != 0 {
            nums[ceillog2(a) as usize] += 1;
            1
        } else {
            0
        }
    }
    /// `rehash`
    fn rehash(&mut self, ek: i64) {
        let mut nums = [0u32; 32];
        // numusearray
        let mut na: u32 = 0;
        let asize = self.array.len() as u64;
        let mut i: u64 = 1;
        let mut ttlg: u64 = 1;
        for lg in 0..=31usize {
            let mut lc = 0;
            let mut lim = ttlg;
            if lim > asize {
                lim = asize;
                if i > lim {
                    break;
                }
            }
            while i <= lim {
                if self.array[(i - 1) as usize].is_some() {
                    lc += 1;
                }
                i += 1;
            }
            nums[lg] += lc;
            na += lc;
            ttlg *= 2;
        }
        let mut totaluse = na;
        // numusehash
        for n in self.node.iter().rev() {
            if n.val.is_some() {
                if let Some(k) = n.key {
                    na += Self::countint(k, &mut nums);
                }
                totaluse += 1;
            }
        }
        na += Self::countint(ek, &mut nums);
        totaluse += 1;
        // computesizes
        let (mut a, mut nna, mut optimal) = (0u32, 0u32, 0u32);
        let mut twotoi: u32 = 1;
        let mut lg = 0usize;
        while twotoi > 0 && na > twotoi / 2 {
            a += nums[lg];
            if a > twotoi / 2 {
                optimal = twotoi;
                nna = a;
            }
            lg += 1;
            twotoi = twotoi.wrapping_mul(2);
        }
        self.resize(optimal as usize, (totaluse - nna) as usize);
    }
    /// `luaH_resize`
    fn resize(&mut self, newasize: usize, nhsize: usize) {
        let oldasize = self.array.len();
        let newnode: Vec<Node<V>> = if nhsize == 0 {
            Vec::new()
        } else {
            let size = 1usize << ceillog2(nhsize as u64);
            vec![Node { key: None, val: None, next: 0 }; size]
        };
        let newlastfree = newnode.len();
        let oldnode = std::mem::replace(&mut self.node, newnode);
        let oldlastfree = std::mem::replace(&mut self.lastfree, newlastfree);
        let _ = oldlastfree;
        if newasize < oldasize {
            // re-insert the vanishing slice into the new hash
            let vanishing: Vec<(usize, Option<V>)> = (newasize..oldasize).map(|i| (i, self.array[i].clone())).collect();
            self.array.truncate(newasize);
            for (i, v) in vanishing {
                if v.is_some() {
                    self.set(i as i64 + 1, v);
                }
            }
        }
        self.array.resize(newasize, None);
        // re-insert the old hash part
        for n in oldnode {
            if let (Some(k), Some(v)) = (n.key, n.val) {
                self.set(k, Some(v));
            }
        }
    }
    /// The keys and values in `pairs` order (`luaH_next`)
    pub fn pairs(&self) -> Vec<(i64, V)> {
        let mut out = Vec::new();
        for (i, v) in self.array.iter().enumerate() {
            if let Some(v) = v {
                out.push((i as i64 + 1, v.clone()));
            }
        }
        for n in &self.node {
            if let (Some(k), Some(v)) = (n.key, &n.val) {
                out.push((k, v.clone()));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::LuaTable;

    /// Compares with the `lua` on the PATH (Lua 5.4.8) on random insert sequences.
    #[test]
    fn same_order_as_lua() {
        if std::process::Command::new("lua").arg("-v").output().is_err() {
            return;
        }
        let mut seed: u64 = 12345;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for case in 0..300 {
            let n = (rnd() % 40) as usize;
            let keys: Vec<i64> = (0..n)
                .map(|_| match case % 3 {
                    0 => (rnd() % 100_000_000) as i64,
                    1 => (rnd() % 20) as i64 + 1,
                    _ => (rnd() % 200) as i64 - 50,
                })
                .collect();
            let lua = format!(
                "local t={{}} for _,k in ipairs({{{}}}) do t[k]=true end local o={{}} for k in pairs(t) do o[#o+1]=k end print(table.concat(o,' '))",
                keys.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(",")
            );
            let out = std::process::Command::new("lua").arg("-e").arg(&lua).output().unwrap();
            let expected = String::from_utf8(out.stdout).unwrap().trim().to_string();
            let mut t = LuaTable::new();
            for &k in &keys {
                t.set(k, Some(true));
            }
            let got = t.pairs().iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>().join(" ");
            assert_eq!(got, expected, "keys {keys:?}");
        }
    }

    /// The same with deletions (`t[k] = nil`) between the inserts.
    #[test]
    fn same_order_as_lua_with_deletions() {
        if std::process::Command::new("lua").arg("-v").output().is_err() {
            return;
        }
        let mut seed: u64 = 777;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for case in 0..300 {
            let n = (rnd() % 50) as usize;
            let ops: Vec<(i64, bool)> = (0..n)
                .map(|_| {
                    let k = if case % 2 == 0 { (rnd() % 30) as i64 + 1 } else { (rnd() % 5_000_000) as i64 };
                    (k, rnd() % 4 != 0)
                })
                .collect();
            let lua = format!(
                "local t={{}} for _,o in ipairs({{{}}}) do if o[2] then t[o[1]]=true else t[o[1]]=nil end end local o={{}} for k in pairs(t) do o[#o+1]=k end print(table.concat(o,' '))",
                ops.iter().map(|(k, ins)| format!("{{{k},{ins}}}")).collect::<Vec<_>>().join(",")
            );
            let out = std::process::Command::new("lua").arg("-e").arg(&lua).output().unwrap();
            let expected = String::from_utf8(out.stdout).unwrap().trim().to_string();
            let mut t = LuaTable::new();
            for &(k, ins) in &ops {
                t.set(k, if ins { Some(true) } else { None });
            }
            let got = t.pairs().iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>().join(" ");
            assert_eq!(got, expected, "ops {ops:?}");
        }
    }
}
