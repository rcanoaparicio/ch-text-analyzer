use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Error, Write};

fn str_to_u32(s: &str) -> Option<u32> {
    let mut r = 0;
    for c in s.chars() {
        match c.to_digit(10) {
            Some(digit) => r = (r * 10) + digit as u32,
            None => return None,
        }
    }
    Some(r)
}

fn get_ids(s: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let parts = s.split(',');
    for part in parts {
        match str_to_u32(part) {
            Some(n) => result.push(n),
            None => {}
        }
    }
    result
}

fn get_word(s: &str) -> String {
    s.split(',').next().unwrap().to_string()
}

fn get_word_from(
    text: &str,
    start: usize,
    words: &HashMap<String, Vec<u32>>,
) -> Option<(usize, String)> {
    let mut result: String = "".to_owned();
    let chars = text.chars();
    let mut i = start.clone();
    for c in chars {
        if i > 0 {
            i -= 1;
            continue;
        }
        let temp_result = result.clone() + &c.to_string();
        if words.contains_key(&temp_result) {
            result = temp_result.to_string();
        } else {
            return Some((start + result.chars().count() + 1, result));
        }
    }
    Some((start + result.chars().count() + 1, result))
}

fn get_learning_words() -> HashMap<String, u32> {
    let file = fs::File::open("./data/.learning_words").unwrap();
    let buffered = BufReader::new(file);

    let mut result: HashMap<String, u32> = HashMap::new();

    for line in buffered.lines() {
        match line {
            Err(_) => {}
            Ok(l) => {
                let parts: Vec<&str> = l.split(',').collect();
                result.insert(parts[0].to_string(), str_to_u32(parts[1]).unwrap());
            }
        }
    }

    result
}

fn save_words(words: &HashMap<String, u32>) -> Result<(), Error> {
    let threshhold: u32 = 3;
    let known: Vec<_> = words.iter().filter(|(_, n)| *n >= &threshhold).collect();
    let learning: Vec<_> = words.iter().filter(|(_, n)| *n <= &threshhold).collect();
    println!("{:?}\n{:?}", known, learning);

    let mut file = fs::File::create("./data/.known_words")?;
    let known_words: String = known
        .into_iter()
        .map(|(word, _)| word.to_owned())
        .collect::<Vec<_>>()
        .join("\n");
    write!(file, "{}", &known_words)?;

    let mut file = fs::File::create("./data/.learning_words")?;
    let learning_words: String = learning
        .into_iter()
        .map(|(word, occurrences)| format!("{},{}", word, occurrences))
        .collect::<Vec<_>>()
        .join("\n");
    write!(file, "{}", &learning_words)?;

    Ok(())
}

fn main() {
    let file = fs::File::open("./data/cedict.idx").unwrap();
    let buffered = BufReader::new(file);

    let mut result: HashMap<String, Vec<u32>> = HashMap::new();

    for line in buffered.lines() {
        match line {
            Err(_) => {}
            Ok(l) => {
                let word = get_word(&l);
                let ids = get_ids(&l);
                result.insert(word, ids);
            }
        }
    }
    println!("Ok");
    let text = "我比现在年轻十岁的时候，获得了一个游手好闲的职业，去乡间收集民间歌谣。那一年的整个夏天，我如同一只乱飞的麻雀，游荡在知了和阳光充斥的村舍田野。我喜欢喝农民那种带有苦味的茶水，他们的茶桶就放在田埂的树下，我毫无顾忌地拿起漆满茶垢的茶碗舀水喝，还把自己的水壶灌满，与田里干活的男人说上几句废话，在姑娘因我而起的窃窃私笑里扬长而去。我曾经和一位守着瓜田的老人聊了整整一个下午，这是我有生以来瓜吃得最多的一次，当我站起来告辞时，突然发现自己像个孕妇一样步履艰难了。然后我与一位当上了祖母的女人坐在门槛上，她编着草鞋为我唱了一支《十月怀胎》。我最喜欢的是傍晚来到时，坐在农民的屋前，看着他们将提上的井水泼在地上，压住蒸腾的尘土，夕阳的光芒在树梢上照射下来，拿一把他们递过来的扇子，尝尝他们和盐一样咸的咸菜，看看几个年轻女人，和男人们说着话。我头戴宽边草帽，脚上穿着拖鞋，一条毛巾挂在身后的皮带上，让它像尾巴似的拍打着我的屁股。我整日张大嘴巴打着呵欠，散漫地走在田间小道上，我的拖鞋吧哒吧哒，把那些小道弄得尘土飞扬，仿佛是车轮滚滚而过时的情景。我到处游荡，已经弄不清楚哪些村庄我曾经去过，哪些我没有去过。";
    let mut i = 0;
    let mut occurrences: HashMap<String, u32> = HashMap::new();
    while i < text.chars().count() {
        let r = get_word_from(&text, i, &result).unwrap();
        let prev_ocurr = occurrences.get(&r.1).unwrap_or(&0);
        occurrences.insert(r.1, prev_ocurr + 1);
        i = r.0;
    }
    println!("{:?}", occurrences);

    let save_result = save_words(&occurrences);
    println!("{:?}", save_result);

    let learning_words = get_learning_words();
    println!("Learnign words: {:?}", learning_words);
}
