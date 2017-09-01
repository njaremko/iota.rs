use curl::*;
use kerl::*;
use inner::Hash;

use super::view::*;
use super::*;

pub fn bundle_hash<'a>(transactions: &[&TransactionView<'a>], kerl: &mut Kerl) -> Hash {
    // We conciously do not validate all transaction's fields here.
    // This loop is easier than expecting the input to be sorted.
    for i in 0..transactions.len() {
        for tx in transactions {
            if tx.current_index() == i {
                kerl.absorb(tx.essence());
                break;
            }
        }
    }

    let mut hash = Hash::default();
    kerl.squeeze(&mut *hash);
    kerl.reset();

    hash
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloc::Vec;
    use trytes::*;

    const TX1: &'static str = "OKKLHSYIJ9LATZYZHZGLGMZPP9BKJXURJFJJKXXCITYJKMUAZSTIASYBHPTI\
                               DSPKFGYUZUVLZXWAMO99D9NIM9F9BUTWOCJ9XDV9MUKSDHONNNQMVOUJRWEQ\
                               NOFOMAJERHHF9KRPSL9EJLWCIEHAG9BVMPKFSGGICDOSUYLEMGXAF9EEWZHJ\
                               TPOHIBHYELNAEKRWADOYKPLUO9HVEHAWNEGLXGJUQJUIOVN9MZDEMYMOXATF\
                               YZCWC9XGJDH9BZHOUUREYSOGAEP9PUVUWCPFA9BMKUVRJPIXDKS9ZNIUVVJY\
                               N9QYKHUIAYEURLUDBJUKQLGDCJ9JAXHPVELKOPKXNIMYFAUJ9NT9UDMK9XTS\
                               IWMVDHZCLWYBXIKVEHHLYHIAEVHMPIGRBABLFORSN9TCBCK9B9DTOCXSPFE9\
                               LVHMWMXBQSWYBAHJSPBISRKUYGLUSPRTQOFILXCANPPTSEKX9DAGZAZDFCGF\
                               H9YBXDMHKSKMALYDMHSYRJQVMKNFKF9SLQIDOGDNQKZBMQXGOJRV9GBRMSYC\
                               BLLSQBKVADCOHSPYRTTGZBEFZYYNTBGUHREVRJPNQUJYSRV9GHUCAKIFPTVP\
                               YQMJAZNCGV9PCNJPVHPYNTCXEBXOMXKURTESP9EDPSJJXPDCVARMRFOZLSDB\
                               EZCE9XDTNDCFLJXFROBAJUJD9D9MHEGSTMLKHADTKOQGNVCNLJJMGONHFMGS\
                               EAEMHHCMCUZJNUXUNULQJKMF9ISNNREHHRPFKDMXADTCIGCNNDYXJR9GTROJ\
                               ECRGSUJADAPTVEHBSVCPTWXBHKW9B9LHMOLQJLCIPZEZOIHXQALXVOTYWXOC\
                               THJZWIPPUEHLOIEWCZXSBNQXYUBKODFZHMTJIIXVCUG9QRLKIFCIWRZOTOJZ\
                               BUDHZXQXTLQRESGHLGWVZSJHGMHOSKKNWTJRJJPAWKCGQZHKSVKCCVS9ATXD\
                               LGULIZVXJQVWVQOWIECVCEPAOXWXNTEMHRJJDNUHFLKGYJUONXQZNITKLFLO\
                               WKCNWPHJTEJTPOCDKUUSBSMBWDYMBCPUWULKACL9BBVCOYZABDJLZPCPKHOF\
                               IAT9QCFEOOPMXYH9YIFNIBDXWKFXVVYKKYBQCJKVAWBOKRZXASQOZCPHMWAL\
                               JBRBGUOD99NYQUGMSQNOOCYXBEJX9IIATMUGZGEHTXTAZIBTQPMIIVAJIYDW\
                               LHLUQ9HZVUMVBEDUQQICZFXZCE9K9CWNBELJGTSSNRCNUVMKLQRQMVNL9AES\
                               TEMRDHIT9YCFONDMVLKYWLHKJSHPRRZCMZNDAGLHAMZESOEUZPQGJSKOVAVT\
                               AVFLLROOGGHQJYOPBWZHAFMGPFCQRGVA99RHBVZTFXZKCLABUYVDKMOWCVKQ\
                               ZCDJOIQKLPHCRZEUIDQIGRWENWCEKIIHCIA9OSCEVYHSYXBNUDKPEWCLQBTI\
                               GS9FHUTVQPPLBRUGUWFJFKOIBPHWJIJKBPNBAICNNLBVMB9UALCWVYQQWUWJ\
                               GMUZMOGZQIBEJDDNQMEPPAPJQSBOPUANFRCHUOANMNLMJUKDPZ99FYVUWGEU\
                               9YSGMALZDEEXVAX9NBEQXWYKBYYOAMZIRABZGTMIWUUDOBGYUAPDNUVUSGMZ\
                               TWHKYHEQ9AHMNEQITUMVXFYJKOTXFLXPPCGXJSSSDJWXZKWYOXKSQQWAID9R\
                               XTJYDSMBEL9NYWFYLQTPWNWEHDXWA9DH9NLUGKAMOPSOPKIVNBMRSRJTRMKP\
                               NAEAXGRZEYNWTIZSZ9OEWBPE99KACCKMESZUM9TUABX99WRJYTRFVOQOU9SF\
                               QKRWXWVNQZIRAZXPRBNSEOJKDMTHOZGOHRVKW9LKGGYYKZHBVZURKGTLOCUQ\
                               AFWOC9VPLUGCSMQZPMBZHCDFTPPTUHCLLZXZX9JXNOEIPXYXLEMCC9PWSNAI\
                               GIFIP9IGLHYBQCHTBDUCBIJYVZRMORRXXZPRRZVUFJNKISVIJWNRM9HSFVIG\
                               ZZNVNCMVKFAPZHSTGBVFVWYZQQGZUBMXKEBRMMTCLGTODASTCTHBUAEMHENO\
                               ARJGNN9DWWZRZFWMEJIE9QLSOCKTXIIMGQHPH9PTODMPTVQHJLGCRYLGKFFF\
                               CCULE9CYCTJAYAAINTDONEWXMHZTUBHRHPHDHZPOGMILKXLE9VKTNGGDHUSY\
                               RJNEDGZRQNXBOQXQMS9JSOUMXLYEBAFXTHWJYJHICYXHGTIQNYIWI9WPGVHL\
                               FSCSNQCZJUQRGPNQQYOQEONJDXWMIFBZCEFDKYPSRXRJXJBBFMBTKZY99999\
                               999999999999999999999999999999999999999999AIUPWXD99A99999999\
                               C99999999TRAUXXJWPWVGJSCXOXMVLNGDQVTFQCINPZER9LILTIWAIIUVSB9\
                               FLBCMGZAHEHX9IUVNZNOGWCEGDLAWWG9TQZEHEJHATTALPUHJTKKFEBFWRCR\
                               YPETWMANGDUUXCGYOSQIVOSLUYZAXKFIZZEBPJJLCFIUEZ99999NZASGZEAB\
                               JYTIYWWNLXA9RVMOHQNYEEPTUYKU9QH9GUZELEAKBEFNZ9OEUVTBBKSVWXFY\
                               JEPYIGT99999HIDMRBXPMNFRAHJZFIXJKDMYIFBIAVQ9FYRQRLTQWCDJCXNY\
                               JPXQPXBENMSIUOOWKWKXIGATE9BGBJSBF";

    const TX2: &'static str = "KDGCCXLSCLLISKAROJNOFKYZGHXLJCUHWBTAKQQMSZWPWJQQEOZQZ9XXEUZW\
                               KIB9CNH9EGLUMO9YFPICY9QLYWTPQXWZZHQZVTDZPBGXUYZTJQPGDACANP9M\
                               TTYUZOGYFNRI9NBYUHFNUYYVIPBZGDP9RHDXJEFZHZDUOVOXPGPCRDMIEIBV\
                               PIGSSMFVNSQTBHTRYKFPAIBBPTMBTGYYLGCESLVLDAZEFXPSSQAKHINILJGQ\
                               EGDXPCZORMEDI9TGLVSJER9YJGKQLCDBVYYHXUBYDTUCOFNVHTTGQMOLOITK\
                               ED9OXQFHYQERFLIZ9ZEGZWKDRWYERQYZPTGQJPQKXWBGJYTIDFEIAPGTWGXP\
                               CQYTKLBS9DU9PTUTTDZKHXOWMR9XWIGDULRASQTUABFVWYZGZTFTWOEMJUVN\
                               WEDDHUBZBSKEUH9YYESNGSOMAVVOHTKTXJJXB9GJLYYB9ZLFRFQAIFTRQRHS\
                               UZUOXWOXIZSOBWTXYJFCNCDSLZKCOLGGZBCRD9E9YN9RSVQBOIF9CAESDWKG\
                               DLQCEXSPFQASFTEWVTUKUFRPQCWG9BERWOZ9PQLLIYWVRD9FL9CFHFQYOFVE\
                               UEMUYULIPPSZDKMCFEJQSPRNUINMXFVQDOKIY9TZVBHWVFTDNAQTDWXRGYYL\
                               XLUSRMDMTINCGHULGTKZCZYIQYWOYGLEGEYIPZCOFGKGTUVAX9NHIVMGOXTZ\
                               CG9WFQOGDBIKDQBKICSLTVUJWSDIMNIFZURDEFVVNNVDPWFJMCIPVZGCIFHD\
                               9YYZ9BCHMMBFTQODNXRCLEBFGXLC9AZHHYSSNZIWISXQGPBKXXWMEKMOYPWM\
                               9DFTKJHTSWYWO9IOUNIEMPGVYNPJGFTPZFQRXNCJ9ZYXMGYMJLDEYYJOXTUM\
                               MGVATZHJNXQKCSKIUYIKOJMZYACFNBIIXJLQWJTUSQXJEZUBPUDUJMUVZHWP\
                               ADAZCLOBYFODDSIBSUXPHMZMFSZUAQNKHIJCUXQXIE9NCFKIEZQXSKXYELGY\
                               QBYVXKILNZKZVHHPCJTUQZDYDWIXIQF9WKLWOQOYVPXNEEZYRIGIDJXTWDFA\
                               QHDEKLITRCUKGYFY9NKNKJVROQZRGCUUTM9PLYQZSA9HLOD9CHDGCXVMFH9I\
                               HDSLPRR9PQPRGBECNBLKVF9HAVVQICZYDJOPPEYZD9ASXVFWW9NCGALNOXJG\
                               EFSEPXXZTHQASXXVDQQZADZZZDONDHXPNMPOTASQUATN9AQPSJWWVLOHVAVX\
                               INYHCATYFF99GOVJV9ILSLYGFHJBIQZWPBDCAHVCGQZQVYVRYWOFZZUQLMYQ\
                               SBGBLELSJY9GJUUPREBOTSWYAEJAPDROWIAADXEQ9ZJBUZGNADXEWUJKBESM\
                               XNOZWIVUBSDHLSEFIRGAMJVOMXDMQVABWPAPBQJLRWRJCL9JYHDLMZJSBASC\
                               AAGBNOUYPSDDGIJZUWGLJJNVBGFZVVSZBRNMAKHDXBYGNTQIRK9UFYTUXPZ9\
                               9MJTWIMWJYYJDVWLAJZKMOSHGUCVBDTPNTVRKV9OPUJPGTUNGJUNHZLUTSZJ\
                               UDBUQXECAGEHRITVQBRKCJEZOWC9QATWBKYLTZZGQIOVSPW9YGAGOLUBY9UX\
                               IRMKWKJDVIEHYDKUYXTAKKTVBBQLEWODGBJTGBHJHZBLOIGPZCSILETKHCEJ\
                               BVLGMBDRQSYAAESSDNLPXAJFSATCWAKRQYZJIZIQYSHIJUSPXXHLOVMWHIBN\
                               LQCWFZTJPOFOWWXIJYNOYCYFO9M9RXTQVNT9FISDHCIANYNMVJQLYFAMSJZI\
                               WNKXULYSTBABWBDBREIOGSD9IYWWQEONBQUWBDIRAUNTGXVUCWWKKKYXELIW\
                               LOXIYTMCRMJWOW9QHCEFE9XSHHBYWPZQDHSDEQVDQEYLRDJPO9OJEKPQZVJV\
                               TKPFYC9OXKHYUJBCZZPULCKWDMCPTENOKXLYUBOGDESDTRXHBEERIMIOQODC\
                               AJYWPNQJFCAVVGTKRGISCFUFLD99RGARGLFXBIVLLTOIDIUWXQXO9ZSFVMML\
                               LWFWSP9PJPRG9SQPJDQZJBDJQ9BTPETMHWIXAPJYOZFBBBZPZLH9ZHNS9GQL\
                               TLTIWBZBNQARAHJHGLUHIIJSFCCWRZYQLRVSAV9PEKHCKZZSQ9UQCXZL9KRO\
                               TSTDNCOBSF9D9SVRYCAUXJZZLJZEBAFXTHWJYJHICYXHGTIQNYIWI9WPGVHL\
                               FSCSNQCZJUQRGPNQQYOQEONJDXWMIFBZCEFDKYPSRXRJXJBB999999999999\
                               999999999999999999999999999999999999999999AIUPWXD99B99999999\
                               C99999999TRAUXXJWPWVGJSCXOXMVLNGDQVTFQCINPZER9LILTIWAIIUVSB9\
                               FLBCMGZAHEHX9IUVNZNOGWCEGDLAWWNEOGZKXSCSSKMXZVANGPFZWOGTX9UR\
                               TYFOAFYLTICRGAZRKMPTHPRIEBEAYTCGSTILCJIWIVRFYG99999NZASGZEAB\
                               JYTIYWWNLXA9RVMOHQNYEEPTUYKU9QH9GUZELEAKBEFNZ9OEUVTBBKSVWXFY\
                               JEPYIGT99999JXYGVXWYHJFYREEQDFCPFDPQQMFFHXERWFECTICPVWXQKSQT\
                               LKXB9RHJFNGICIFDUJQHDK9QHHRRJCTSR";

    const TX3: &'static str = "999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999BMFRKDPEMIMAUPTKVFIBQHKW9UELRFESE\
                               GTBSSYWEY9ECAFA9QLSTVFKUXQBFEBXFNLZXETMIJFXPNIG9ZTBD99999999\
                               999999999999999999999999999999999999999999DIUPWXD99C99999999\
                               C99999999TRAUXXJWPWVGJSCXOXMVLNGDQVTFQCINPZER9LILTIWAIIUVSB9\
                               FLBCMGZAHEHX9IUVNZNOGWCEGDLAWWNZASGZEABJYTIYWWNLXA9RVMOHQNYE\
                               EPTUYKU9QH9GUZELEAKBEFNZ9OEUVTBBKSVWXFYJEPYIGT99999GYFWWVTRS\
                               SFENOZPBRWKPANYMZCSTVBJ9X99LTOUWFC9QZTWHDFBAVEXPRYBXTOOFX9QK\
                               SS9E9DW99999VLROCMPOKODRUFRVOBBURIISGTFBBCRZSCPDXZW9MHSQIMZB\
                               CDKWQRURCFECYHDDCRFDTLV9UXGMQWDCW";

    const TX0: &'static str = "999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999BIJWVSGMBFUVARHRZWMFBVQVEPIMSWYEY\
                               PHYDHHOVGSUGQWEKOLH9NKTHAHSBRPWOSNSKNTXQMMOCNQRWVUWCPAB99999\
                               999999999999999999999999999999999999999999AIUPWXD99999999999\
                               C99999999TRAUXXJWPWVGJSCXOXMVLNGDQVTFQCINPZER9LILTIWAIIUVSB9\
                               FLBCMGZAHEHX9IUVNZNOGWCEGDLAWWYDRAAROZUQDYUGQCUSDIBNZUWSFFLO\
                               I99XG9TZSJVWZGXWEAIQFLTADQRIF9XIQTZFLEUTYNRHEY99999NZASGZEAB\
                               JYTIYWWNLXA9RVMOHQNYEEPTUYKU9QH9GUZELEAKBEFNZ9OEUVTBBKSVWXFY\
                               JEPYIGT99999E9ANJZJVNBJFPFORJKKTFNQYNUFCSTFZVEHYMSIS9IVOJM9A\
                               GIBCNYRNJLWXINEWY9QFVIGIRVHALA9MJ";

    const STX: &'static str = "SPAMSPAMSPAM999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999999999999999999999999999999999999999999999999\
                               999999999999999LIQIO9999999999999999999999AKUPWXD99999999999\
                               999999999IOCXGQCUWVNWXVZIZWDJCWRHFQAMLIEGEWKNRBRKBW9JLFVM9YE\
                               BXMYOVVMCEPNJNZMFURFPFY9CXALMDHAGSWZJIPJMEWLNOSFFPEXANVGULYD\
                               IMSSMBEFFRNEIBJPXSXHWMYSIJRCDDVGKNXMJNCUTNHVVL99999QDFXDSXXH\
                               TOPSUZUKPKJP9GBRS9FAGKF9EUEUEQZO9UPTNBTHIWAQGFBKRZXLFECJ9LDJ\
                               BIYCSFH99999CILBILMGFGTQVRHWMTBBBPGTCKXNMMANSY9ZLI9AVMKOODGU\
                               CYCPNMIAMUPUUFEMASQQ9WSCBBPYZRXOC";

    #[test]
    fn hash_single_tx_bundle() {
        let stx: Vec<Trit> = STX.chars().flat_map(char_to_trits).cloned().collect();
        let stxv = TransactionView::from_trits(&stx).unwrap();
        let txvs = [&stxv];

        let mut kerl = Kerl::default();
        let hash = bundle_hash(&txvs, &mut kerl);
        let expected = "IOCXGQCUWVNWXVZIZWDJCWRHFQAMLIEGEWKNRBRKBW9JLFVM9YEBXMYOVVMCEPNJNZMFURFPFY9CXALMD";
        assert_eq!(trits_to_string(&*hash).unwrap(), expected);
    }

    #[test]
    fn hash_multi_tx_bundle() {
        let tx0: Vec<Trit> = TX0.chars().flat_map(char_to_trits).cloned().collect();
        let tx1: Vec<Trit> = TX1.chars().flat_map(char_to_trits).cloned().collect();
        let tx2: Vec<Trit> = TX2.chars().flat_map(char_to_trits).cloned().collect();
        let tx3: Vec<Trit> = TX3.chars().flat_map(char_to_trits).cloned().collect();


        let tx0v = TransactionView::from_trits(&tx0).unwrap();
        let tx1v = TransactionView::from_trits(&tx1).unwrap();
        let tx2v = TransactionView::from_trits(&tx2).unwrap();
        let tx3v = TransactionView::from_trits(&tx3).unwrap();

        let txvs = [&tx1v, &tx2v, &tx3v, &tx0v];

        let mut curl = Kerl::default();

        let hash = bundle_hash(&txvs, &mut curl);

        let expected = "TRAUXXJWPWVGJSCXOXMVLNGDQVTFQCINPZER9LILTIWAIIUVSB9FLBCMGZAHEHX9IUVNZNOGWCEGDLAWW";
        assert_eq!(trits_to_string(&*hash).unwrap(), expected);
    }
}
