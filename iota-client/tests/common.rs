#![allow(dead_code)]

pub const STARTING_MILESTONE_INDEX: u32 = 1050000;
pub const OLDER_TIMESTAMP: u64 = 1562581100;
pub const NULL_HASH: &str =
    "999999999999999999999999999999999999999999999999999999999999999999999999999999999";
pub const TEST_TRUNK_HASH: &str =
    "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA";
pub const TEST_ADDRESS_0: &str =
    "XUERGHWTYRTFUYKFKXURKHMFEVLOIFTTCNTXOGLDPCZ9CJLKHROOPGNAQYFJEPGK9OKUQROUECBAVNXRY";
pub const TEST_BRANCH_HASH: &str =
    "99999999IP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA";
pub const TEST_BUNDLE_HASH_0: &str =
    "MKQKKUKBRQTJEQZRSJCPOABSBEHRMDLRKFHHYYIGZPNKKCDTXHJQBORAX9KEFDBDBZDEWZFOKOCICAUBC";
pub const TEST_BUNDLE_TX_0: &str =
    "SVHIDTVSJRHLNFXIFUVYPIWBV9IZGCSMLUZCFOEQMCXMUTHRQCESOIHHKKEVXOUGGOYOSF9ATDMBFK999";
pub const TEST_BUNDLE_TX_1: &str =
    "IITL9EALLVZEGFIFBCCAHUOKHFBIIKQACBCEVVNZUEQLUJTOPXRICFRZKJDQGSVHARJANFDDAHMERS999";
pub const TEST_MILESTONE_0: &str =
    "FBOCIRYP9IVIUER9URIZVPOMYZJSOJJHVTYLYTKLOPNCRJECEVELQSBHY9ESZLJTBHUNSQHNKWLUVP999";
pub const TEST_TAG_0: &str = "CCLIENT99999999999999999999";
pub const TEST_TX_TRYTES: &str = "BCDDPCADADXCBDVCEAKDXCHDWCEARBYBACXBOBCCEAHDXCDDGDTC9DTCRCHDJ9MBCDIDBDHDDBEAUAVAVAUAABUAJ9CCXCADTCGDHDPCADDDDBEAWAUAWAUARAUAXARAUAWACCUAABDBVAZADBXAABPAUAVADBUAUAJ9VCCCCCKBEAHDCDCDZCEAVAZASAYAUAXAGDEAMASCTCDDHDWCGBXANA9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999HORNET99SPAMMER99OLIVER99VPS9XL99999999999999999999999999999999999999999999999999999999999999999999999999999HORNET99INTEGRATED999999OE9VRJXAIIPF999999999999999999HOOEWOKSZZCBVXZERQFIL9DMBYVFYELVHWLJOMRJKLXYXZRBESBDDMYQUT9HSROAKUETMD9WZCBDDCUJXOKVJQXIRRORVXPMWA9SXSNSILRZBFAJWQPKXTZKPMAJFZORVH9FDYEXABQQOORJXLMUBWLBXKZSY99999OKVJQXIRRORVXPMWA9SXSNSILRZBFAJWQPKXTZKPMAJFZORVH9FDYEXABQQOORJXLMUBWLBXKZSY99999HORNET99INTEGRATED999999OE9VRJXAIIPF999999999K99999999AXD9999999LIK99999999999999";

pub fn client_init() -> iota_client::Client {
    //iota_client::Client::new("https://nodes.iota.cafe")
    iota_client::Client::new("https://nodes.comnet.thetangle.org").unwrap()
}
