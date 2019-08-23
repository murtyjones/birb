use std::io::prelude::*;

fn encode_uu_chunk(bytes: &[u8]) -> impl Iterator<Item = u8> {
    let combined: u32 = bytes.iter().enumerate().fold(0, |acc, (index, &val)| {
        acc + ((val as u32) << 8 * (2 - index))
    });

    (0..4).rev().map(move |val| {
        let val = (combined >> (6 * val)) & 63;
        (val + 32) as u8
    })
}

pub fn encode_uu(bytes: &[u8], name: Option<&str>) -> String {
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

pub fn decode_uu(encoded: &str) -> Option<(Vec<u8>, String)> {
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

    Some((output, name))
}

mod test {
    use super::*;

    #[test]
    fn test_encode_decode_1() {
        let to_encode = "Cat".to_string();

        let encoded = encode_uu(&to_encode.into_bytes(), Some("Cat"));
        println!("Encoded:\n{}", encoded);

        let (decoded, _) = decode_uu(&encoded).unwrap();

        assert_eq!(String::from_utf8(decoded).unwrap(), "Cat");
    }

    #[test]
    fn test_encode_decode_2() {
        let to_encode = "testing this is line 1
testing this is line 2"
            .to_string();

        let encoded = encode_uu(&to_encode.into_bytes(), Some("Cat"));
        let (decoded, _) = decode_uu(&encoded).unwrap();

        assert_eq!(
            String::from_utf8(decoded).unwrap(),
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
        assert_eq!(
            String::from_utf8_lossy(decoded.as_slice()).into_owned(),
            "GIF89a�\u{2}�\u{0}�\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}3\u{0}\u{0}f\u{0}\u{0}�\u{0}\u{0}�\u{0}\u{0}�\u{0}+\u{0}\u{0}+3\u{0}+f\u{0}+�\u{0}+�\u{0}+�\u{0}U\u{0}\u{0}U3\u{0}Uf\u{0}U�\u{0}U�\u{0}U�\u{0}�\u{0}\u{0}�3\u{0}�f\u{0}��\u{0}��\u{0}��\u{0}�\u{0}\u{0}�3\u{0}�f\u{0}��\u{0}��\u{0}��\u{0}�\u{0}\u{0}�3\u{0}�f\u{0}ՙ\u{0}��\u{0}��\u{0}�\u{0}\u{0}�3\u{0}�f\u{0}��\u{0}��\u{0}��3\u{0}\u{0}3\u{0}33\u{0}f3\u{0}�3\u{0}�3\u{0}�3+\u{0}3+33+f3+�3+�3+�3U\u{0}3U33Uf3U�3U�3U�3�\u{0}3�33�f3��3��3��3�\u{0}3�33�f3��3��3��3�\u{0}3�33�f3ՙ3��3��3�\u{0}3�33�f3��3��3��f\u{0}\u{0}f\u{0}3f\u{0}ff\u{0}�f\u{0}�f\u{0}�f+\u{0}f+3f+ff+�f+�f+�fU\u{0}fU3fUffU�fU�fU�f�\u{0}f�3f�ff��f��f��f�\u{0}f�3f�ff��f��f��f�\u{0}f�3f�ffՙf��f��f�\u{0}f�3f�ff��f��f���\u{0}\u{0}�\u{0}3�\u{0}f�\u{0}��\u{0}\u{319}\u{0}��+\u{0}�+3�+f�+��+\u{319}+��U\u{0}�U3�Uf�U��U\u{319}U���\u{0}��3��f�����\u{319}����\u{0}��3��f�����\u{319}����\u{0}��3��f�ՙ��\u{319}����\u{0}��3��f�����\u{319}���\u{0}\u{0}�\u{0}3�\u{0}f�\u{0}��\u{0}��\u{0}��+\u{0}�+3�+f�+��+��+��U\u{0}�U3�Uf�U��U��U�\u{300}\u{0}\u{300}3\u{300}f\u{300}�\u{300}�\u{300}�\u{32a}\u{0}\u{32a}3\u{32a}f\u{32a}�\u{32a}�\u{32a}���\u{0}��3��f�ՙ��������\u{0}��3��f����������\u{0}\u{0}�\u{0}3�\u{0}f�\u{0}��\u{0}��\u{0}��+\u{0}�+3�+f�+��+��+��U\u{0}�U3�Uf�U��U��U���\u{0}��3��f�����������\u{0}��3��f�����������\u{0}��3��f�ՙ��������\u{0}��3��f���������\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}!�\u{4}\u{1}\u{0}\u{0}�\u{0},\u{0}\u{0}\u{0}\u{0}�\u{2}�\u{0}\u{0}\u{8}�\u{0}\u{1}\u{8}\u{1c}H����\u{8}\u{13}*\\Ȱ�Ç\u{10}#J�H��ŋ\u{18}3j�ȱ�Ǐ C�\u{1c}I���}(S�\\ɲ�˗0U\u{2}�I��\u{35b}8s��ɳ�ϟ@�\n\u{1d}J��ѣH�*]ʴ)L�N{ΌJ��իX�\u{59c}���ׯ`Ê��ulʲfӪ]�\u{16}(ڶp�ʵ�v,T�u�����j\u{7be}�\u{3}\u{b}n�\u{17}�ݵ�\u{7}+^��e�Ɛ#\u{1b}n{X��\u{6}Q*������\t�\u{e}�L�sgӢ\t��\u{b}���ңY�.�z��Ҷ[㥫;5��\u{10}W�v�2sn��7\u{1f}��\u{1c}9��O>_κ gۄ+;��U�Y�X�_��\u{e}\\<���ǟ_?�:���Y��ܞ������Fߜ�~����|\u{4}ZVU���F\u{1f}~�\u{11}\u{8}\u{1c}��9��y� {�\u{15}� {\u{12}�\u{6}߂��a}�m�]oT�\u{17}��~�uao\u{16}�\u{7}��\n�fc���a�<�7\"h7�w��\u{1f}��#�9\u{1e}�[T�]\u{18}!�@�\u{8}a�\u{1a}��b��e\t�}Mv9��R��\u{1d}�[�9b�/��$e,��b�\u{15}�܌\u{1a}�\u{18}��D���k^�i��\u{7f}:He�A�Hf����\u{1a}�a�\u{19}�g{ڧ�\u{c}��\'���))��Fꞓ\u{12}j\u{7}�wБ�\u{14}��R�\"�!Z�%��2�z�\u{13}�j�}�ydl�ʗ%��F�%�-2����I*�F\u{1e}�+�d��#��ɚᖵ�z+���9\u{1d}�c��꒖A\u{6e4}���\'���B����v+k��\u{e}8ܕ�\u{1a}J\"�ߥ*����z&���Xp��bj���b�c��~\tq��nǦ��]��\u{7f}HR�n��\n,j��\"H��BB���.\u{1b}2�M����\u{3}\u{7}y\'�ڂ���\u{1b}�,(��|���B���&����L�\u{734}�KA��j\u{c}�G\u{1c}u�V�5�4c}�ȱJ\u{1b}�c��M6�՚=)b��]��Y��g�d�}��t�=��a�-j�s�MGu�:��p�\u{17}�+��\u{3}&u�G���SS~��g��[�w�yP�\u{17}�ejX�\u{7f}n��2����%�6�ǎS�\u{5ee}xZ�Of�\u{ef07}����.T�_\t\u{f}��q\u{19}���1�n ��k�|��\u{13}5�O�{u}��\u{1f}����g�1��\u{7}�}���<��?�[�3\\vh\u{7f}*\u{7}h��Ū��\u{11}�&4��q���g?�\u{1}PLx�\u{35c}�נ���8�\u{1b}���G�\t֏bEj������e\u{2}\u{b}��8�3\u{1}��[)렇t�4����0\u{4}��X��\u{1f}��Y�]\u{5}w��\"�Ѕ\u{3}���r�)�1ml��\u{19}\u{12}\r��^\u{5}+OCK��N��:��f5�k��/\u{1e}z��;J\u{1d}\u{10}[�81\r6J�\u{7}\\�>%F��+/�┻�fD8\u{2}�oS�\"þ\u{15}8\u{1c}�k|_\u{c}dx���1\"-Y�\"\u{18}�F�,\u{1}.k~6�Q\u{16}��� 0:\nCc� �4+V2�o�����CA��y��a�L�Ė���8C\u{16}\u{16}�E3�!\rX�j��\"\u{5b5}YvRe��d(\u{1d}�E����$\u{e}�����2@�\n�2�I�W\u{12}\u{355}V�W�\u{e}9D0�O�S\nؾB8Jܝof�Lg�6�4��&~[3�*�V���\u{6}�^����@z�3:������!x\u{e}�m��\'�\"�C\u{b}��C9\'D�\'���Utv\u{13}�(�4��r~�7\u{1}iG�\'ґ®�4�^WPjR건��{);K\tӚzԦ��)!i�Ӗ괧����\u{7}T�\n���;*.ŢT����Nm\u{c}T\u{f}g\u{328}�t�V\u{1d}�TUڝ�ή��êW\u{1d}WL��u+\u{10}l�XϺ��n��Oq�Z��>�\u{2}���!h�֊W�\u{615}�l]H]����f�$�M�b\u{17}���:�����d\'��������+���Y���}\u{15}�\\;��=�z��\u{b}\r+iKkY0vV�{]m�L�>�Rg����k?\n����}�uik�\u{17}ܛ\u{16}��Уm\u{17}��[�4\u{17}��j�i}���B�{\u{12}�.i�����N�Uenu�{<\u{1ad78}�\u{5},y�:\\�!7�\u{e}]o��{Y�W����n��[�淽+}�}�;_\u{0}wվ�%p��[[�&X����/q\u{11}\\A�B8�\u{6}N\u{14}�)h�\u{b}o����m�C�a�Iؽ\u{1b}\u{1e}l�Ww�\u{0}�x�+6q�\u{7}�b��X� \u{e}��\u{7}|�Ϲ��@�\u{1f}��Ldў��H> j���&;\u{19}�L}���,� o��I�l��kG�\u{1e}\u{18}�\\��9GLcRζ��#�S�l,6c��iFs��lb:?��\u{16}���,���Y)J�\u{1c}?�����1�p\u{1e}\u{1e}Bq�_��F\tf���G\u{6}\n�ї�r�\u{6}]iJRQy�\u{16}t��f��-5�[��=\u{15}IG_Ʊb��s�F=�Rs��S���i�G[�,��6�3[�4C^\u{f}�a�J\u{1c},]X�(���\u{613}��\u{b}�LW91�K��\u{1b}�X�m�/Ԣ�\u{12};M�h@��\'�~\u{5ab}c���J;\u{711}\u{1c}��1�i?k{�\u{7f}����m�d��o�\u{6}!m��j=�:�kT��1��>\u{35f}Ĕ�p\u{10}g�{���lb���noF\\�>�x�\u{15}�j�R��\u{13}S��w\u{1d}ob���\u{18}���[.\u{1b}�-�T\u{1c}���\u{19}�o~�|�*/#\u{1c}�%��\u{7}}���Wq\u{1a}\tlQ\u{16}\u{1c}��V:CA��`\u{b}�f�}���\rtUw�~7t��\u{8}�ue�<��559s-s����\u{19}\';�یHK���9G����i��{գ��W�s\u{2}�\r��:��7m��\"��y��E���T4��1�t�&�9\u{32b}��\u{14}=��._:�\'w�.�<{�^��m��o��U�bї������ho:З���3=�t/=���\\���a?|�ϾM��y���\"d\u{5ca}�:Y;\u{319}_v����\u{18}}�ES���k\u{15}j�g(H����gw�oμ���}�;|�<����\u{1f}�s\u{19}w���q��~\u{13}�R�l\r�R�w�N\u{17}R�gv�ǀ�C~2�R��\u{18}\u{18}�l�\"I\u{6}\u{13}E�qG��,�TE�f�\t�\u{1d}\u{1c}H8$�pbC\"�I��D�\u{12}W�GL\r�%\u{1a}S\u{1e}նKBÃ��q�\u{7}9��?>�&\u{12}�%HB+\u{6}�+{S\u{1f}��V��t�\u{14}-\'$#�\u{12}A�1j=C\u{1e}Z\u{18}vQhf��k02J�rC/\u{18}4\u{1c}��Yt�Sw��!|�s�X��.Æi�*s(nY�\u{7f}_�m\u{1e}5IT8p�(��7�gX�\\���Xh\u{8}�~tx�\u{f}ↄ8�\u{13}��\u{1d}��!%Aa����%w8�9S��H5�$D�\u{7}m\u{1a}ȅ\u{1e}�x�d�I�5Jb5�\u{14}~PG\u{7f}�\u{8}�#ȉ�X�\u{1}� �Ȇ�\u{8}V�L�؉�xMs��K�J\u{2}�O�~\u{1b}����3�(*t���h/�\u{8}�{8S����\u{18}�ĸ����]Xs|(��%�؈��(�\'�����\u{488}�\u{e202}����,�7��}\u{b}�7��?af}�S#Mx/�g)(\u{8}A\u{0}��\u{2}�sp�}\u{3}�A\n%�\r�VDSwG�������@�đ�Q�8X�\u{7}�k��O\"d����e\u{18}R�\"\u{16}\u{6}��\'{R1��1�\u{1d}�~kE��Ɠ�A��g�3�U؇\u{19}G�{��;B\u{19}|��eAh=K�;MɔO9|I�cS�_\u{f}�D�{Y�9>�|f�g�\u{18}gQ�s�\u{17}g58�g�g���6�}���s)����\u{16}\'���bɷ�\u{7f})w4hu\u{e}�8y�=.�u��r��>]��j9<�����}�Yo\u{1a}H��w�\u{1a}���نR�V��I��)�\u{1f}��.\t�\u{12}X�\u{557}=���E\u{17}N�\u{603}�ys�ه��\u{7f}14�\u{1}�hJ\u{18}7�Ɓ��y]�?,x�\u{b}4o\'�A�ف>�Ri�8�bm\u{1}���4�{T�$���H������������2ҍ��4�h\u{17}�w��I�a\u{14}��x�r�qm��V8���H�(��Ɏ˘��\u{19}I�)��韉��z��\u{1}W�!��6�(�\u{12}�1\u{1e}�A0dB�\u{17}OUG���zs�\'\u{15}J�\u{1f}Ċ�ɠ�(3BɎ�x��R1\u{14}:PX�\u{e}��x���\u{8}7/\u{1a}�ࢡ�\u{2}\"@c����i�\u{1c}�)Ez�Ӹ�zY�vX�ki�#*��dx�刢\u{f}��WJ��Ĥ�\u{16}��襶\u{618}5��\tJ�F��HʢV��(�c\nz\u{7}�5\u{2}r��\u{59f}\u{4}�\u{8}��\u{11}١�I�\u{16}I8\u{18}\n�~�W!\u{19}��הƉ����FÝ\u{5}y�Ɩ~�)\u{19}1(�\"d����\u{13}��\u{8}Su^Ý\n\u{17}��g��5��E��W�}���ז^��ϧe\u{1e}\n��j���\u{1a}|��{�:_��;�\n\u{19}��\u{7f}Ú9cɖe�yW\t�����I�|�~\u{8}��ꪺz�T��\u{14}�w�z8ק�e9�\u{8}s��\u{1a}�)E{����ـc�U�*:�\nv�8~\rǮ�8���F�\u{17}��j��8��Y��J�\u{5ee}_���I�\u{1}�}\u{6}K�te�m��O������(F5\u{17}��\u{8}��)�\u{14}\u{b}�\u{1d}k�w������y\u{19}����\u{16}�\u{1}k�\u{3}[��y��Xh:8����M\u{7}�\u{5aa}.9���\u{19}*�A�(ӂ\u{18}������y��\u{2}��Z��:@\u{2}Z�\u{10}���,�\u{14}o3� �لVkG\u{5}��k\u{8}���ȃ7\u{b}\u{7f}�����oJ�l�#a\'n��XۯMg���B�&�)�\u{18}�\u{1c}�Ӛ�yK��\t��9��Y��H�gC�`k�S\u{17}�⸱VC��I:��+|b���pz�a�hĉ�$ꆳq���� ڟ�8�t\u{1b}U�f�[\u{1a}�\"\u{18}�ss��\u{b}�\"��^���˲�k�\u{7f}+�n�O\u{1a}�o۹�8t�;���~����h��˻\u{11}���+��������K��+�ݘ��\u{b}����&��,y�6�6�@�\u{b}�#+��������\u{19}#��P8��j����)y���7b#��Ԉ��\'�4�����w�E8��\n�U��i\u{5ab} k^��8�\u{1a}�\u{1f}\u{c}��屮WV\u{1d}�T�!\u{1c}�ų�����*�PY��;{$�])\u{314}\',�\u{1a}\u{1c}a�`2��.��ϊ�0��*K�;��=|R?,;Cl>Q|�S��;k�[�VW\'��C;�*�I줠\u{19}�_���4\\�:K�jln�(~\t��.\t�A\u{c}�\u{e}\u{6ea}�j9�\n�M��-ےɚ�h�vq\u{1c}�\u{b}l�t��\u{18}x|g���\u{11}�D���\u{659}���!���Yl�5���ۑ�\\���BO觅�E�|���.K�h3\u{1a}��\u{2}�K�¬\u{7}�k�\"S��\u{7}�o�\u{6e0}���U�ˇ˸{��r�E����[�|A<\u{ea7c}�ȟbؤ仱����6g[\u{6}�����;p\u{14}��\n�`åm�}���\\�il:�\u{1b}څè\u{322}��L\n�j8�!*\u{36c}����V�\\��Klì��ˍ賓e��gZ\u{380}���;�м�\u{15}�ˋ�I�\u{3}��r���˲\u{10}m��K�����Q���괜|�\u{9b}��L����\\|��,�\u{15}l�\ty�+��\u{4}l�\u{15}\u{1d}@\u{f}���t�}\u{7}��\u{14}J\u{3}���KRM�f7\u{c}ĥ�����8L�J��P�ûg��,ƌ\u{c}�n�|a\u{c}��,�G�bO\u{1c}aT\u{355}WM�h�zRM�]}�g�������s��om��s�[��z�־�����I��X�������\u{17}�8x8-�h\tҘ�hx�\u{c}��-]��\u{15}�\u{59c}��|V_-U�\u{1d}<a��\u{6db}�\n\\�+�V7[�.��e�Հ��-|�E�KU��!(�%��P��϶�C\u{1b}O�\u{c}�zj���ȫ�RM\u{11}X\u{5eb}\n�qM���\tm��h�[Z��쀩}�m\u{1b}��l��l�㻃�)�c\u{15}ڋ��J��\u{0}\u{1d}�>���l����`K}�\u{1d}��(�\u{1e}=���8�2J\u{1a}h�}�n�\u{733}m�bz�\u{1b4d}�[�����]2���+z�\u{1b}\r\u{7eb}��Xͽg6���\u{1b}T,�7�7\u{4}��\u{6}l��=�\n�D�j�v�o\u{2}�@*I�/$��\rW�\u{18}��=�P\u{355}k\r�@�W�]�F��^��\u{1}4��3�\u{7f}m�yMRc}RL\u{e}�9\u{1e}��]gH^{~M�\u{1e}��+\u{2}�Y�yS��]~�{\u{16}�8��*��+�\u{1fdc}�j��e~�1\u{15}��\u{b}�n\u{17}�\u{1c}��EN�G��}�,\u{730}A�UU�����t�{v�ᑹƎ~�{N�}^�\\F�J؛�͵H\u{8}�)\u{8}�r��8\u{1e}�;��~~�7l����\u{9c}�\u{13}]�4猝}ޤ��Z��>��\n���\u{2}L����[�\u{e}Jю\r�X�dor�D��\u{11}ݥ[8�M���\u{18}��ν�ne=��s���\n�\u{17}yP\u{6}\\���?\u{f}���Ae���W�d�z��c��b)Q�^��^��~�\u{19}|����>���~�����\u{1c}��J�:[\u{6}/��N`��\t��\u{b}�_�μ�-�\u{f}/�\u{11}�_U���u�������^f\u{1e}\u{f}�%�y�\u{16}��ʿ�l�\'��/_�\u{16}ӷR��n\u{12}��\r�����K��Σ\u{14}���\'��\u{e}�\u{17}��\u{19}]�N�}��L\u{7f}v��Ӟ\u{18}�>��\u{13}����\u{0}��Z��:O^N��]}��_�]a\u{f}\\c��i\u{f}�\u{8}���U��u����oO��J�v��y��m�����\u{7f}/�o郯�\\_�ύ�d���\u{7f}�������H)��?��\u{1d}p\u{f}]r�<���\u{f60c}��\u{7}������ا�����P���{��/���59���o��o���s�\u{1f}ҫ���\u{6}��\u{1f}{��z����\u{7f}�:�����\u{30f}�Ϗ��\u{1f}�\u{7}K��\u{1f}��\u{f}��oyؿ��������_\u{7f}�\u{1f}��_�\u{11}��������\u{1f}k���G,��/���������|��\u{0}�O�@��\u{0}\u{0}(�P�B�\r\u{1d}>�\u{18}QbB�\u{13}-^ĘQ�F�\u{1d}=~\u{4}\u{19}��A�%M�D�R�J�\u{e}\u{f}��\u{19}S�L�5m�ęS�Ζ=}�\u{4}\u{1a}T5�ȊC�\u{1e}E�T�ƝM�>�\u{1a}U�R�U�^�(U�V�]�~\u{5}\u{1b}V�X�e\u{35e}E�V�Z�m\u{742}\r\u{8}\u{0};\u{1}"
        );
    }

    #[test]
    fn test_another_image() {
        let contents = r##"begin 644 aumpiechartscombinded5217v4.jpg
M_]C_X  02D9)1@ ! 0$ > !X  #_X1$&17AI9@  34T *@    @ ! $[  (
M   4   (2H=I  0    !   (7IR=  $    H   0UNH<  <   @,    /@
M   <Z@    @
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M                                                 $QI;F1S87D@
M4W=I871K;W=S:VD   60 P "    %   $*R0!  "    %   $,"2D0 "
M S$S  "2D@ "     S$S  #J'  '   (#   "*      '.H    (
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M
M                               R,#$W.C U.C R(#$W.C$V.C0Q #(P
M,3<Z,#4Z,#(@,3<Z,38Z-#$   !, &D ;@!D ', 80!Y "  4P!W &D 80!T
M &L ;P!W ', :P!I    _^$+)FAT=' Z+R]N<RYA9&]B92YC;VTO>&%P+S$N
M,"\ /#]X<&%C:V5T(&)E9VEN/2?ON[\G(&ED/2=7-4TP37!#96AI2'IR95-Z
M3E1C>FMC.60G/SX-"CQX.GAM<&UE=&$@>&UL;G,Z>#TB861O8F4Z;G,Z;65T
M82\B/CQR9&8Z4D1&('AM;&YS.G)D9CTB:'1T<#HO+W=W=RYW,RYO<F<O,3DY
M.2\P,B\R,BUR9&8M<WEN=&%X+6YS(R(^/')D9CI$97-C<FEP=&EO;B!R9&8Z
M86)O=70](G5U:60Z9F%F-6)D9#4M8F$S9"TQ,61A+6%D,S$M9#,S9#<U,3@R
M9C%B(B!X;6QN<SID8STB:'1T<#HO+W!U<FPN;W)G+V1C+V5L96UE;G1S+S$N
M,2\B+SX\<F1F.D1E<V-R:7!T:6]N(')D9CIA8F]U=#TB=75I9#IF868U8F1D
M-2UB83-D+3$Q9&$M860S,2UD,S-D-S4Q.#)F,6(B('AM;&YS.GAM<#TB:'1T
M<#HO+VYS+F%D;V)E+F-O;2]X87 O,2XP+R(^/'AM<#I#<F5A=&5$871E/C(P
M,3<M,#4M,#)4,3<Z,38Z-#$N,3(Y/"]X;7 Z0W)E871E1&%T93X\+W)D9CI$
M97-C<FEP=&EO;CX\<F1F.D1E<V-R:7!T:6]N(')D9CIA8F]U=#TB=75I9#IF
M868U8F1D-2UB83-D+3$Q9&$M860S,2UD,S-D-S4Q.#)F,6(B('AM;&YS.F1C
M/2)H='1P.B\O<'5R;"YO<F<O9&,O96QE;65N=',O,2XQ+R(^/&1C.F-R96%T
M;W(^/')D9CI397$@>&UL;G,Z<F1F/2)H='1P.B\O=W=W+G<S+F]R9R\Q.3DY
M+S R+S(R+7)D9BUS>6YT87@M;G,C(CX\<F1F.FQI/DQI;F1S87D@4W=I871K
M;W=S:VD\+W)D9CIL:3X\+W)D9CI397$^#0H)"0D\+V1C.F-R96%T;W(^/"]R
M9&8Z1&5S8W)I<'1I;VX^/"]R9&8Z4D1&/CPO>#IX;7!M971A/@T*(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @( H@(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @"B @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" *(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @( H@(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @"B @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" *(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @( H@(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @"B @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" *(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @( H@(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @"B @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" *(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M( H@(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @"B @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" *(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @( H@(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M"B @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" *(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @( H@(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @
M(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @(" @"B @(" @(" @(" @
M(" @(" @(" @(" @(" @(" \/WAP86-K970@96YD/2=W)S\^_]L 0P '!04&
M!00'!@4&" <'" H1"PH)"0H5#Q ,$1@5&AD8%1@7&QXG(1L=)1T7&"(N(B4H
M*2LL*QH@+S,O*C(G*BLJ_]L 0P$'" @*"0H4"PL4*AP8'"HJ*BHJ*BHJ*BHJ
M*BHJ*BHJ*BHJ*BHJ*BHJ*BHJ*BHJ*BHJ*BHJ*BHJ*BHJ*BHJ*BHJ_\  $0@!
MXP:S P$B  (1 0,1 ?_$ !\   $% 0$! 0$!           ! @,$!08'" D*
M"__$ +40  (! P,"! ,%!00$   !?0$" P $$042(3%!!A-180<B<10R@9&A
M""-"L<$54M'P)#-B<H()"A87&!D:)28G*"DJ-#4V-S@Y.D-$149'2$E*4U15
M5E=865IC9&5F9VAI:G-T=79W>'EZ@X2%AH>(B8J2DY25EI>8F9JBHZ2EIJ>H
MJ:JRL[2UMK>XN;K"P\3%QL?(R<K2T]35UM?8V=KAXN/DY>;GZ.GJ\?+S]/7V
M]_CY^O_$ !\!  ,! 0$! 0$! 0$        ! @,$!08'" D*"__$ +41  (!
M @0$ P0'!00$  $"=P ! @,1! 4A,08205$'87$3(C*!"!1"D:&QP0DC,U+P
M%6)RT0H6)#3A)?$7&!D:)B<H*2HU-C<X.3I#1$5&1TA)2E-455976%E:8V1E
M9F=H:6IS='5V=WAY>H*#A(6&AXB)BI*3E)66EYB9FJ*CI*6FIZBIJK*SM+6V
MM[BYNL+#Q,7&Q\C)RM+3U-76U]C9VN+CY.7FY^CIZO+S]/7V]_CY^O_:  P#
M 0 "$0,1 #\ ^D:*** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHS4$E[:Q?ZVXB3_><"@">BJ7]L:9_T$+7_ +_K_C2'
M6M+!(.I6@(_Z;K_C0*Z+U%5H]1LIO]3>02=_EE4_UJQD'H:!BT444 %%%% !
M3)YX[:WDGG;;'$A=VQG  R3Q3Z9/!'<V\D$Z[HY4*.N<9!&".*3O;0:M?4\(
M\>_%2_UO9%X9:]T_3X92#=H[1O.V.F5Z#J<9R>"<=*]HDOWL?"IOVCDN'AL_
M.V*"S2$)G'KDUY?\;-/M-+\-:)9Z=;1VUO'/(%CB4*!\H_7W[UZG;W=O8Z!;
MW%[/%;P1P(7EF<(J\#J3P*%\,DN_Z"?Q1OV_4\.2X^(.OZ'>^,8M;EM[2W8O
MY$=RT8(7KMC'RD#_ &N3@]:[O3/'>HW7P;N_$,JK_:-LC0^9M 5W!"A\=/X@
M2.F0>U3^._#D_CG1EN]!\0H;.&)S]FA;?#=,#GYF5L<8QT.#5?X=ZCI_C7X?
M76B3Z=%910*;:6*V)"[6R0RELG.<]2>1GO1JXR2_KS#:46^_](PM)U/6M UC
MP?=SZW?ZC'XA3%U;W4Q=%+$8* _=QN'3T]#BNF\1:C>Z[\1=-\,:3>7%K!9C
M[9J4MO*T;%>,1Y!!YR/^^AZ527P1%X2CAU[7]<EU.T\/P.UC;&!8O+[@%LG<
M<X Z<X[#%9OAO6Y/#6G1:K=6+ZGXD\67!EAME<1_N\_*"QSM7G(^H[#-4FKK
MR?Y[+]16=OZ^;/7**YSPCXOC\4QWL4MF]AJ&GS&&ZM7</L.2!AAC(X/8<@_4
M]'2&%%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !11
M10 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%%
M !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444
M%%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4
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
M **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
MHHHH **** "BBB@ HHJM?:C::;#YM[.D*=BQY/T'4_A0&Q9HS7#ZE\1$4,FE
M6Q=L\2S<#_OD<_J*Y+4=>U+5&/VR[D9/^>:G:@_ <?C4.:1A*O%;:GI]_P")
M](TX'S[R-G'\$9WM^G3\:YV[^(\8R+&Q9O1IGQ^@S_.N!HJ'-F$J\WL=%=>.
M=:N2?+FCMU](HQ_,Y-9<VM:G< B;4+EP>QE./RJC14W9DYR>[%)).3R:2BBD
M0%4YSFXD/JY_G5RJ4O\ K7_WC6E,ZL/NQE%%%:G87+?5M1M&#6U]<1$?W92*
MV+3QYK]KC==K.H_AFC!_48/ZUS=% 'H5C\42 !J6GY]7@?\ ]E/^-=)8>-]"
MU!E5;P0.?X9QL_7I^M>,T4 ?0B2+(H9&#*1D$'(-.KP?3]9U'2GW:?=R0^J@
MY4_@>*['2/B9*KK'K-LK+T\Z 8(^JGK^'Y4 >CUXY\6_^2B^&/JG_HT5ZGIF
MN:=J\>[3[J.;'5>C#Z@\UH4+22?8/LM=SE?'G@MO&VF6UF-1-BL$WFD^3Y@?
M@C&-P]:\M^(_@W5O#?AVUGU#Q5>:Q UP(DMYPX5#M8AAEV'08Z=Z]\HI6[#O
MW.2^&^CWFD^$[=KW5I]1%W%'-$LN?]'4H,1KECP/;'TK(\>2/XG\6:3X*MV(
M@=A>:BRGI&O(4^F<?F5KT2JT>G64-]+>PV=O'=S ++<+$HD<>A;&3T'7TJF[
MRO\ UY$I6C8\TU62VT;XY6MSJ[PVFG?V:4MY)B%C4!2-H)X'?CW]ZN_!&*=/
M!ET\@802WKM!GH1M4$CVR#^1KO;_ $NPU6)8M4L;:]C1MRI<0K(%/J 0>:GA
MABMH$@MXTBBC4*D:*%50.@ '04HZ7_KK<;\OZTL/HHHH **** "BBB@ HHHH
M **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
MHHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "B
MBB@ HHHH **** "BBB@ HHHH **** "F331P1-+/(L<:C+,QP /K6)X@\6V&
M@H49O/NL?+ AY'^\>PKR_6O$6H:[-NO)L1 Y6%.$7\._U-2Y)&,ZL8:=3M=<
M^(T$&Z'18Q<2=/.<80?0=3^GXUP.H:K?:K-YNH7,DS=@QX7Z#H*J45DVV<<J
MDI[A1112,PHHHH **** "BBB@ JG+_KG_P!XU<JG+_KG_P!XUI ZL/NQE%%%
M:'8%%%% !1110 4444 3V=]=:?<">QN)()1_%&V#_P#7KO\ 0/B@R[8-?BW#
MI]IA7G_@2_X?E7G-% 'T397]KJ-JMQ93I/$W1D.15BOGS2M:O]%NO/TZX:)O
MXEZJ_L1T->J>&?B!9:SLMK_;9WIX )^20_[)['V/ZT =?11G-% !1110 444
M4 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110
M 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !
M1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%
M%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 444
M9H *@N[RWL;=I[N58HU'+,<5@:_XSM-+WV]IBYNAP0#\B'W/]!^E>=ZCJEYJ
MMP9KZ9I#V7/RK]!VJ')(PJ5E'1;G6:Q\078M%HT6T=//E&2?HO\ C^5<;<W=
MQ>SF:[F>:0]6=LFH:*R;;..4Y2W"BBBD0%%%% !1110 4444 %4I/]:_^\:N
MU2D_UK_[QK2F=6'W8RBBBM3L"BBB@ HHHH **** 'Q2R02K+!(T<BG*NC$$?
M0BNST/XC7=IMBUA#=Q=/-7 D']#^E<310![QI>L6.L6HGL)UE7N.C*?0CM5Z
MO ;*^NM.NEN+&9H95Z,O^>:]'\.?$*WO3':ZP%MYSP)A_JW/O_=/Z4 =O12!
M@PR.:6@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
MHHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ***BN+F
M*TMWFN)%CC099F. !0!(S!%+,0 !DD]J\_\ $_C_  7L]";/9[K_ .)_Q_+U
MK'\5>,IM9D:ULF:*Q!P>S2^Y]O:N6K.4NQQU*U](BN[2.SR,69CEF8Y)/K24
M45F<H4444 %%%% !1110 4444 %%%% !5.7_ %S_ .\:N53E_P!<_P#O&M('
M5A]V,HK1T/0[OQ!J0L[$*&VEF=SA44=S7;WWPUMI-+@CTF_@>_C5RY8D"XPV
M#_$=NWIP.O7%:/17.S=V/-Z*UM(\-ZAK.KOI]M&$EB)\YI#A8L'!S^/%=3J'
MPJN[>R>6PU!;N91GR6A\O</8[CS^7UHZ7#K8X"BM31/#]]K^I&RLD"NH)D>3
M(6,>_P#+%=7J'PJN[>R>6PU!;N91GR6A\O</8[CS^7UHV5PZV. HI64HQ5@5
M8'!!'2DH **ZOPQX%G\0Z;+?277V2%6VQDQ;_,QU[C [?GZ5RI&"11L[!TN)
M1110!W'A3XA3Z=LL]:9I[7HLW5XQ[_WA^O\ *O5+:YAN[=)[:198I!E74Y!%
M?.=;_A?Q;>>'+H!29K-S^\@)_5?0_P Z /<J*IZ7JEIJ]BEW8RB2)_S4^A'8
MU<H **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
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
MN Y4YS]:\A@\HW,?VG=Y.\>9L^]MSSCWQ7IECX3.EM=77AVXDGANM.=89#*
MXD)!7!&.".A]JV?P_P!=CK7Q'$^(K#6M,GBM=:GEF11F!FE9T(Z?+GIVXX[5
MVWACQ9_:NMP:98VJVMC#;GC W.1CGC@#V'YUF>/[G9HNDZ?>2K+J,:AYB#G'
MRX.?J?Y5G?#C_D;!_P!<'_I1'=H);)E36HA<?$&>%QE9+T*0?0L!6Y\3KN5-
M3L((Y&18XS*H4XPV<9^O%<]XCF-OXVO9E&3'=;P/H<UW'B3P\?&<.GZCI5U"
MJ;,$R$X*GGC /(.1BIC?DC;^M"G\3,_X@_Z5X9T>]D_UK8R?]Y,G]16%X%T.
M/5=9-Q=@&TLP)'W=&;^$'VX)_"M7XCWD,<6GZ/ X8VR[I #]W@!0??&34^EC
M^ROA-=W2<276[)'7EMG\J=[<TE\A6NHQ9RGB?6WU[6Y;DDB%3LA0_P *C_'K
M^-8]*.:V->\,7OAU;=KV6"07&[;Y+$XQCKD#UI6LA[L[?PQXL_M76X-,L;5;
M6QAMSQ@;G(QSQP![#\ZX?Q9_R-FI?]=VK4^''_(V#_K@_P#2LOQ9_P C9J7_
M %W:G+XD_7\Q1V:,>BBB@"_H^L76B:@MU9O@CAT/W7'H:]DT+7;77M/6XM6P
M1Q)&3\T9]#_C7AE:>A:W<Z#J2W-N24/$L>>'7T_^O0![G15/2]3MM6L([NSD
MWQN/Q4^A]#5R@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB
M@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH "<"O./'GB@RR/I%B_
MR*<7#@]3_<_Q_*N@\9^(_P"Q=-\JW;_2YP5C_P!@=V_SWKR4DLQ+$DDY)/>L
MY2Z'+7J6]U"4445F<84444 %%%% !1110 4444 %%%% !1110 4444 %4Y?]
M<_\ O&KE4Y?]<_\ O&M('5A]V,!(.1P:V]3\47NIZ+9Z?-+<8@#"9VN&;[1E
MLC</;MG-8E%:'8*"58%200<@CM5J[U;4;^,1WU_=7**=P6:9G /K@FJE% "@
ME6!4D$'(([5:N]6U&_C$=]?W5RBG<%FF9P#ZX)JI10!/:WMU8RF6QN9K:0C:
M7AD*$CTR*BEEDFE:69VDD<EF=SDL3U)/>FT4 6;34KZP5Q8WEQ;"3[XAE9-W
MUP>:K444 %%%% !6KI>DFYQ-< B'LO\ ?_\ K4_2M(\W;/=+\G54/\7N?:N@
MZ=*ER,:E2VB$ "J H  & !VI:**S.8W/#FM'3[CR)V_T>0]3_ ?7Z5WBL&4$
M<UY/79>%=8\Z'[%<-^\C'[LG^)?3\*N+Z&]*?V6=/1115G0%%%% !1110 44
M44 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !111
M0 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%%
M!1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %
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
M-0HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH *
M*** "BBB@ HHH/% %+5K^/3=-GNIC\L2%OKZ#\37B%W=2WMY+<SMNDE8LQ^M
M=U\2-6.V#38F^]^]EQZ?PC\\G\!7G]93>MCBKRO*W8****@Y@HHHH **** "
MBBB@ HHHH ***D@@DN91'$N2?TH&DWHA@!8@*,D] *U+'0Y;@@S J/[HZUN:
M-X<QABNYSU8BNSL-%2)066M%'N=E.@EK(YG3O#>% 6/:/I70VOA]$ W+6[%;
MI&.!4H %:'3L4(M+B3^$586TC7^$58HH B^SIZ"C[.GH*EHH @:TC;^$57ET
MR)Q]T5?HH Y^YT"-P<+6!J'AL,A5HPR^A%=_C-120*XY% 'BNI>%WA):UR/]
MAOZ&N?DC>*0I(I5AU!%>[7VCQS*<+7&ZYX9692&3D?=8=10!YS15J^T^;3YM
MDPX/W6'0U656=@J LQ. !WH  "S *,D\ #O70Z7I M\3W(S+U5?[O_UZ?I>E
M+:*)9P&F/3T3_P"O6G4.1SU*E]$%%%%08!1110 4444 %%%% "@D$$'!'0BO
M1="U#^T--CD)^<#:_P!17G-;WA2^^SZ@;=C\DPX_WA_]:JB]36G*TK'=T4BG
M(I:T.H**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@ HHHH **** "BBB@ HHH)P* ,[7-232]*FN7QE%^4'^)NP_.
MO'9)'FE:25BSN2S,3DDFNN\?ZF9;V*PC;Y(QYC@?WCT_3^=<=6,W=G#7E>5N
MP4445!SA1110 4444 %%%% !1110 445+!;R7,FR(9/<^E TFW9$8!9@%&2>
M@%:ECHDMP09 1_LBMO1_#W0E<GN2*Z^RTF.%1E:UC#N=E.@EK(YW3_#@"CY,
M#Z5OVVB11@945K)$J#@5)6ATE2.QC3HHJ<0(.U244 ,\M?2E\M?2G44 1F!#
MU J%[*-QRHJU10!D7&C12 X45A7_ (<#*?D!'H17:4QHU8<B@#Q_4O#+Q$M;
MC:?[IZ&N?EB>&0I*I5AU!KW"\TN.93\HKD=:\-)*IW)GT(ZB@#SJBK=_I\MA
M-MD!*D_*V.M5* "BBN;UWQ,(-UMIS!I>CRCD+]/>@B<U!79>UG7X-+4QIB6Y
M(X0'A?<_X5PUS=37EPTUS(9)&ZD_RJ-W:21GD8LS'))/)-)2//J5'-A11109
M!1110 4444 %?4OPM\4_\)-X+M9)Y UY;#R+C+9)9>C'_>&#]<U\M5Z'\&?$
MC:-XU2PE?%KJ0\I@3P)!RA_/*_\  J#>A/EGZGTO12*<K2TST0HHHH ****
M"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH *
M*** "BBB@ HHHH **** "BBB@ HHHH **** "O+_ !OXX\1/XO7PMX'B!O$
M::4(K,3C=M&_Y5 !&2>_IW]0K)UG2GEM[J[T2&S@UQX3%!>RQ+N3ZMM)Q[8(
MI/N-'#^ /'6O7/BF?PMXPB4W\:L5E"*K!AR5;;\I&.01Z=\U4\3>//%.K>,+
MCPYX"@ DLV(EEVH6<KPW+_*J@G'J2.O.*J>#G/AKXJ7%EXQB>?7K\?NM1$P:
M)@W("KM!&<$9[8   S4>O:)XL\$_$.\\0^&-/;4H+\N<)"TV-YW,K*OS###(
M(XZ<]13WY6^WXBVYDOZ1M^!/'NM2>)I/"OC6$)J2Y\N;8JEF W;6"_*?EY!'
M''?.:@UKQ9XGU'7/$@\/ZC%I]GX=AW&,VR2&Y8?>!+=.C=/0>N:XVSN-=NOC
M/I%YXA@6UU"YFBD,*@+Y<9& ",DCY>QY]>:[_7/A_KIUK6KCPS?6$5IKL6R\
MCNPX:,]RFT'.>>O]X_6AW:3]?OZ!HFUZ?=U-;_A.T3X6IXIN(T2:2#Y8AT:;
M)7 ]MPS]*UO!_P#;#>%[27Q'<FXU"=?-DS&J>6&Y"84 <#'XYKSVZTZTEU_2
MO!_G@Z)X8@^VZG,XPLCXW<C_ (%T]&;TKK=(^)FB:OJUM8QP7]J+PL+.XNK?
M9%<E3C"-DY_$#TX/%5HVVNNWI_P?T%JDK]-_Z\OU.PJ.=Q'&23@ 5)7.^--1
M^P>';DJ</(/+7ZMQ_+-2#=E<XJ/1=0\9:]<7JJ\5F[D+<,N5  (4 $C/3!QT
MS7/ZEI5[I-SY&H6[P/U&[HP]B.#^%>LV*33:?HDNE2!;&.$^:B-R?W>!]<-G
M(]?I7G6NZ[JUW8PZ9K-J$> AEDEB99CVR<GG/TYQ64TD['%*$>3F>[*>F>'=
M6UB-I-.LGEC4X+DA5)]BQ /X5!J6DWVD7 AU&V>!R,C/(;Z$<'KVKJ;[QVEO
MHUI9>&UEM6A 5WEC0\ =N3U/-:?BF=[GX=VDVKJJWLC1LHQ@[O7';Y<YH<5T
M$H0:WUM<XO3?#6KZO"9M/LGEB!QO+*@/T+$9_"J=]876FW36U] \$J]58=1Z
M@]Q[BO3_ !+<:QIVD:>OA:)S%C#&"$2$+@;>,'CKSBJ?C>UDO/!EK?7T*QWL
M.PR ?P[N&'YXHE&U[=!^R5O.U_(X;3/#^J:RCOIUF\R(<%\A5SZ98@$^U5K_
M $V\TNZ-OJ%N\$H&<-W'J#T(]Q7H>OZC<>%/"6EVVE,L,K@ OM!Q@98X(QDD
MU7\4N-;^'UEJTJ*)T*EF QU.UOPS@T2BE>W0/9QM;K:YYW5_3-#U+668:;:/
M,$^\V0JCVW' S[50KUBXAU+1_"MA8>&K<M=R ;G"CY>-S,=W')XY]?I22T;9
MG3CS/4\SU+2;[2)Q#J-L\#L,KG!#?0C@U3KLO%NLZE+I,&FZ]I)@NE(=;KS
M0Y Y( &.AY -<<JL[A4&68X '>IZA.*3LB2WMY+J811#)/Z5W6@^'UC5?ESZ
MDCK4?AO0?+12XRS<L:[RTM%@C  K:,;'72I\JN]QEG8) @P*N@ =*6BJ-PHH
MHH **** "BBB@ HHHH ****  C-5;FS292"*M44 <+KWAY)HG5DW*?TKE+/1
MTTV1BWSR9.&(Z"O7KBW65""*Y#6M+*Y9!2>J)FFU9'.44I!4X/6DK(X@HHHH
M **** "BBB@ HHHH *?#*T$Z2IPR,&'X4RB@#T^PN5NK2.5.CJ&%6JYGPC=^
M98M QYB; ^AY_P :Z:M4=T7=7"BBBF,**** "BBB@ HHHH **** "BBB@ HH
MHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB
M@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
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
M@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH *
M*** "BBB@ HHHH 0G KS?Q_>>;J<%L#Q$A<_4G_ZWZUZ-*<(:\?\17'VKQ#>
M2=A)L'_ >/Z5$WH85W:%C,HHHK$X HHHH **** "BBB@ HHHH *Z'PUI9N)1
M.X]E_P :P[:!KFY2%>K'%>H:#IZP0)A< # K2"ZG30A=\S-2QM5AB  J[2 8
M%+6IVA1110 4444 %%%% !1110 4444 %%%% !1110 57NK<31D$58H/- 'B
MWQ6\)&^TLWMM&3<V>6&T<LG\0_K_ /KKPZOL/5[(30MQ7RYXVT(^'_%-S;(F
MV"0^;" . I/3\#D4CBQ$/M(Y^BBB@Y HHHH **** "BBB@ HHHH [WX.:R=+
M^($$#'$5_&T#9['[RG\UQ^-?3:'*BOC70[\Z7X@L+\?\NUS'*?<*P)K[%M7#
MPJ0<@CK3.[#.\6B>BBB@Z@HHHH **** "BBB@ HHHH **** "BBB@ HHHH *
M*** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HH
MHH **** "BBB@ HHHH **** (+MML)^E>%WTYN=0N)S_ ,M)6;\S7L^NS^1I
M5S(/X(F;\A7B-9S.3$/9!11169R!1110 4444 %%%% !1110!T7@^P^TZ@TS
M#(0;1]3_ )_6O5[.$10@#TKC? UCLTV-R.9"7/\ 3],5W2#"BMHJR/2I1Y8(
M6BBBJ- HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** (KB,21D&N=_
MLO3KB>:"]4)._$,A8C!/&,9P3GM73$9%<WXCM=]NY YQ1T$]BV(!I%Q'>WES
M&D45HL!4'EV'/'K7"7$OGW,LN-OF.6QZ9.:DO+V2],)E51Y42Q+M!Y ]?>J]
M9-W9RRDFK(ZW4O\ D0[3_@']:Y*BBAZMLERNEY'6ZE_R(=I_P#^M5?">H00O
M/973!$N -I8X!/0C\:YRBG?5ON/G=TUT.YM;"U\+0W-U-=;RXPBE<$^P]37/
M:/HYUV6Y=KCR2A#'Y-V<Y]QZ5CT4KC<D]+:&[X=M=+O)9K;44!F)_=,7*Y[8
M'.,UM6]K:>$[2>::Y\V64?(N-I;'8#/J>37$44[Z!&270<[M)(SL<LQR3[UZ
M/I$WG6$3G^) ?TKS:N[\,2[]*B]@1^1IQ+I/5F]7CFM_\G'Z=](__19KV.N:
MNO NF7?C:'Q1)/=B]AV[8U=?+.%QR-N>_K5KXHOLSH?PM=T97Q7ET-?#=M'X
MFBU-K5[D%)-.V;D<*<9WG&""?6N.\:GX?'X:QG0!8"]_=?9A#M^T@YY\S'S?
M=W9W=\>U>OZKI-CK>G26&JVR7-M+]Z-L_F".0?<5Q5C\%O"MEJ"7,AO;M4.1
M!<2J8SZ9"J"<>F?KFIMNAWV9I?#RS-S\+M+M=8MTF22$YBG0,K1ER4R#U&W%
M8GCFVM[J\TKP'X>MX;-;^87-ZEM&$5(5Y)( QDXS_P !'K7I*JJ(%10JJ,
M8 %<[H_A/^SO%NJ^(+R]^V7=^ D8\K8((Q_ .3GHO/'3WJVU*=^G]6)6D;=3
M@=3N]1UCQ#XEL;/5K[2++PW8,;."PE,08HH^_C[PXZ?ECG/>_#_7+KQ%X(L-
M0OR&N6#)*X &\JQ7.!ZXK,U_X=2:EK%]J.BZW)I#ZE 8+Z,6XF692 .,D;3@
M<D?ISGI]"T:U\/:);:78!O(MUVJ7.68DY)/N22:4?AUW_76['+?3^EIH:%%%
M% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444
M %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110
M4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% %:
M^E$5L[MT4$FO%'<R2,[=6))KUOQ--Y.A7;=_*8#ZD8KR*LJAQXAZI!11169R
MA1110 4444 %%%% !1110!T/A.Q,]XTQ'"_*/Z_TKTVUB$<0 KE/!UGY=A$Q
M'+#<?QKLE&%%;Q5D>E3CRP2%HHHJC0**** "BBB@ HHHH **** "BBB@ HHH
MH **** "BBB@".9-\9!KQ3XU:'OTF'4HU^:TEPQ_V'X_]"VU[>>17)^-=*74
M]!O;4C/G0L@]B1P?SH(G'FBT?*-%%%(\H**** "BBB@ HHHH **** "OK?P)
M?G4?!>DW+'<SVD>\_P"T% /Z@U\D5]+_  9O/M/P[L5.=T#21'\')'Z$4'5A
MG[S1Z'1113.X**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ H
MHHH **** "BBB@#GO&$OE^'KP^L17\^*\>KUCQRVWP[<_0#_ ,>%>3UE/<XL
M1\2"BBBH.8**** "BBB@ HHHH ***D@7=<QKZN!^M SV#PY;>1I\*8^Z@7\A
M6]6?I:;;=?I6A70>J%%%% !1110 4444 %%%% !1110 4444 %%%% !1110
M4444 %9NK1;[=OI6E56]7= WTH \SE79,Z_W6(IE6+]=M_,/]HU7K(X7HPHH
MHI""BBB@ HHHH **** "NQ\)/FQQZ.17'5UGA _N)!_TT/\ (54=S6E\1UM%
M Z45H=04444 %%%% !1110 4444 ?/M%%%<YY(5H>'_^1ETS_K[B_P#0Q6?6
MAX?_ .1ETS_K[B_]#%-;E1W1[E1116YZ@44V1UBC:20A44%F)[ 5Y3H]_<VW
MBC3_ !C<LXL_$5Y+9LK?P1\"W_\ 0#S[T+5V_KR^\'HKGK%%<3XPDEB\?^$I
M+>'SYE%ZT<6X+O80C"Y/3)XS6=\/?$%]#INO7>O6)M;*&\N)Y;R2Z$A5P1F+
M:.3M X/0]!0O\_S!Z6\_^#_7_#'H]%<-_P +!U.*V35;WPC>6^@N=WVXW"-(
ML9Z.T(^8#H3SP.>>^AXC\:'1-4TVQL=,?5)M3B=K80S!=[#&T<C&#G);/ &>
M: .IHKB+SQYJNGZAI^FW?A>0ZG?P/)%:0WBN58-@*S;0H! R6SP/6M#P]XQ?
M5+S4;#6M+DT:_P!.02S0R3"53&1D,'  /O\ ASUP =/17"-\0=8GT^35]+\'
MW%UHJJ9!>/>QQNT:_>818)[''K5#QIXCO[QO"=WH6G27=G=7<5Q$3<K%YTF"
M1"RGIZ[CD BCM\OQ ]*HKAY]<6Q\1ZQ=1>']VKVVD17,W^F']XN<F+&"HV\_
M,!S6U?>*[6S\#_\ "2I'YL+6RS1Q;L%RV-J9QUR0.E'2_P#77_(.MOZZ?YF]
M17+:IXPNK6ZM-,TO1)=3UJ:W%Q-9QSJB6Z$<[I6&.O XY]N,RZ/XQ2\AU"/6
MK&71[[3(_.NK:5Q(!'@D.C#[ZX'4#KQ0!TE,DGBA,8FE2,R-L0,P&YNN!ZG@
M\>U<7;^.M<N(8M07P5>OH\Q#1W,-RDDS1D_*WD ;N>#C/ YS6N7-W\0DCDR8
M[/31-$I&,/*Y4M]=J8_X$?6CK;^NX=/Z]#=$\1N&@$J&95#M&&&X*20#CK@D
M'GV-5M-U.'5()'A62-X96AFBE #QNO4'!(]#D$@@@U@:-I\=A\1-9V2S2M/9
MP32/-(7))DEX'8 #  &  *DTPE/B3KJ0Y\I[2VDEXX$OS@?CM _*A= >E_*W
MZ?YG3T444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%
M%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110!S7C
M5]OAZXYZ[1_X\*\NKTGQVQ&AN!W=?YUYM6,]SAQ'QA1114'.%%%% !1110 4
M444 %*!D@#J:2IK4;KR$>LBC]:!K5GJV@P"*T11V4"MJJ&F+BW7Z5?KI/5"B
MBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "L_5(]]NWTK0JM
M>KF%OI0!\@>([7[%XGU.V PL=U(JCVW''Z5FUTOQ$B$/Q!U9%Z>:K?FBG^M<
MU2/)DK2:"BBB@D**** "BBB@ HHHH *^@?@-+N\'7*=TOG'_ (XA_K7S]7NG
MP"D)T74H^<+=!A^*#_"@WP_QGLPHH'2BF>B%%%% !1110 4444 %%%% !111
M0 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%%
M!1110 4444 %%%% !1110 4444 %%%% !1110!R7CO\ Y%ZX_P" _P#H0KRJ
MO6?&R[M!N?\ <S^1KR:LI[G%B/B04445!S!1110 4444 %%%% !4MJ=MY"3V
MD4_K45*K;6##L<T#1[KIW_'NOTJY5'2Y!)9QNO1E!%7JZ#U0HHHH **** "B
MBB@ HHHH **** "BBB@ HHHH **** "BBB@ J"Z_U+?2IZ@NSB$_2@#S?4O^
M0E/_ +U5:GO7WWT[>LA_G4%9,X9;A1112$%%%% !1110 4444 %=5X1_U4G_
M %T_H*Y6NK\)#]RW^^:J.YK2^(ZX=**!THK0Z@HHHH **** "BBB@ HHHH ^
M?:*T/^$?UG_H$WW_ (#/_A1_PC^L_P#0)OO_  &?_"L+,\OE?8SZT/#_ /R,
MNF?]?<7_ *&*/^$?UG_H$WW_ (#/_A5[0]#U:+Q!ITDNF7B(EU$S,UNX"@.,
MDG%"W'&+NCV2BBBMSTSE?B%>31^&QI=BVV]UB9;& C^'>?F8^P7-<[K/P\\3
M7/A5M-/BT7=O:Q V]H-+CCR8Q\BAPV1T S7HTMG;3W,-Q-;Q23V^?)E= 6CR
M,':>HR.N*FI6T\_ZL.^IYG:ZZ/$>N_#[4CQ+(MXLP])%B"M^H/X5GP*-3^'7
MC#1[&:-]2&HW,WV4,/-*+(I)V]><$ ^M>G0:)I5M+');:99PR1.TD;QVZ*49
MAAF! X) P3WJ2'2]/M[^6^@L;:*[F&);A(5$CCW8#)Z#KZ4WK^/YW$M+>5OU
M_P SR5QX3F\)B\N?'GB*X26$*^G+JBO*Q/!C\HKGKQSQ[XYKI7M([3QYX(MH
MTG5(-.G1%N"/,4"-0 V.-WKCC-=@F@Z/'J/]H1Z58K>EB_VE;9!)N/4[L9S5
ME[.VENXKJ2WB>XA!6*9D!= >H#=1GVIWUO\ UL_\Q6TM_6Z_R./U0 _&C0\C
MIIL^/;DU4N+B>S^*/B*YLX_-N(=!$D28SN8'(&/K7=M96KWJ7CVT+74:E$G,
M8+JIZ@-U ]J%LK5;Y[Q;:$73H(VG$8WLHZ*6ZD>U3;1+U_&_^975OT_"W^1X
MU+_86J^"#J6O>++_ %?4[F!G&D_;<I]H8':! OS+M8\=OPXK1^UP0>!_AY=7
M$J16\-_#YDLC *F%8$D] .*]+AT+2;:]DO+?2[**YEW>9.ENBN^[KE@,G/>G
M-HNEOIJZ<^FV;6*G*VI@4Q#G/W,8Z\U5^W=/[B;=^S_$Y?3)(KKXM:K)$Z30
MRZ3 592&5U+'D'N"*Y?2X)KC7[3P#*&^S:3J4MW+D?>MEP\(/L6?!_"O5(-.
MLK6;SK6SMX9?+6+?'$JML7HN0.@[#I3EL;1+Y[U+6%;J1 CSB,"1E'0%NI'M
M26C7]=;K[AO5/^NEF>8Z]8VL/Q5OO[<U_4=!M]0MHGM+FTNQ;I)L7:R.Q!&1
MU&?7W%6_#]KX?77M9N-/U#7?$SVVGF&YFFG2XBD1OF\I&X+-P< '')YS7H5]
MIUEJ=OY&I6=O>0[MWEW$2R+GUP1C-+9V5KIULMMI]M#:P*25B@C"*,\G '%)
M:*PWO?T_ \=,^EZ)X>&L^"/'<]BD<3R)HM[*DW);/EB(GY3G(SACWSWKTZVM
MKR\N-(UHQI;W+6OEWD#Y'RN V!Q]Y7' /8M5L>']&74/MXTBQ%YOW_:1;)YF
M[^]NQG/O6A57)*Z6%LFHRWZ1XN98EB>3<>54D@8Z=6/YU7TG2O[-6YDEF^T7
M5W,9IYMNW<>  !DX4*  ,GI6A12&%%%% !1110 4444 %%%% !1110 4444
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
MOV[=]H_M%TEWG+;PJ[LGUR34?:_K\#MC%*2=K'K8Z44#I2.ZQHSNP55&2Q.
M!ZU1T"T5Y[?_ !J\+65]);QK?7BH<>=;1*48^Q9@3]<8],UTDWC/1U\(S>)+
M69KS3X1EC OSYR!C:V,'D<'%'2X=;&]17FG_  O7PS_SXZM_WYB_^.5T7ASX
MA:-XDTO4-0MUN;2VT\!IWND5<#!.1M9L\ T=+AUL=317(Z'\2=&UW5H+".WO
M[1[I6:UDNX-B7(!.=A!.>AZXZ8Z\5J:_XILO#L]A;W,5Q<7.H3>3;P6RJSL?
M7D@8&1S[T=@-JBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH Y
M_P 51>9HUTN.L3?RKQVO;M9C$EJZD9!!!KQ-T,<C(>JD@UG,Y,0M4QM%%%9G
M(%%%% !1110 4444 %%%% 'L7@^Z%UX>M'!Y$80_5>/Z5T%<!\-[\-:W%DQ^
M:-]ZCV/_ -<?K7?BMUJCTZ;YHIA1113+"BBB@ HHHH **** "BBB@ HHHH *
M*** "BBB@ HHHH *S==O4T_2+J[E_P!7;PM*WT4$G^5:5><?&O6_[+\ W$"-
MB6^=;9?H>6_\=!'XT$R?+%L\:@^*.K*Y-U:VLX)R=H9#^>2/TK9M?BE8/_Q^
M6%Q"?^F;"0?KBO,**7*CS.9GM-KXY\/7> NH+$WI,A3'XD8_6MFVO;6\7=:7
M,,XQG,4@;^5?/M*K%&#*2I'0@]*7*5SGT117A5IXFUJRQ]GU.Y '16D+*/P.
M16U:?$K7(,"X%O<CN7CVG_QT@?I4\K'SH];HKS^U^*D)P+[3)$]6AD#?H0/Y
MUM6GQ!\/W.-]U);L>TT1'ZC(_6E9E<R.FHJG::OIU]C['?6\Y/9)03^57*0P
MKL/"J8M%/J2?UKCZ[GPW%LL8_P#=!JX[FU'<Z =****LZ0HHHH **** "BBB
M@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH *
M*** "BBB@ HHHH **** "BBB@ HHHH **** "O+O'FD>%]>^(>G0:[X@>TN$
MC2(V/V=\2@L2,2_=7.<=_P *]1KDO&_P]T[QK'')-,]I?0KLCN47=\N<[67C
M(Z]QUZTNJ8^C1YC\2?#VE>#/$VBR>%-UM=L=YMTE9RI5AL;YB2,DD>G'UKJ_
MB%HOA+Q+XDM=/OM7;2O$#JD:$6[NLBL?E#<!2>H!W#K@YX%6O"?P<TWP[JR:
MC?WSZG/"P:!3"(D1A_$1DY([<@#TK5\<_#FP\:^5.]P]E?0KL6X5-X9<YPRY
M&>^.1U[TWLEOJ+JWMH>0>(/!EIX>\9:-H.G7KZA>RR(;E\!0K,_R@+SM^7GD
MGKGI7NVO>$-!\3M$^N:<ER\(PC[V1@/3*D$CV-<[X,^%.F>$[[^T+BY;4KY,
M^5(\81(@>X7)^;KSGOT%=M=_:/L<WV(1FYV'RO-)";L<9(!.,^U#^&V_4%\5
M]CS*Z@LM5\?0Z996ZIX?\(VYN)HHA\IFQD+[GCOW#>M9?_"P/%46AV_C"2_@
M;3IM1-L=)%LN FT\^9][/!_'VXKOO!?A!O#WAJXL]4DCNKV^DDEO94)(<MQ@
M$@$C'KW)KDA\*];:SA\/2ZE8GPY#?&[#A7^U$;<;2,;>Y&<]\^U-73L_Z=]?
MP!VM?^K6T_'4]41Q)&KK]U@"*=2*H10JC  P!Z4M(2O;4YOQA'OT.Y]DS^7-
M<_X0\4&#[/H]Y!YT3R!87&,H2<@$'J,\Y[>]==KL/GZ?-'_?0K^8KF/#>OV%
M_9V6E:J'2>WE0VT@!(8@_*..AYQZ8I?:.>KI-.]A_B/P_I=[IU[J>D.5GMY&
M\]<L0S _-D'H><\<4S3]/N] \*/<VEK--J5^,*88RQA0CVZ>OUQZ5'X@UO3]
M/L[_ $O2/,DFNIF:YE<8"DGYE'Y8^G<UC1^,==BB6..^PB*%4>2G '_ :QO'
M7^O4F<H*=WN=-XHL[J7P3IL<=O,\D8C,BJA)3"')([56T_3[O0/"CW-I:S3:
ME?C"F&,L84(]NGK]<>E&M>,5E\.V\>GWV;UU5;D>3U!4[NJXZ^E84?C'78HE
MCCOL(BA5'DIP!_P&JDUS2\Q.4$T_(Z;Q19W4O@G38X[>9Y(Q&9%5"2F$.21V
MJMI^GW>@>%'N;2UFFU*_&%,,98PH1[=/7ZX]*-:\8K+X=MX]/OLWKJJW(\GJ
M"IW=5QU]*PH_&.NQ1+''?81%"J/)3@#_ (#1)KFEY@Y033\CIO%%G=2^"=-C
MCMYGDC$9D54)*80Y)':O/:[G6O&*R^';>/3[[-ZZJMR/)Z@J=W5<=?2L1?\
MA'_^$2.[_D,=O]9_?_[Y^[4SUDV3/EE:SZ'0:6R^'_ XN]1W7L5R04LW ,8R
M>.H/IGT]N]&K%?$7@D7MANLXK4DM9KC9\O7H!T!R.WMWJEI?B'2KWPZ-&\0>
M9$D8PDR G@=.F2#^!%)JFOZ58^'3HWA]I9DD'SS.".">>H&2>G0#%5-JS_ J
M,ERK72VIU7A>Y%SHMLX_YY@'ZC@_J*W*X?P!>[K26U9N8GRHSV/_ -?/YUW
MZ5HG='3!\T4PHHHIEA1110 4444 %%%% !1110 4444 %%%% !1110 4444
M%9VMWD=CI=Q=3'$<,32,?8#)_E6C7GGQCUS^R? MS'&Q6:]86R$'LW+?^.@C
M\:"9/EBV>7_#K3_"?BPZIIOB94@UFZ<O97;S.OS-QM"A@K,&((!'.3Z5ZFFG
M)X(U*V\0Z]JUO;V5IHD6G&)6.9Y4.[@$#/ X YY/2OF6M77?$-WXA>Q:]CA0
MV5I':1^4I&43."<D\\^WTH;TTW_X#7ZGGTZBBM5_5TSU?X.^&;7Q!KNI^,;^
MW4A;Q_LD##*I(3O+>Y4, /Q/4"NC\8_"K5?&FJFZU+Q;L@0G[/:I8?)"OM^\
MY/JQY/TP!\X44G:R2Z#C52336Y[#\)M9T[P_K^O>$-9ND6&YF:*&:3Y%D=2R
M%3SP6&,?3'7%=7HWAK1?@UI^K:Q?ZQ]H-PNV"-XPC'&2(U&268DC)X'&< 5\
MYT4.]O.UKA&JHZ6T3NO(EN;A[N[FN)>7F=G;ZDY->T^+?^3;=%^EO_6O$:*3
M7N<J\OP(C4<9.3ZIK[SM?AY\//\ A/3J'_$T^P?8A&?^/?S=^[=_M+C&WWZU
MT7P7\7:?H5]J.AZS<I;07I#13R-M57 (()Z#(Q@GT]Z\HHJKZ^0HRY;-;H^C
M_#G@;1OA5>:CXAU#7BUL\1BC69 FU2=VW.3YC_*,8 [\<\6?A;<_VGHUWJNW
M8-1U"XN0IZ@,Y&#^5?,]?4?PNLA9>!-*C QN@$AX[O\ /_[-2W:?8[*,U)V2
MMU.W'2LWQ#I<FM^'KW3(;HVC741B\X)NV@]>,C.1D=>]:55-4U*VT?2KG4;Y
MBEO;1F1R!DX'8>_:B5K.YU*]]#@6\$>%O _@&]_M^.TOY"KDW<T"K([$?*D>
M22IXXP?4U2^"NCM-X.U$ZI;)-87MP/+AG0,C[1AC@\$9P/JM<:?$>G>.O%!O
MO'>LG3]+MV_T?3XHY'W#T!52!_M,>3T&.,>U^%M?\/ZU8-#X7G22VL@L91(7
MC$8(X # >E4KV;?4EVNDNAPGQ2@T32K&TT30] TM=6U5Q'&T=G&&C4D#(.."
M2< _7TK<O?!']D_"&^T+2(_-O'M]\K(/FGE!#-CUSC 'I@5S>DN?$_[0=Y<2
MG?#I2N(U/1=F$X_X$Q:O8*FUZ?\ B_+H5>T_3\SP[2;M/$VL^!;#2(I3/HJ9
MOR8F40;2N03COM_-@.M=7X9_XK'XDZCXF;Y].TL&RT\GD,W\3C\S_P!]#TK7
M^).O2:1X6:TL<MJ.J-]CM44_,2W!(^@/YD5QFN:*--U#P9X'F9FTR;Y[U(V*
M"YDW9.2.<9Z?7Z52;;^?XO?[E^9+5E\OP7^;_(]BHKS7X632VFN^*-!B>0Z?
MIUX1:QNQ;R@7<%03VX!^N3W->E4NB?<?5KL%%%% !1110 4444 %%%% !111
M0 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%%
M!1110 4444 %%%% %/4$W6[?2O%]:@^SZU=1XQ^\+#\>?ZU[=<KNB(]J\E\:
M6ODZPLH'$BX_$?\ ZQ43V.?$*\+G.T445D<(4444 %%%% !1110 4444 ;'A
M;4O[+\06\SMMB<^7(?8__7P:]GC;<@-> 5ZUX(UL:IHRQ2OFXMP$DSU(['\O
MY&M(/H=>'G]DZBBBBM#K"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HH
MHH 1CA37S3\</$@U?QBFFV[[H-,0HV#P96Y;\@%'U!KW/QUXIA\)>%;K4Y=K
M2*NR",G_ %DA^Z/ZGV!KY%N;B6[NI;FY<R33.9)';JS$Y)_.@Y<1.RY2.BBB
M@X@HHHH **** "BBB@ J]:ZWJECC[)J%S$/[JRG'Y=*HT4 =AHWCGQ%-J-M9
M^?%<M/*D2B6(=6('\./6OI[1X]ENH]J^7?AEIQU+Q]8#;E+<F=_;:./_ !XK
M7U98IM@7Z4'=A[\K;+5%%%!TA1110 4444 %%%% !1110 45A_\ ";^%/^AG
MT;_P81?_ !5'_";^%/\ H9]&_P#!A%_\50+F7<W**P_^$W\*?]#/HW_@PB_^
M*I\'C#PS=7$<%MXBTF::5PD<<=]&S.Q.   V22>U <R[FS1110,**** "BBB
M@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH *
M*** "BBB@ HHHH **** "BBB@ HHHH J7Z;H&^E>/WBR66K3"-FC>.4E64X(
MYX(KV:X7=$?I7E'BJV\C7';'$BAOZ?TJ)[7.?$*\;F.[O+(SR,SNQRS,<DGU
M)IM%%8G"%%%% !1110 4444 %%%% &OX8U#^SM=A=CB.3]V^3T![_GBO7(FW
M(#7AE>I^$-8_M+2461LSP_))GOZ'\1_6M(/H=>'E]DZ.BBBM3K"BBB@ HHHH
M **** "BBB@ HHHH **** "BBB@ HHHH :YVJ:^<?C7XD75O%4>F6[*T.FJ5
M9@<YD;&X?@ !]<U[;XW\3P>%/#-SJ,Y!=1LAC/\ RTD/W1U_$^P-?)MW=37U
MY-=73F2:=S)(YZLQ.2:#EQ$[+E(J***1PA1110 4444 %%%% !1110 ^&%[B
M>.&%2TDC!%4=R3@"OL'0;5;33H88QA(T"+] ,5\P_#S3?[4\=Z=&5W)%)Y[^
MVP9'_CV/SKZJLDV0K]*9VX9:-EJJ>K:39:YIDNGZI#Y]K-C?'O9=V"".5(/4
M"KE%!UG&_P#"I?!/_0%_\FYO_BZV] \*Z-X7CG30K/[*LY!D'FN^XC./O$XZ
MFM>B@#&TOPGHFC:O=:IIMEY-Y=[O.E,KMOW-N/#$@<^E;-%%'D!BW_A:RU/Q
M-8:W>2W$DVGJ1! 67RE8_P 6,9W=._84GB3PII_BB&W%ZT\%Q:OYEO=6LGER
MPGC[K8/H.U;=%'2P&+X:\*Z?X5LYH=/,TLEQ(99[BX??+,Q[L>/7T_4DUM44
M4 %%1W$\=K;2W$[;(HD+NV"<*!DGBL/_ (3KPY_T$?\ R!)_\32NB7)+=G04
M5S__  G7AS_H(_\ D"3_ .)H_P"$Z\.?]!'_ ,@2?_$T70N>'<Z"BL>P\6:+
MJ=[':65[YL\F=J>4XS@$GDC'0&MBF4FGL%%%% PHHHH **** "BBB@ HHHH
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
M %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110
MCC*FO/\ QS9?NTN%',;8)]C_ /7Q7H-8'B2Q^UZ?+'CEE./K2:NB)QYHM'E%
M%*RE6*L,$'!'I25SGF!1110 4444 %%%% !1110 5I:%JSZ/JB7"Y,9^65?5
M?\:S:*-AIM.Z/;[2YCNK=)H7#HZAE8=Q4]>9>$?$W]ES"SO7Q:.?E8_\LR?Z
M'_/>O2T<.H*G(/0BNB+NCT:<U-7'4444S0**** "BBB@ HHHH **** "BBB@
M HHHH *9+(L499R  ,DD]*<3@<UXC\8/B4"LWAO09^3E+V=#V[Q@_P _R]:"
M)S4%=G'_ !4\<?\ "6^(/L]C(QTRR)6+(QYC_P 3_P!![>F37"444CS)2<G=
MA11102%%%% !1110 4444 %%%% 'J_P.T<RZA?:HZ\(!;H?<X9OY+7T!$NV,
M"N!^%VAG2/"-E$Z%99$\Z7(P=S<X/T&!^%>@ 8%,]2E'E@D+1110:!1110 4
M444 %%%% !1110!G^(/^1:U/_KTE_P#0#7AM>Y>(/^1:U/\ Z])?_0#7AM9S
M./$;H****S.4Z#P+_P CK8?]M/\ T6U>Q5X[X%_Y'6P_[:?^BVKV*M8;'=A_
M@"LN^U^UT_7],TB:.9KC4O,\ED4%%V+N.XYR..F :U*XKQ-_R5/P;]+O_P!%
M5:^)+^MCHZ-G:T5YIINFZKXI\4^)[:?Q)JUC965[MACL[C8P8C^\<D* /NC
MR<]JJ?\ "2:^/A1)*;JYFN+._:RO+Z",-,MNI^:1?]K&!N/USGFDGI?T?WV_
MS';6W]?UH>K5FV^O6EW?ZG9VRRR3:9M$X5.K,I8*OJ<#]:X[P;/I=SK4<GA[
MQY?ZG'AEGL-4D,CR\9!3>%9<=20"#5+PUHC67BWQ?<)J^J2-8D "2YR)BT).
MZ3CYBO\ ">,4/1/T;$M;>J1V[>)[2&RTN>[MKRU?5)U@AMYX=LJNV>'&?EQC
M/6MFO&[RPGUOP[X#N[S5]4$]U<I [I<G()+GS 3D^9VW>E=CHL]U%\3]4TU[
MVZGM;;3;?8DTI8;N 6QTW'N<<U5M6O-K[E<F^B?DOQ=CJM1U"WTK3+B_O&*P
M6T;22$#)  R>*JG6T,.F30V5[-'J6W:T46X0AEW!I#GY1VSSS7G>M_:M5T_X
M@PW&I7RQ6,BR0QQSD* (VRF#GY#W QFKT,5SH>G^![>VU34)4O+M&E\^X+95
MH@?+XQ\@[*>E*.O_ )+^)4M/Q_ ])HK@(;?4?'/B'6/,US4M+TS2[DV<$6FR
M^2\DB@;V=\'/L/Y<YSKS6M:L/"7B_1[W499KW14C^SWZ'9*\;@%22/X@!UZG
MO2O[M_*X[:V^1ZA17E>J6VM^'=!TGQ2WB74[F[>6 W5K+(#;NCX!58^@P"!G
MGN>#6EXYU."W\0Q6^O>+9=#TPP;X8-,=Q=229Y9RJ':G. .AQ[53TT\[$K7\
MSN=1OH],TNZOIU9HK6%YG" %B%!)QGOQ2:9?Q:KI-KJ%NKK%=0K,BN & 89&
M<9YYKRW0M;NKO1_&6F-=:I=6$&FM/:OJP_T@!HVSD]U/!'MBO0O!O_(C:'_U
MX0_^@"A;-^GZ_P"0_P#@_I_F;5%%%( HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH *J7\/F0L/:K=-D7<I% 'C7BFQ-IJID PLO/XCK6)7H_C+2OM%HY1
M?G3YE^M><5C)69P5H\LK]PHHHJ3 **** "BBB@ HHHH ***,XZT (S!%+.0J
M@9))P *M>$?C1!I6KC3-35GT?A([KDM$?7'=/;J.OL/._$_B/[<S65B_^C*?
MG<?\M#_A_.N:K6,;:E1DXNZ/MVVNH+RVCN+65)H95#))&V58'H0>]2U\G>"/
MB9K7@J98H'^UZ<6R]E*WRCU*'^$_IZ@U]%^$O'^@^,;<'2[M5N0N9+24[94]
M>.X]QD59WTZL9^ITU%%%!J%%%% !1110 4444 %%%% !2,P12S$  9)/:LGQ
M%XIT?PKIYN];O8[=/X$SEY#Z*O4FOGGQ_P#%[4_%JR:?IJMIVE,<% W[R<?[
M9'0?[(X]2:#.=2,-SJ_BE\85:.;0_"-QNW92YOXSP!W6,_S;\O6O#^M%%!Y\
MYN;NPHHHH("BBB@ HHHH **** "BBB@ KH_ 6B'7?&%G R[H86\Z;CC:O./Q
M.!^-<Y7NGP8\-&TTAM2G3$UZP*Y'2,=/S.3^5!K2CS3/7-*@\JW7Z5I5' FR
M,"I*#T@HHHH **** "BBB@ HHHH **** /AVBNX_X4UX]_Z /_DY!_\ %T?\
M*:\>_P#0!_\ )R#_ .+H/+]G/L</6YX(_P"2@^'O^PI;?^C5K<_X4UX]_P"@
M#_Y.0?\ Q=:WA;X3>-=-\8:-?7NB^7;6U_!-*_VJ$[46123@/D\ ]*!QA.^Q
M]*4444'IF)XN\/GQ+X=EL8IA!<JRS6TQ'^KE4Y4_T_&L2'QQJVGQ"V\0>$-:
M:^C #R:9;BXADX^\&!&,\_+SBNVHH \Y73->U1/$VOZEI7V W^DM:VUB)/-F
M;"$@D =3G&.N>,>K(M/U;0%\,^([;2[F]%OI$=CJ%G$N)T7 (*H>20W4=?U(
M])HHVV_K?_,-]_ZV_P CAK$ZEXM\;Z=K,NDWFE:9I,<HB%_'Y<TTLBA3\G4*
M!WZ'^6+X8UK7O#&@7-A+X5U6YFFNIWLGA@^7+.>)<X*#.#NP00?:O4Z*+?UZ
MNX'E5]X4U#1_#/A87NFRZS;Z?<23ZG86ZB0NTAR"$Z/M)(Q_3)ITJ7E]XF\,
MWFD>$KG1]'M+U@R-:B*0LXPSM$GW%&W[QZYKU.BG?6_G<3U5OZ_K4Y3P-9W-
MI)XC^UV\L'G:U<2Q>:A7>AVX89Z@^HK3\6P2W/@W5X+>)Y99+.54CC4LS$J<
M  =36Q14M7CR^5BD[2OYW//]%\<2:;H-A8S^$/%+2VUM'$Y33"5)50#C+=.*
MLZ]]M\9>%X+[2M*OK.^TV_CNH+34H1 \QCYQR3@$,<'U%=O15-MN_4E*RL>6
MZWXBOM>\0>%"^A:AI=K'JD>]K^,1LTI!^55ZE0,_-Q5^"ZU3PEXRUY!X>U'4
MXM6F2XM)K- R;B,%9&. @SW/0<]*ZO7M _MN[TF;[3Y']FWJW6WR]WF8!&WJ
M,=>O-;%)?Y_I_D-[_P!>?^9Y'!HVL#X;W-M<:;<"\;7Q,T*0L25\Q2648R5Z
M\],5TUTUWI?Q;6];2[ZZL[^PCM5N+6'>D3B3)+G^$ =Z[:BA:6\O\K">M_ZZ
MW/+4\)ZEJGP[N[9;(B^M]9EO8+:[38LX#GY2&P,,I/7@T]&@U5K:ST;X9165
MX[K]HGU32HTMX%R-Y##!<^@&">OM7I]%"TM\OP_X8;U.#\62,-8EM_$?@]M>
MTB2'%G<:=:>;/"3]]6^;*^H9=O;J>B_#_0)+:QUI)M-N-/T6_E'V33;QRSI&
M5PY8$DKN]"<\?0GNZ*%H@(;.V^QV4-L)9)A$@022D%F ]2 ,FFM86SZBE\\>
MZYCC,:.6)V*3DX'0$X&2.3@>E6**.MP\@HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH *K7L7F0L/:K--<;E(H \A\167V/5GP,++\X^O?_/O637?>
M,=,,UJ9$7YX_F'N.]<#6$E9GGUH\L@HHHJ3$**** "BBB@ HHHH **** "NL
M\*^+6T[;9:BQ:VZ))U,?L?;^5<G133:*C)Q=T>YQ3)-&KQL&5AD,IR"*?7D>
MA>)[S0VV)^^MB<M$QZ?0]J])TC7['68-]K+\X^]$W#+^']:VC),[X58S]33H
MHHJC4**** "BBB@ HHHH **** "D9@HYJGJ6JVFE6QFO9A&O8=2Q] .]?/OQ
M+^)FLZS?7.CVRMIVG(Q!5&^>X7L6;L".=H]<'.*5S*I4C!:G2?$GXPB/SM'\
M)3AFP4FOXSD+ZB,^O^U^7K7B!))R>2:**#SYS<W=A11100%%%% !1110 444
M4 %%%% !6[X,T3^W_%EG9.A>'?YDW'\"\G/UX'XUA5[9\%_#1@L)-6N$(DNS
MMCR.D8[].Y_0"@UI1YIV/7M+MQ%;J ,<5H4R)-D8%/IGIA1110 4444 %%%%
M !1110 4444 9_B#_D6M3_Z])?\ T UX;7O&J6KWND7EK$5#SP/&I;H"RD#/
MYUYQ_P *SUG_ )^;'_OX_P#\16<DV<M>$I-61QU%=C_PK/6?^?FQ_P"_C_\
MQ%'_  K/6?\ GYL?^_C_ /Q%3RLY_93[&=X%_P"1UL/^VG_HMJ]BK@O#7@;4
M]'\0VU_=3VCQ1;]PC=BQRA'&5'K7>UI%61UT8N,;,*Y?7-'OKSQ]X:U*V@WV
MEC]H^T2;U&S<F%X)R<GT!KJ**KK<WZ-'+^%-'OM-U_Q+<WL'E17U\);=MZG>
MNW&< \?CBLC2-"\5:3X3O4TLQ6>I#59;M(9BCI<Q$_<)&=N[UX/TZUW]%&WW
M6_+_ "#_ #O^?^9YU_8>M>(_%6FZC?\ A:S\.FRN1<SWBW,<TUS@8"90#\=W
M;ITP;T&EZ[8>,?$/EZ6MQIVLH'6\%RJF)EB*A2AY.3QZ"NWHI6TM_6O_  R#
MK<\XE\-:_:^!?"J6FGK<:CHUTD\MFTZ)O W9 ?.WN.]6[VS\3:;XO7Q)I.AQ
MW_\ :%BD-W8F\2-K>1>?OG@CMP.<&N\HJF[N_G?\+"225OE^IYW8>%/$$NG^
M,XM52W6ZUA08'B<>6S&,@@<Y !(&2 3UJ2'2_$.H6OA/[?HWV*32;Q1.HNHY
M/W:Q;?,X/<_PC)%>@44EI^'X;#W_ !_$X9K#Q'X4\0:G=>'])AUK3]5F^T-!
M]J6WDMY<#<<MP5/MS],<U)_".N7/A'Q+<7R12ZYKH0_9H7&R)5P%CW,0"0,Y
M.<>GK7HE%*VEO*WR'?6_S.-\7:#J6I_#^STVQMO-NXFMB\?F*N-A&[DG'&/6
MJ]_I_B'0O'E_KNBZ+%K<.I01QLGVM('MR@ QENJGKQW],<]U15-W=_ZU)2LK
M?+[M3S:'PWXH?5O%%WJ=O [ZQI31Q_9Y5V)+M*K%\Q!Z8RQ &2:[7PS9SZ?X
M4TJSO$\NXM[2*.5,@[6" $9'!YK4HI;*R_K?_,?6_P#73_(**** "BBB@ HH
MHH **** "BBB@ HHHH **** "BBB@ HHHH S-5M1-"W':O(M=L#8:DX Q'(2
MR^WJ*]ME3>A!KA?%FC?:(&VCYQRI]ZF2NC*K#GB>=44I!5B&&"#@@]J2L3S@
MHHHH **** "BBB@ KB?%'B3[07L-/?\ =#B653]__9'M[]_IUF\4^)/OZ?I[
M_P"S-(I_\=']?RKCZTC'JP"BBBM!!3X9Y;:=)K>5XI8SN22-BK*?4$=*910!
MZEX7^.VO:2([?78DU>V48\PG9,!_O#AOQ&3ZUZSH7Q=\'ZX$4:FMC.V,Q7H\
MHC_@1^4_@:^5**#>-:<?,^WXI8YHUDA=9$895E.01[&GU\5:?K&IZ3)OTO4+
MJS;.<V\S)G\C746?Q>\<6:A4UQY5'::&-R?Q*Y_6@V6)75'U;17S1'\>O&*+
MAAI[G^\UN<_HPI7^/?C!E("Z<I/\0MSD?FU!7UB!]+4C,%4EB  ,DGM7RQ>?
M&;QO=J5&K+;J>H@MXU/YX)_6N8U+Q+KFL@C5=7O;Q2?N37#,O_?).!0)XB/1
M'U)KOQ-\)^'U<7FL033+_P L+4^:^?3"\ _4BO*?$W[0&HWB/!X8L5L$/ N;
MC#R_4+]U3]=U>/44&$J\WMH6=0U&]U6\>[U.ZFN[A_O2S.68_B>WM5:BB@P"
MBBB@ HHHH **** "BBB@ HHHH **** -?POH<GB+Q%;6" ^6S;IF'\*#J?Z?
M4BOJW0-/2UM8TC0(B*%50. !VKS;X3^#SI>EK=W4>+N[ =\CE%_A7^I^OM7L
M-M$(XP!0>A1ARQN^I,.****#<**** "BBB@ HHHH **** "BBB@ HHHH ***
M* "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
MHHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH S]3MA-"
MW':O)]7L38:@\>,(3N3Z>E>S2+N4BN)\6:/Y\!>-?G3E?\*F2NC*K#GB<#12
MTE8'G!1110 4444 %%%% !1110 4444 %/CD>&021.T;J<AE."/QIE% '6:5
MX]OK11'J$8NT'1\[7']#77:=XNTC4,!;E89#CY)OD/TR>#^!KR6BK4VC>-:<
M3W1)$D4,C!E/((.0:=7B-M?W=DP-I<RPG_8<BM2#QCKD'_+[Y@])$4_KC-5[
M1&RQ$>J/6J*\Q7Q_K"]5MF^L9_QI7^(&L-T2U7Z1G^II\Z*]O ]-I"P Y->4
MS^-=<F! NEB![)&H_F*R[K5+^]7;=WD\R_W7D)'Y4N=$O$1Z(]4O_$^DZ>#Y
MUY&SC^",[V_3I^-<GJ?Q"N)=R:7 (5Z"63YF^N.@_6N,HJ7-LQE7D]M":YNI
M[R8S74SS2'JSG)KEO&&C'4+$7=NN;BV!R .73N/PZC\?6NCHJ4[.YCN>-T5N
M>*M&_LO4O,@7%M/EDP.%/=?\/8^U8=;+4@****8!1110 4444 %%%% !1110
M!I>'M&EU_7K73H0?WK_.P_A0?>/Y?KBOJSP_IL5A8Q0P1A(XT"HH&  !P*\Q
M^$/A#[%IXU2[BQ=78!3<!E(^WY]?R]*]E@C$<8 IGH4(<L;OJ2T444'0%%%%
M !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444
M%%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4
M444 %%%% !6?J5H)X3QVK0I&7<N#0!XYXFTHVET9T7Y6/S^Q]:P:];U_2EN(
M7RN01R*\NU&Q>PNFC;.WJI]164EU.*O3L^9%6BBBH.8**** "N5\4>)/LX>P
MT]_WQXEE4_<_V1[_ ,OKTE\3^(_L*-9V+_Z2P^=Q_P LQ_C_ "KA"23D\DUI
M&/5@%%%%:""BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "B
MBB@ HHHH **** "BBB@ KLOASX4?Q!KB7-Q'FQM6#-D<2/U"_P!3_P#7KG=#
MT:YU[5H;"S'S.<LV.$7NQKZ9\'^&K?2--@M;:/;'$N 3U8]R?<T'11I\SN]C
MHM(LA#"O%:X&*9$@1 !3Z#O"BBB@ HHHH **** "BBB@ HHHH **** "BBB@
M HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "
MBBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **
M** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ JE?
MVHFA(Q5VD89% 'DGB+2C8WAE1<1N>?8UBUZMKNE)=6[JRY##FO,;VSDLKIHI
M >#\I]16,XVU.&M3Y7S(KT445!SA1110 4444 %%%% !1110 4444 %%%% !
M1110 4444 %%%% !1110 4444 4]6TV+5=.DM9<#<,H^,[&['_/;->5W-O+:
M7,D$ZE)(V*L#ZU[!7)>--%\V'^T[9/GC&)@H^\O9OPZ?3Z5<7T!G$4445J2%
M%%% !1110 4444 %=7X \*/XEUQ6F0_8;9@TI(X<]D_'O[?6N?TK3+G6-3AL
M;)-\LS8'HH[D^PKZ7\$^%H-"TF"T@7A!EVQR[=V-!O1I\[N]CI-*LA;PK@8X
MK4IJ*%4 4ZF>B%%%% !1110 4444 %%%% !1110 4444 %%07M]::;9O=:C=
M0VEM'C?-/($1<G RQX') _&LK_A-_"G_ $,^C?\ @PB_^*H%=(W**P_^$W\*
M?]#/HW_@PB_^*H_X3?PI_P!#/HW_ (,(O_BJ YEW-RBLW3O$6B:O<-!I.L:?
M?3*F]H[:Z21@N0,X4DXR1S[UI4#O<**** "BBB@ HHHH **** "BBB@ HHHH
M **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@
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
M8<=& KC;BWDMI2DJX/\ .L91:."I3</0BHHHJ3$**** "BBB@ HHHH ****
M"BBB@ HHHH **** "BBB@ HHHH **** "N)\:Z-Y4@U.W7Y7(68 =&[-^/3Z
MX]:[:H[B".ZMY()UWQR*593W%-.S \?HJ[J^FR:3J4EK)R <HW]Y3T/^>^:I
M5N2%.CC>:18XD9W<X55&23Z 58T[3;O5KU+2PA::5NP'0>I/8>]>U^!?AQ!I
M&RYN@MQ?$<R$<1^R_P"/>@UITW-E#X>_#@V+QZAJB![PC*1]1#_BWO\ _KKV
M.PL%@C'%+8V"0(.*T ,4ST8Q459 !BBBB@H**** "BBB@ HHHH **** "BBB
M@ HHHH **** .'^,O_)(]9_[8?\ H^.OE6OKCXF:-?\ B'X=ZGIFD0?:+R?R
MO+BWJF[;*C'EB!T!/6OG_P#X4UX]_P"@#_Y.0?\ Q=!QUXR<M$</17<?\*:\
M>_\ 0!_\G(/_ (NC_A37CW_H _\ DY!_\70<_LY]C=_9Z_Y*#??]@N3_ -&Q
M5]&UXK\'/ 'B;PKXPNK[7]-^R6TE@\*OY\3Y<R1D#"L3T4_E7M5!W44U#4*Y
MIM)O3\4EU<0_Z"-(-L9=Z_ZSS=VW&<].<XQ72T4=;FW2QR7CGP[>ZDEEK'AX
M :WI<@>WY"^<A/S1DG P1Z^X[U%KVE:VVJZ7XJT*SC;4K>W\F[TV>8+YL;8)
M0./E#*<\YQ_(]E11L&YR^C3>+=3UM;O6;2'1--AC*_85F2XDG<]&+@<*.P&#
MD<Y%<XGA7Q!<:%XF\-36,<%O>W,MW:ZB;E2LA:16"%!\PZ<G]*]+HH!:'FFM
M6?C;Q+X-FT0^'K72$2!0Q^V)(;C;@A(U7 3)4?>. *VX]&U1_%GAW4);7;%:
MZ8\-RQD3]W(57Y< \\@\C(KL**'K?S_R:_45M+?UT_R.;^'^DWNB>";.PU2'
MR+F)I2\>]6QF1B.5)'0BNDHHIMW=QA1112 **** "BBB@ HHHH **** "BBB
M@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "@\T44 5+JS2=""*
MX[Q!X7M[ZVD@N8%FA<?,C"N\J*6!9%P10!\L^*?AQ>Z2\EQI8>ZM@<F/&9(Q
M_P"S#Z<UQ%?7^I:(LH)5:\R\6_#6SU1WGC4VMT?^6L:\,?\ :'?Z]:#DJ8?K
M$\-HK8USPOJF@2$7L!,.<+/'RA_'M]#6/0<C33LPHHHH$%%%% !1110 4444
M %%%% !1110 4444 %%%% !1110 445?TK1-0UJX\K3K9Y3G#-C"K]3T% TF
M]$4*ZCPSX%U'Q!(DLJ-:V1Y,K#EQ_LCO]>E=YX4^%]M9LD^H@7ER.0"/W:'V
M'?ZG\J]5TS05C ++0=5/#]9&!X8\'6NE6B06< C0<D]2Q]2>YKN;*P2!!@58
M@MDB4 "IZ#K2MH@ QTHHHH&%%%% !1110 4444 %%%% !1110 4444 %%%%
M!1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %
M%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 44
M44 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !111
M0 4444 %%%% !1110 4444 03VRS*017+ZSX?29&^3/]*Z^F21!UP10&YXW?
MZ5-9,>"R>N.15"O6M0T=)E)"\UQFJ>&BC,T0V'Z<&LI0[')4H=8G,45+/;RV
M[;94*^_8U%69RM-.S"BBB@04444 %%%% !1110 4444 %%%% !1110 4444
M%%%% !113XXGE;;&I8^U ]S!\4:+_:VF[H1FY@RT?^T.Z_CC\Q7,^&?!6H^(
MY5=4-O9Y^:=U^]_NCO\ RKV+2_#K2N&G&[_9[5V>FZ''"J_( !V K:*:6ITP
MP]]9'.>$_!%EHMLL=I!M) WR-RTA'<G_ "*[JULT@48%310+&N *EJSK225D
M &****!A1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110
M 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !
M1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444 (5
M##FJ-UIT<RG*U?HH XS4_#BRHZF,,K#!4C((KS+Q#\*[&X9I+ &QF]$&8S_P
M'M^'Y5[Z\:L.15&YTR.8'Y103*,9+4^2M7\'ZSHS,;BT:6)?^6T/SK_B/Q K
M#KZTO?#P.2HKB]=^'>F:B6:XLE60_P#+6+Y&_3K^.:#FEA_Y6?/]%>BZG\)Y
MXLMIE[N]$N%Q_P"/#_"N4O?"&NV#'SM.F=1_%"/,'Z4'/*G..Z,6BG.C1N4D
M5E8=0PP13:#,**** "BBB@ HHHH ***.O2@ HK4L?#6LZCC[+IT[*>C,NQ?S
M.!74:;\*]0N-K:A=1P#ND0+M^? 'ZT%QISELC@ZT]*\.:KK+#[!9R.A/^M8;
M4'XG^E>Q:+\,M*LBK"T^T2#^.X^?].GZ5W5CX<"A<K@#H,=*#HCAW]IGE'A[
MX4P(RRZNYNI.OE)E8Q_4_I7I^D^&([>)(X84BC7HB* !^%=/:Z3'"!\M:"1*
M@X%!U1A&.Q1L],C@4?**T%0*.!3J*"@HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **YO_A8/AC_H)_\ DO+_ /$T?\+!\,?]!/\
M\EY?_B: .DHKF_\ A8/AC_H)_P#DO+_\34EMXY\.W=U%;6^H[Y9G$<:^1(,L
M3@#)7UH Z"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** $(!ZU6N+))E.15JB@#E=1\/)(IP@(],5R=_X;>)
MB805]CTKU5E#=:J3V$<H.5%)I/<F4(RW/&YK6: _O8R/?M4->HWGA]'SM6N<
MOO"XY*IM/JO%9N'8YI8?^5G(T5J3Z'<0YV_-]1BJ$EM-%]^-A^%0XM'/*G*.
MZ(J***1 4444 %%%% !1110 4444 %%2)!+)]R-C^%7(-&N9L9&T?F::39<8
M2ELC/J2*"29L1(6KI;+PON(+J6^M='9>'53&Y?TJU#N;QP[^T<=8^'I9V!EZ
M?W175Z;X<6-1\@ ^E=#;Z;'$!A15U8PHX%:));'3&$8[%2VT^.%1A:N!0O2E
MHIEA1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 45'
M<7,%I T]W-'!$N-TDKA5&3CDGWJC_P )'H?_ $&=/_\  I/\: -*BLW_ (2/
M0_\ H,Z?_P"!2?XT?\)'H?\ T&=/_P# I/\ &@#2HJI::MIU_*8K&_M;F0+N
M*0S*Y ]< ].15N@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "B
MBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ***
M* "BBB@ HHHH **** &E W45!+9QR=5%6:* ,:XT2*3.%K*N/#8.=HKKJ0J#
M0!YQ?^$H;I=MU:Q3KZ21AOYUS=Y\,M&FSG35C/K$S)C\ <5[0T"-U%0M8QM_
M"*"7%/='@EQ\)=-;/ER7D1] ZD?J*SY/A&O_ "SU&5?]Z$'^HKZ%;2XF_A%1
M-HT)_A%!'LH/H?/#?"28 [=5R>V;;'_LU-3X27!/SZHH'M;D_P#LU?0W]B0_
MW11_8D/]T4"]C3['@$7PCZ>;J4C>NV #_P!F-7H/A'8 CS;B\D]@54?RKW,:
M-"/X14JZ5"O\(H'[&"Z'C]I\+M&B()L&E/K)(Q_3.*Z/3_!EI:8^R6,$!]8X
M@I_.O0UL(E_A%3+;HO0"@M1BMD<E;^&^FX5JV^A1IC*BML(!T%.Q045(K"./
MHHJRL:KT%.HH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M **** "BBB@ HHHH ^;J*** "M+PY_R-6E?]?L/_ *&*S:TO#G_(U:5_U^P_
M^AB@#W^BBB@"O?WUOIFGSWM])Y5O;H9)7VD[5'4X')_"H-&UO3_$&FK?Z/<B
MYMF8J'"LO(.",, 16?X[_P"1 US_ *\I/_0:YKX9L-(NIM$/RQ7-C;ZI;+[.
M@63_ ,> /XT+5M?UU_1 ]$G_ %T_S.PN_$ND66O6VBW5X$U"Z7=#!Y;'<.>X
M&!T/4BM2O'03J?Q,TCQ"W*7NJS6]L>WDPQ[01]6W&NZ\*:Q?:EK_ (EMKV?S
M8K&^$5NNQ1L7;G&0.?QS1'57_KI_F$M'_7G_ )&YJ.K66DBV.H3>2+JX2VA^
M1FW2-]U> <9QU/%":M8R7%[#'<J\E@ ;E5!/E9!(S[X&<=:\RN-8OM:T/2IM
M2G\Z2+Q>D"-L5<(I.!P!5KPO::U9^,/%UQ+KOG"U8&=/L:+]H8PDHV?X=O'
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
M$^M6WB'P_P"$M3L2?*N-<M3M/5&RP*GW!R*(ZI?+\0>C?I^EST*D9@JEF(
MR2>U<)XG\4W3^*FT#3]>L/#L=M"LMS?WFQF9F^['&CD \<D_R[U]*\0:EJBZ
MOX;/B.QO;V*U^T6NKV<<;AX\X8/&#M##I@=B#]4W[MT.VMG_ %<[O3]0M=5L
M8[S3YEGMY<[)%!PV"0<9]P:B76K!]>?1EG_XF"0"X:'8P_=YQNSC!Y]ZY;X3
M6U[#X#LY;O4/M-O,F;>#R53[. S C<.6R><GI2^*$&E_$7PQK71+AGTV8D_W
MP3'_ ./9JVK2Y?Z_JY*=XW_K^K'0ZMXETC0[RRM=5O%MY[Y]ENA1FWG('4 @
M<L.3@5:U35+/1=-FU#4YQ!:P &20J3C)P. "3R1TKSGQ1IC>+];\421Y9='T
M];>U(./W^?-;'O\ *!5OQ#J8\6>%_"EC'R==N8FG4<_NXQNE_(BI6J7?3\=O
M\QO1_P!=-_\ (]#AE2>%)8CE)%#*2",@C(X-/KC-5U/7-9\73>'?#5]#I4=C
M"DUW>M )G#-]V-4/R].23_3!71-;UJSUK4/#GB&>*[N[>T-W:W\40C\^/./F
M3H&!XXX_F4VK7!)WMZ?B=BS!5+,0 !DD]JRI?%&C0Z/!JKWJ_8;B4113*C,'
M8L5 &!GJ#STKB?#Q\:>*O",>KS>*$L(Y(740QZ?%(9-N079CC!)!X Q@#O7-
MF#4;;X)Z9/+>_;(Y+V!K6V,(3R,2/E=PY;)YR>E5:SL^Z_%BOI=>?X(]PHK@
MFO\ Q3X<\3Z,NNZO;ZE::O.;>2WCM1$+5R,KL8?,P!XRW;WZ=[2Z7'UL%%%%
M !1110 4444 %%%% !1110 4444 %%%% !1110 4444 %%%% !1110 4444
M%%%% !1110 4444 %%%% !1110 4444 8?B[Q1;>$?#TNJ749F(8)%"K;3(Y
MZ#/;H3GT%>:6GQF\0VEQ:W/B'0(X]+NC^[EBADC++D?,K,2KX'88SZBO5=<\
M/Z7XDL19ZU:+=0*X=5+%2K#N"I!'YUY1\3EU=[ZUMO$%C)9>$K2X"P3V2))(
MXVX&<OQW SCKT)%):/7_ ( ]T>E>*/%UEX:\+'6W7[3&X46\:-CSF8949[#'
M.?05YO:?&;Q#:7%K<^(= CCTNZ/[N6*&2,LN1\RLQ*O@=AC/J*]*FT3P]XK\
M,V4$UM'>Z9Y:/;?,RX4#"D$$,../YUYG\3EU=[ZUMO$%C)9>$K2X"P3V2))(
MXVX&<OQW SCKT)%/:5O/^K^8EK$[SQGXS;0-(T^32(8[R\U258[,2$[#G!W'
M')'(XXZTSPCXMU'4O$&J>'O$5M:PZGI^'WV9;RI$..0&YXR/KGH,51\7^'I/
M$OAS0M1\(&*X;37CN+2)GVK+'@84$]#\J]<=ZQ9&U;PK+XB\<^(+2.PO[Y%M
M;&Q6992&. "2O!^Z#^#<#BC9N_G^EOQ#5I6\O^#^!UFE>*;W6_'VHZ9I\5N=
M(TQ D]P58N\Q_A4YP .>Q^Z?45UE>=>&]4\/_#;P[:Z?XBU-8-4O ;NY38\C
M;VQ][:#C P.>N#7H-O<0W=O'<6LJ30R*&22-@RL#T((ZT[65OZN*]W<DHHHI
M#"BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@#YNHKM
MO^%5:Y_S]:?_ -_'_P#B*/\ A56N?\_6G_\ ?Q__ (B@#B:TO#G_ "-6E?\
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
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ H
MHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ H
MHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ H
MHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ H
MHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ H
MHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ H
MHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BB
MB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH ****
M "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH
M**** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ HHHH **** "BBB@ H
=HHH **** "BBB@ HHHH **** "BBB@ HHHH _]D!
`
end"##;

        let mut result = decode_uu(contents).expect("Couldnt decode this example!").0;

        let mut file =
            std::fs::File::create("/Users/murtyjones/desktop/wow.jpg").expect("Couldn't make file");
        file.write_all(result.as_slice())
            .expect("Couldn't write example");
    }
}
