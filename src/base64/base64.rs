#[allow(non_upper_case_globals)]
const enc_map: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '+', '/'
];

#[allow(non_upper_case_globals)]
const dec_map: [u8; 128] = [
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127,  62, 127, 127, 127,  63,  52,  53,
    54,  55,  56,  57,  58,  59,  60,  61, 127, 127,
    127,  64, 127, 127, 127,   0,   1,   2,   3,   4,
    5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
    15,  16,  17,  18,  19,  20,  21,  22,  23,  24,
    25, 127, 127, 127, 127, 127, 127,  26,  27,  28,
    29,  30,  31,  32,  33,  34,  35,  36,  37,  38,
    39,  40,  41,  42,  43,  44,  45,  46,  47,  48,
    49,  50,  51, 127, 127, 127, 127, 127
];

/*
 * Encode a buffer into base64 format
 */
fn encode(dst: &mut String, dlen: usize, olen: &mut usize
          , src: &String, slen: usize) -> i32  {
    let (mut i, mut n): (usize, usize);

    let (mut C1, mut C2, mut C3):(i32, i32, i32);

    let mut p: String;

    if slen == 0 {
        *olen = 0;
        return 0;
    }

    //n = slen / 3 + if slen % 3 != 0 {1} else {0};

    //n = n * 4;

    n = slen;

    p = *dst;
    for i in  0..n {
        C1 = src[0];
        C2 = src[1];
        C3 = src[2];
        src = src[3..];



    } 
    


    

}
