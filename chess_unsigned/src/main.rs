use std::collections::HashSet;
use rand::Rng;

pub const A_FILE: u64 = 72340172838076673;
pub const H_FILE: u64 = 9259542123273814144;
pub const GH_FILE : u64 = 13889313184910721216;
pub const AB_FILE: u64 = 217020518514230019;

pub const LSB_TABLE: [u64; 64] = [
    0,  1, 48,  2, 57, 49, 28,  3,
    61, 58, 50, 42, 38, 29, 17,  4,
    62, 55, 59, 36, 53, 51, 43, 22,
    45, 39, 33, 30, 24, 18, 12,  5,
    63, 47, 56, 27, 60, 41, 37, 16,
    54, 35, 52, 21, 44, 32, 23, 11,
    46, 26, 40, 15, 34, 20, 31, 10,
    25, 14, 19,  9, 13,  8,  7,  6
];

pub const bbits: [i32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

pub const rbits: [i32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12
];

static mut BISHOP_MAGIC: [u64; 64] = [0; 64];
static mut ROOK_MAGIC: [u64; 64] = [0; 64];

static mut BISHOP_MASKS: [u64; 64] = [0; 64];
static mut ROOK_MASKS: [u64; 64] = [0; 64];

pub fn setbit(bb: u64, sq: i32) -> u64 {
    if sq < 0 || sq > 63 {
        return bb;
    }

    return bb | (1u64 << sq as u64);
}

pub fn getbit(bb: u64, sq: i32) -> bool {
    if sq < 0 || sq > 63 {
        return false;
    }
    return (bb & (1u64 << sq as u64)) != 0;
}

pub fn clearbit(bb: u64, sq: i32) -> u64 {
    if sq < 0 || sq > 63 {
        return bb;
    }
    return bb ^ (1u64 << sq as u64);
}

pub fn printbb(bb: u64) {
    for x in 0..8 {
        for y in 0..8 {
            let sq: i32 = x * 8 + y;
            if getbit(bb, sq) {
                print!("x ");
            }else {
                print!("- ");
            }
        }
        println!();
    }
    println!();
}

pub fn getls1b(bb: u64) -> u64 {
    const DEBRUIJN_64: u64 = 0x03f79d71b4cb0a89;
    return LSB_TABLE[(((bb & (!bb + 1)) * DEBRUIJN_64) >> 58) as usize];
}

pub fn get_random_u64() -> u64{
    let mut rng = rand::thread_rng();

    let a1 = rng.gen::<u32>() & 0xFFFF; let a2 = rng.gen::<u32>() & 0xFFFF;
    let a3 = rng.gen::<u32>() & 0xFFFF; let a4 = rng.gen::<u32>() & 0xFFFF;

    return a1 as u64 | (a2 as u64) << 16 | (a3 as u64) << 32 | (a4 as u64) << 48;
}

pub fn random() -> u64{
    return get_random_u64() & get_random_u64() & get_random_u64();
}

pub fn get_pawn_move(sq: i32, white_to_move: bool) -> u64{
    let mut bb: u64 = 0;

    if white_to_move {
        if !getbit(A_FILE, sq-7) { bb = setbit(bb, sq-7); }
        if !getbit(H_FILE, sq-9) { bb = setbit(bb, sq-9); }
    }else {
        if !getbit(H_FILE, sq+7) { bb = setbit(bb, sq+7); }
        if !getbit(A_FILE, sq+9) { bb = setbit(bb, sq+9); }
    }

    return bb;
}

pub fn get_king_move(sq: i32) -> u64 {
    let mut bb: u64 = 0;
    if !getbit(H_FILE, sq-1) { bb = setbit(bb, sq-1); }
    if !getbit(H_FILE, sq+7) { bb = setbit(bb, sq+7); }
    if !getbit(H_FILE, sq-9) { bb = setbit(bb, sq-9); }

    bb = setbit(bb, sq+8);
    bb = setbit(bb, sq-8);

    if !getbit(A_FILE, sq-7) { bb = setbit(bb, sq-7); }
    if !getbit(A_FILE, sq+1) { bb = setbit(bb, sq+1); }
    if !getbit(A_FILE, sq+9) { bb = setbit(bb, sq+9); }
   

    return bb;
}

pub fn get_knight_move(sq: i32) -> u64 {
    let mut bb: u64 = 0;
    if !getbit(AB_FILE, sq-6) { bb = setbit(bb, sq-6); }
    if !getbit(GH_FILE, sq-10) { bb = setbit(bb, sq-10); }
    if !getbit(GH_FILE, sq+6) { bb = setbit(bb, sq+6); }
    if !getbit(AB_FILE, sq+10) { bb = setbit(bb, sq+10); }

    if !getbit(A_FILE, sq-15) { bb = setbit(bb, sq-15); }
    if !getbit(H_FILE, sq-17) { bb = setbit(bb, sq-17); }
    if !getbit(H_FILE, sq+15) { bb = setbit(bb, sq+15); }
    if !getbit(A_FILE, sq+17) { bb = setbit(bb, sq+17); }
   
    return bb;
}

pub fn get_bishop_mask(sq: i32) -> u64 {
    let mut bb: u64 = 0; 

    let r: i32 = sq / 8;
    let f: i32 = sq % 8;

    for (rank, file) in (r+1..7).zip(f+1..7) {
        bb = setbit(bb, rank*8+file);
    }
    for (rank, file) in (1..r).rev().zip(f+1..7) {
        bb = setbit(bb, rank*8+file);
    }
    for (rank, file) in (r+1..7).zip((1..f).rev()) {
        bb = setbit(bb, rank*8+file);
    }
    for (rank, file) in (1..r).rev().zip((1..f).rev()) {
        bb = setbit(bb, rank*8+file);
    }

    return bb;
}

pub fn get_bishop_all(sq: i32, occ: u64) -> u64 {
    let mut bb: u64 = 0; 

    let r: i32 = sq / 8;
    let f: i32 = sq % 8;

    for (rank, file) in (r+1..8).zip(f+1..8) {
        bb = setbit(bb, rank*8+file);
        if getbit(occ, rank*8+file) {
            break;
        }
    }
    for (rank, file) in (0..r).rev().zip(f+1..8) {
        bb = setbit(bb, rank*8+file);
        if getbit(occ, rank*8+file) {
            break;
        }
    }
    for (rank, file) in (r+1..8).zip((0..f).rev()) {
        bb = setbit(bb, rank*8+file);
        if getbit(occ, rank*8+file) {
            break;
        }
    }
    for (rank, file) in (0..r).rev().zip((0..f).rev()) {
        bb = setbit(bb, rank*8+file);
        if getbit(occ, rank*8+file) {
            break;
        }
    }

    return bb;
}

pub fn get_rook_mask(sq: i32) -> u64 {
    let mut bb: u64 = 0; 

    let r: i32 = sq / 8;
    let f: i32 = sq % 8;

    for rank in r+1..7 {
        bb = setbit(bb, rank*8+f);
    }
    for rank in (1..r).rev() {
        bb = setbit(bb, rank*8+f);
    }
    for file in f+1..7 {
        bb = setbit(bb, r*8+file);
    }
    for file in (1..f).rev() {
        bb = setbit(bb, r*8+file);
    }

    return bb;
}

pub fn get_rook_all(sq: i32, occ: u64) -> u64 {
    let mut bb: u64 = 0; 

    let r: i32 = sq / 8;
    let f: i32 = sq % 8;

    for rank in r+1..8 {
        bb = setbit(bb, rank*8+f);
        if getbit(occ, rank*8+f) {
            break;
        }
    }
    for rank in (0..r).rev() {
        bb = setbit(bb, rank*8+f);
        if getbit(occ, rank*8+f) {
            break;
        }
    }
    for file in f+1..8 {
        bb = setbit(bb, r*8+file);
        if getbit(occ, r*8+file) {
            break;
        }
    }
    for file in (0..f).rev() {
        bb = setbit(bb, r*8+file);
        if getbit(occ, r*8+file) {
            break;
        }
    }

    return bb;
}

pub fn set_occ(index: u64, bits: u64, mut m: u64) -> u64 {
    let mut result: u64 = 0;
    for i in 0..bits {
        let j = getls1b(m);
        m = clearbit(m, j as i32);
        if (index & (1u64 << i)) != 0 {
            result |= 1u64 << j; 
        }
    }
    return result;
}

fn transform(bb: u64, magic: u64, bits: i32) -> u32 {
    ((bb * magic) >> (64 - bits) as u64) as u32
}


pub fn find_magic(sq: i32, m: i32, bishop: bool) -> u64 {
    let mut b = [0u64; 4096];
    let mut a = [0u64; 4096];
    let mut used = [0u64; 4096];
    let mut magic: u64;
    let mut mask: u64;
    let mut fail: bool = false;
    unsafe {
        mask = if bishop {BISHOP_MASKS[sq as usize]} else {ROOK_MASKS[sq as usize]};
    }

    let n = mask.count_ones() as usize;

    for i in 0..(1 << n) {
        b[i] = set_occ(i as u64, n as u64, mask);
        a[i] = if bishop {get_bishop_all(sq, b[i])} else {get_rook_all(sq, b[i])};
    }

    for k in 0..10000000 {
        magic = random();
        if ((mask * magic) & 0xFF00000000000000).count_ones() < 6 {
            continue;
        }

        fail = false;
        
        for i in 0..4096 {
            unsafe {
                used[i] = 0;
            }
        }

        unsafe {
            for i in 0..(1 << n) {
                let j: usize = transform(b[i], magic, m) as usize;
                if used[j] == 0 {
                    used[j] = a[i];
                }else if used[j] != a[i] {
                    fail = true;
                    break;
                }
            }
        }
        if fail == false {
            return magic;
        }
    }
    println!("magic error");
    return 0;
}

pub fn init() {
    for sq in 0..64 {
        unsafe {
            BISHOP_MASKS[sq] = get_bishop_mask(sq as i32);
            ROOK_MASKS[sq] = get_rook_mask(sq as i32);
            BISHOP_MAGIC[sq] = find_magic(sq as i32, bbits[sq], true);
            ROOK_MAGIC[sq] = find_magic(sq as i32, rbits[sq], false);
        }
    }
    for sq in 0..64 {
        unsafe {
            println!("{},", BISHOP_MAGIC[sq]);
        }
    }
    println!("-------------------------------------------");
    for sq in 0..64 {
        unsafe {
            println!("{},", ROOK_MAGIC[sq]);
        }
    }
}

fn main() {
    init();
    printbb(get_bishop_mask(44));
}
