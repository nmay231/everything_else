use std::collections::{HashMap, VecDeque};

use advent_2024_rust::Zipper;
use itertools::Itertools;

type Output = usize;

#[derive(Debug, Clone, Default)]
struct Trie {
    map: HashMap<char, Self>,
    is_terminal: bool,
}

impl Trie {
    /// Checks if the pattern can be composed from pre-existing smaller patterns
    fn is_composable(&self, pattern: &[char]) -> bool {
        let mut frontier = VecDeque::from([(&pattern[..], self)]);

        while let Some((pattern, current_towel)) = frontier.pop_back() {
            match (pattern.first(), current_towel.is_terminal) {
                // Perfect match
                (None, true) => return true,
                // Leftover towel
                (None, false) => continue,
                // Need another towel to continue the pattern
                (Some(char), is_terminal) => {
                    if is_terminal {
                        frontier.push_back((pattern, self));
                    }
                    match current_towel.map.get(char) {
                        None => continue,
                        Some(child) => {
                            frontier.push_back((&pattern[1..], child));
                        }
                    }
                }
            }
        }

        return false;
    }

    /// Adds a new pattern and returns self
    fn add_new(self, word: &str) -> Self {
        // TODO: Is there a way to use my zipper pattern while only holding a
        // `&mut` reference?
        let mut zipper = TrieZipper::new(self);
        for char in word.chars() {
            zipper.source().map.entry(char).or_default();
            zipper = zipper.child(char).unwrap();
        }
        zipper.source().is_terminal = true;
        zipper = zipper.to_root();
        zipper.unzip()
    }
}

#[derive(Debug, Clone)]
struct TrieZipper {
    parents: Vec<(Trie, char)>,
    current: Trie,
}

impl Zipper for TrieZipper {
    type Source = Trie;

    type Index = char;

    fn new(root: Self::Source) -> Self {
        Self {
            current: root,
            parents: vec![],
        }
    }

    fn source(&mut self) -> &mut Self::Source {
        &mut self.current
    }

    fn child(mut self, index: Self::Index) -> Result<Self, Self> {
        match self.current.map.remove(&index) {
            None => Err(self),
            Some(child) => {
                self.parents.push((self.current, index));
                self.current = child;
                Ok(self)
            }
        }
    }

    fn parent(mut self) -> Result<Self, Self> {
        match self.parents.pop() {
            None => Err(self),
            Some((mut parent, key)) => {
                parent.map.insert(key, self.current);
                self.current = parent;
                Ok(self)
            }
        }
    }

    fn unwrap_source(self) -> Self::Source {
        self.current
    }
}

fn part1(text: &str) -> Output {
    let mut towels = text
        .split_once('\n')
        .expect("Missing towels")
        .0
        .split(", ")
        .collect_vec();
    towels.sort_by_key(|towel| towel.len());

    let mut trie = Trie::default();
    for towel in towels {
        let chars = towel.chars().collect_vec();
        if !trie.is_composable(&chars) {
            trie = trie.add_new(towel);
        }
    }

    let mut total = 0;
    'patterns: for pattern in text.lines().skip(2) {
        let pattern = pattern.chars().collect_vec();
        let mut frontier = VecDeque::from([(&pattern[..], &trie)]);

        while let Some((pattern, current_towel)) = frontier.pop_front() {
            match (pattern.first(), current_towel.is_terminal) {
                // Perfect match
                (None, true) => {
                    total += 1;
                    continue 'patterns;
                }
                // Leftover towel
                (None, false) => continue,
                // Need another towel to continue the pattern
                (Some(char), is_terminal) => {
                    if is_terminal {
                        frontier.push_back((pattern, &trie));
                    }
                    match current_towel.map.get(char) {
                        None => continue,
                        Some(child) => {
                            frontier.push_back((&pattern[1..], child));
                        }
                    }
                }
            }
        }
    }

    total
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day19.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        bwu
        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 7);
    }

    #[test]
    fn trie() {
        let towels = "bb, rgub, ubub, ugwbg, rwbb, rg, guuwbrw, gur, rrgb, bwgrbgg, ugg, wgu, ugbrbb, bbu, ubgw, guw, gubwu, grwr, rgbb, wrurw, rw, urww, uug, ugwu, ubr, rgwwbbb, urur, rwuwbwrr, gbbbburw, uwruu, rrrr, ubwgbbb, wrwr, uuu, uurubuuw, uwgw, gbbgu, wwbu, wuwr, brrug, bggggb, bgw, rrbr, wwuu, ubu, rggrub, rrgr, uwu, uuwguu, gbbbu, wuubb, rgb, bggwwr, g, gbw, wuu, bug, br, gwg, gbb, wgwrrr, rbu, bwwu, buubruwg, gwbub, bbrbw, wwwgb, wbwgw, www, rrw, uggwb, wubgw, gugrr, gruuwur, rrbgrg, bbwg, bgg, wgg, wuw, uugggrbg, wgrruu, uwuwrbw, wbbr, uuburr, wbg, ggbwurug, uubrwu, urug, gururb, bgbg, rurwg, brr, rbrg, bbubuu, rwrbw, uwgbwurb, wbuwb, bbw, rrrgbgb, ru, ugwru, bggrw, gggwbu, ggwuwu, rru, wrrug, rrrgrub, bwbww, guuu, wub, gu, buwbu, rwwww, wugbuwb, bgrr, uguuu, gbub, bww, grb, bwg, wwb, uurw, urwuu, grrbgu, rwg, rrbwb, bwu, wwug, ubwub, wrbg, wgrw, gww, rww, bbgrb, ub, rggbwu, bbuwbrgw, rb, wb, bwbugg, gurw, bubb, guug, wr, wrwg, uwgwbbr, bubwbrw, wru, rrbrbub, rgrg, rwgbrwb, rwbr, uwwrbbu, uwrbgw, gwub, uwbgr, ruugrrub, rubguggr, buuw, wwu, gbuub, gurg, wrg, bwub, rub, bgu, wbwrbb, ggb, bwbb, wwggbw, wgb, uwr, uwgur, bwb, u, urw, bgb, bbr, uwwu, bg, urr, rgbu, bugwg, rbbr, burb, bgruw, bwuu, rrugrgg, wrwubw, bubbgbg, bguwuru, ubg, ubwu, bwbbgbub, brguu, urwgrbu, guurrw, ubrgr, uubuwr, wrw, uuwr, ugur, ubwuwg, wbrw, wgr, rguuw, rwgwu, rbbgb, ggbgg, rgu, ug, gwrb, rrwgwr, rubuu, ugb, grrbr, grwb, bgugwwur, bbugruu, wbwg, ggu, guwbbw, bgr, grrrgw, brwwbgu, wburb, rgbg, bw, ruwb, brb, wwgbgwr, rbr, grr, ugbb, ruruu, wgbr, rwwbwg, wwg, ubgwu, urru, guuuwu, ururr, buuub, bru, bwrrwubu, wuww, rgbr, gub, wwrbg, grw, grg, wu, burguu, gbug, uggbr, brwub, uwb, grgru, rur, rgugugg, rubrbwg, rbrrw, gwrurgr, bur, uurbwu, ggw, rbwbu, wbw, uuw, ugu, bwr, wwwwru, gbwg, gbu, rgrwb, urbuw, rugwr, uwbbgrw, uurrbugg, wbgbrg, ubb, urb, burrb, gbgu, rwu, bbrruwb, uurrru, bugbw, b, ugguuw, uwgugwb, rug, wug, bwurgr, wgwubu, rgr, ugwb, gwwwgu, gwwbgggw, bwug, rbrug, uurb, ruw, wwwrb, brw, wggbubu, ugbrr, bbrgg, rruw, ggr, gbbw, wguwug, rbb, wuugbwgg, gug, rbuguu, gwb, ubrr, burrrb, rrwrrrg, ubw, rwgrrru, wugb, gggbbwg, uur, bu, urg, bgrb, brgrb, rbbwu, rgruu, gugruurr, bgbwb, rrbg, ruub, bgggrb, rugr, ggwb, uw, brrgr, wwugbrr, uruwub, wg, rwbugu, gru, bbrurr, rgbug, rrr, burw, gwgrubw, ww, wrburbb, rwwrwbu, uu, uww, urrw, wwwrbu, ugbugr, bbbb, wbuu, gbg, rrwuwb, rwgugg, ggrg, bub, gwr, rrwbw, rwr, rgg, r, guu, rrg, uggbbggr, wggbbb, wbuuwub, buwbw, ugrgu, gbbrr, gubgguu, bubwbg, ububw, uruw, wbr, gbwgrg, guwu, bwwur, rr, bggr, gg, rrwu, wrb, ruu, wbgb, ruwbwbuw, wbb, rwuug, rrb, ugw, bwbwuruu, ur, rbbbrrr, rgwg, gbr, grrb, wurr, uruwbr, burbww, ugrguw, wgw, wrgwug, buu, wbu, gubw, rgbgu, gbrrgg, buwb, wwr, bgwwwr, bbbug, rwwg, uuwgwbw, uwg, buw, gbbu, bbbw, bgrru, gwbu, ubwg, brg, wrr, ggg, uru, ugr, rwrww, bwgbbr, bwuw, brbgrrg, brrr, bwrb, gbgggu, gr, brubw, rwb, rbgr, bbb, gwu, wur";
        let towels = towels.split(", ").collect::<Vec<_>>();

        let mut trie = crate::Trie::default();

        for towel in towels.iter() {
            trie = trie.add_new(towel);
        }
        assert!(!trie.is_terminal, "There shouldn't be empty towel patterns");
        for towel in towels.iter() {
            let mut tmp = &trie;
            for char in towel.chars() {
                tmp = tmp.map.get(&char).expect("towels to be init'ed correctly");
            }
            assert!(tmp.is_terminal);
        }
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
