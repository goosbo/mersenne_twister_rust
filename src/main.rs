trait Mt {
    const W:u64 = 32;
    const N:usize = 624;
    const F:u64 = 1812433253;
    const M:usize = 397;
    const R:u32 = 31;
    const U:u32 = 11;
    const D:u32 = 0xffffffff;
    const S:u32 = 7;
    const B:u32 = 0x9d2c5680;
    const T:u32 = 15;
    const C:u32 = 0xEFC60000;
    const L:u32 = 18;
    const A:u32 = 0x9908b0df;

    fn seed(seed:u32)->MT;
    fn gen_num(&mut self) -> u32;
    fn twist(&mut self);
}


struct MT{
    state:Vec<u32>,
    index:usize
}

impl Mt for MT{
    fn seed(seed:u32)->MT{
        let index = 624;
        let mut state = vec![seed];
        for i in 1..Self::N{
            let t:u64 = Self::F * (state[i-1] ^ (state[i-1] >> (Self::W-2))) as u64 + (i as u64);
            state.push((t&0xffffffff) as u32);
        }
        MT{
            state:state,
            index:index
        }
    }

    fn gen_num(&mut self) -> u32 {
        if self.index >= Self::N{
            self.twist();
            self.index = 0;
        }
        let mut num = self.state[self.index];
        num ^= (num >> Self::U) & Self::D;
        num ^= (num << Self::S) & Self::B;
        num ^= (num <<Self::T) & Self::C;
        num ^= num >> 1;
        self.index+=1;
        return num & Self::D;
    }

    fn twist(&mut self) {
        for i in 0..Self::N{
            let mut t = (self.state[i] & 0x80000000) + (self.state[(i+1)%Self::N] & 0x7fffffff);
            t = t >>1;
            
            if t%2 == 1{
                t^= Self::A;
            }

            self.state[i] = self.state[(i+Self::M)%Self::N] ^ t;
        }
    }
}

fn main() {
    let mut e = MT::seed(0);
    println!("{}",e.gen_num());
    println!("{}",e.gen_num());
    println!("{}",e.gen_num());
}
