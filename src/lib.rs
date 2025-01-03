use std::collections::HashMap;

const ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const SAFE_ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '-', '_',
];
const PAD: char = '=';

fn encode(bytes: &[u8]) -> String {
    if bytes.len() == 0 {
        return "".into();
    }

    let size = ((bytes.len() + 2) / 3) * 4;
    let mut enc = String::with_capacity(size);
    let mut bytes_iter = bytes.chunks_exact(3);

    while let Some(b) = bytes_iter.next() {
        let i1 = (b[0] >> 2) as usize;
        let i2 = (((b[0] & 0b11) << 4) | (b[1] >> 4)) as usize;
        let i3 = (((b[1] & 0b1111) << 2) | (b[2] >> 6) & 3) as usize;
        let i4 = (b[2] & 0b11_1111) as usize;

        let val = [ALPHABET[i1], ALPHABET[i2], ALPHABET[i3], ALPHABET[i4]]
            .iter()
            .collect::<String>();
        enc.push_str(val.as_str());
    }

    let rem = bytes_iter.remainder();
    match rem.len() {
        0 => {}
        1 => {
            let i1 = (rem[0] >> 2) as usize;
            let i2 = ((rem[0] & 0b11) << 4) as usize;

            let vals = [ALPHABET[i1], ALPHABET[i2], PAD, PAD]
                .iter()
                .collect::<String>();
            enc.push_str(vals.as_str());
        }
        2 => {
            let i1 = (rem[0] >> 2) as usize;
            let i2 = (((rem[0] & 0b11) << 4) | (rem[1] >> 4)) as usize;
            let i3 = ((rem[1] & 0b1111) << 2) as usize;

            let vals = [ALPHABET[i1], ALPHABET[i2], ALPHABET[i3], PAD]
                .iter()
                .collect::<String>();
            enc.push_str(vals.as_str());
        }
        _ => unreachable!(),
    }
    enc
}

fn decode(mut b64: String) -> Vec<u8> {
    let mut data = Vec::new();
    if b64.len() == 0 {
        return data;
    }

    let lookup = build_decode_hashmap();
    b64.retain(|c| c != PAD);
    let mut b64_chars = b64.as_bytes().chunks_exact(4);

    while let Some(c) = b64_chars.next() {
        let [c0, c1, c2, c3]: [u8; 4] = c.try_into().unwrap();

        let v0 = lookup.get(&(c0 as char)).unwrap();
        let v1 = lookup.get(&(c1 as char)).unwrap();
        let v2 = lookup.get(&(c2 as char)).unwrap();
        let v3 = lookup.get(&(c3 as char)).unwrap();

        let b1 = (v0 << 2) | (v1 >> 4);
        let b2 = (v1 << 4) | ((v2 & 0b0011_1100) >> 2);
        let b3 = (v2 << 6) | v3;

        data.extend_from_slice(&[b1, b2, b3])
    }

    let rem = b64_chars.remainder();
    match rem.len() {
        0 => {}
        2 => {
            let c1 = rem[0] as char;
            let c2 = rem[1] as char;

            let v1 = lookup.get(&c1).unwrap();
            let v2 = lookup.get(&c2).unwrap();

            let b1 = (v1 << 2) | (v2 >> 4);

            data.push(b1);
        }
        3 => {
            let c1 = rem[0] as char;
            let c2 = rem[1] as char;
            let c3 = rem[2] as char;

            let v1 = lookup.get(&c1).unwrap();
            let v2 = lookup.get(&c2).unwrap();
            let v3 = lookup.get(&c3).unwrap();

            let b1 = (v1 << 2) | (v2 >> 4);
            let b2 = (v2 << 4) | ((v3 & 0b0011_1100) >> 2);

            data.extend_from_slice(&[b1, b2]);
        }
        _ => unreachable!(),
    }

    data
}

fn build_decode_hashmap() -> HashMap<char, u8> {
    let mut m = HashMap::new();
    for (idx, val) in ALPHABET.iter().enumerate() {
        m.insert(*val, idx as u8);
    }
    // Add the two url-safe characters
    m.insert('-', 62u8);
    m.insert('_', 63u8);
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 4648 test vectors
    #[test]
    fn encode_empty() {
        let input = "".as_bytes();
        let expected = "";
        let actual = encode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn encode_f() {
        let input = "f".as_bytes();
        let expected = "Zg==";
        let actual = encode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn encode_fo() {
        let input = "fo".as_bytes();
        let expected = "Zm8=";
        let actual = encode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn encode_foo() {
        let input = "foo".as_bytes();
        let expected = "Zm9v";
        let actual = encode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn encode_foob() {
        let input = "foob".as_bytes();
        let expected = "Zm9vYg==";
        let actual = encode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn encode_fooba() {
        let input = "fooba".as_bytes();
        let expected = "Zm9vYmE=";
        let actual = encode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn encode_foobar() {
        let input = "foobar".as_bytes();
        let expected = "Zm9vYmFy";
        let actual = encode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn decode_empty() {
        let input = "".into();
        let expected = &[];
        let actual = decode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn decode_f() {
        let input = "Zg==".into();
        let expected = "f".as_bytes();
        let actual = decode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn decode_fo() {
        let input = "Zm8=".into();
        let expected = "fo".as_bytes();
        let actual = decode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn decode_foo() {
        let input = "Zm9v".into();
        let expected = "foo".as_bytes();
        let actual = decode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn decode_foob() {
        let input = "Zm9vYg==".into();
        let expected = "foob".as_bytes();
        let actual = decode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn decode_fooba() {
        let input = "Zm9vYmE=".into();
        let expected = "fooba".as_bytes();
        let actual = decode(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn decode_foobar() {
        let input = "Zm9vYmFy".into();
        let expected = "foobar".as_bytes();
        let actual = decode(input);
        assert_eq!(actual, expected)
    }
}
