use super::Duellist;

#[derive(Debug)]
#[non_exhaustive]
pub enum Outcome<'a> {
    #[non_exhaustive]
    Won {
        winner: &'a mut Duellist,
        loser: &'a mut Duellist,
    },
    #[non_exhaustive]
    Tie {
        p1: &'a mut Duellist,
        p2: &'a mut Duellist,
    },
}

impl<'a> Outcome<'a> {
    pub fn new(p1: &'a mut Duellist, p2: &'a mut Duellist) -> Self {
        let s1 = p1.status().score();
        let s2 = p2.status().score();

        #[allow(clippy::comparison_chain)]
        if s1 > s2 {
            Self::Won {
                winner: p1,
                loser: p2,
            }
        } else if s2 > s1 {
            Self::Won {
                winner: p2,
                loser: p1,
            }
        } else {
            Self::Tie { p1, p2 }
        }
    }
}
