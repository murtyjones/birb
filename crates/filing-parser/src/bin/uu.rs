fn encode_uu_chunk(bytes: &[u8]) -> impl Iterator<Item = u8> {
    let combined: u32 = bytes.iter().enumerate().fold(0, |acc, (index, &val)| {
        acc + ((val as u32) << 8 * (2 - index))
    });

    (0..4).rev().map(move |val| {
        let val = (combined >> (6 * val)) & 63;
        (val + 32) as u8
    })
}

fn encode_uu(bytes: &[u8], name: Option<&str>) -> String {
    let mut output: Vec<u8> = Vec::new();

    for chunk in bytes.chunks(45) {
        output.push((chunk.len() + 32) as u8);
        for uc in chunk.chunks(3) {
            output.extend(encode_uu_chunk(uc));
        }
        output.push(10);
    }

    format!(
        "begin 644 {}.txt\n{}`\nend\n",
        name.unwrap_or("file"),
        String::from_utf8(output).expect("Couldn't unwrap encoding result!")
    )
}

fn decode_uu_chunk(bytes: &[u8]) -> impl Iterator<Item = u8> {
    let combined: u32 = bytes.iter().enumerate().fold(0, |acc, (index, &val)| {
        acc + (((val as u32) - 32) << 6 * (3 - index))
    });

    (0..3).rev().map(move |val| {
        let val = (combined >> (8 * val)) & 255;
        val as u8
    })
}

fn decode_uu(encoded: &str) -> Option<(String, String)> {
    let mut lines = encoded.lines();

    let name = lines
        .next()
        .expect("Couldn't unwrap lines!")
        .split(" ")
        .collect::<Vec<_>>()[2]
        .to_string(); //eugh

    let mut output: Vec<u8> = Vec::new();
    for line in lines {
        if let Some(chr) = line.chars().nth(0) {
            match chr {
                '`' => break,
                ' '...'_' => {
                    for dc in line[1..].as_bytes().chunks(4) {
                        output.extend(decode_uu_chunk(dc));
                    }
                }
                _ => {
                    return None;
                }
            }
        }
    }

    Some((
        String::from_utf8_lossy(output.as_slice()), //.expect("Could not unwrap decoding result!"),
        name,
    ))
}

fn main() {
    use std::io;
    use std::io::Write;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!(">> ");
        let _ = stdout.flush().unwrap();
        let mut input = String::new();
        let _ = stdin.read_line(&mut input).unwrap();
        println!("\nOutput:\n{}", encode_uu(input.trim().as_bytes(), None));
    }
}

mod test {
    use super::*;

    #[test]
    fn test_encode_decode_1() {
        let to_encode = "Cat".to_string();

        let encoded = encode_uu(&to_encode.into_bytes(), Some("Cat"));
        println!("Encoded:\n{}", encoded);

        let (decoded, _) = decode_uu(&encoded).unwrap();

        println!("Decoded:\n{}", decoded);
        assert_eq!(decoded, "Cat");
    }

    #[test]
    fn test_encode_decode_2() {
        let to_encode = "testing this is line 1
testing this is line 2"
            .to_string();

        let encoded = encode_uu(&to_encode.into_bytes(), Some("Cat"));
        let (decoded, _) = decode_uu(&encoded).unwrap();

        assert_eq!(
            decoded,
            "testing this is line 1
testing this is line 2"
        );
    }

    #[test]
    fn test_decode_image() {
        let encoded = r#"begin 644 image1.gif
M1TE&.#EAT +B /<         ,P  9@  F0  S   _P K   K,P K9@ KF0 K
MS  K_P!5  !5,P!59@!5F0!5S !5_P"   " ,P" 9@" F0" S " _P"J  "J
M,P"J9@"JF0"JS "J_P#5  #5,P#59@#5F0#5S #5_P#_  #_,P#_9@#_F0#_
MS #__S,  #, ,S, 9C, F3, S#, _S,K #,K,S,K9C,KF3,KS#,K_S-5 #-5
M,S-59C-5F3-5S#-5_S.  #. ,S. 9C. F3. S#. _S.J #.J,S.J9C.JF3.J
MS#.J_S/5 #/5,S/59C/5F3/5S#/5_S/_ #/_,S/_9C/_F3/_S#/__V8  &8
M,V8 9F8 F68 S&8 _V8K &8K,V8K9F8KF68KS&8K_V95 &95,V959F95F695
MS&95_V:  &: ,V: 9F: F6: S&: _V:J &:J,V:J9F:JF6:JS&:J_V;5 &;5
M,V;59F;5F6;5S&;5_V;_ &;_,V;_9F;_F6;_S&;__YD  )D ,YD 9ID F9D
MS)D _YDK )DK,YDK9IDKF9DKS)DK_YE5 )E5,YE59IE5F9E5S)E5_YF  )F
M,YF 9IF F9F S)F _YFJ )FJ,YFJ9IFJF9FJS)FJ_YG5 )G5,YG59IG5F9G5
MS)G5_YG_ )G_,YG_9IG_F9G_S)G__\P  ,P ,\P 9LP F<P S,P _\PK ,PK
M,\PK9LPKF<PKS,PK_\Q5 ,Q5,\Q59LQ5F<Q5S,Q5_\R  ,R ,\R 9LR F<R
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
45NQ8LF7-GD6;5NU:MFW=@@T( #L!
`
end
"#;
        let (decoded, _) = decode_uu(&encoded).unwrap();
        assert_eq!(decoded, "1");
    }
}
