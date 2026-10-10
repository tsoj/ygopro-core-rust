//! Super Soldier Synthesis
use super::prelude::*;

pub const ID: u32 = 45948430;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACK_LUSTER_SOLDIER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,filter=aux.FilterBoolFunction(Card.IsSetCard,SET_BLACK_LUSTER_SOLDIER),lv=8,extrafil=s.extragroup,
    //   extraop=s.extraop,matfilter=s.matfilter,location=LOCATION_HAND|LOCATION_GRAVE,forcedselection=s.ritcheck,specificmatfilter=s.specificfilter,extratg=s.extratg})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            filter: Some(std::sync::Arc::new(|d, c| card::is_set_card(d, c, &[SET_BLACK_LUSTER_SOLDIER as u16]))),
            lv: Some(ritual::Lv::N(8)),
            extrafil: Some(std::sync::Arc::new(extragroup)),
            extraop: Some(std::sync::Arc::new(extraop)),
            matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d, c, _e, _tp| matfilter1(d, c)))),
            location: Some(LOCATION_HAND | LOCATION_GRAVE),
            forcedselection: Some(std::sync::Arc::new(ritcheck)),
            specificmatfilter: Some(std::sync::Arc::new(specificfilter)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DECKDES);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.extragroup(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extragroup(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // return Duel.GetMatchingGroup(s.matfilter1,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    Some(duel::get_matching_group(d, Some(&matfilter1), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX))
}
/// `s.extraop(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn extraop(d: &mut Duel, mat: &CardSet, _a: &EffArgs, _tc: CardId) {
    // Duel.SendtoGrave(mat,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)
    duel::send_to_grave(d, mat, REASON_EFFECT + REASON_MATERIAL + REASON_RITUAL, None, None);
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.specificfilter(c,rc,mg,tp)`: `return mg:IsExists(s.matfilter2,1,nil,c,tp,rc)`
fn specificfilter(d: &mut Duel, c: CardId, rc: CardId, mg: &CardSet, tp: u8) -> bool {
    group::is_exists(d, mg, &|d, x| matfilter2(d, x, c, tp, rc), 1, NO_EX)
}
/// `s.ritcheck(e,tp,g,sc)`: `return #g==2 and ..., #g>2`
fn ritcheck(d: &mut Duel, _e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    let ok = g.len() == 2
        && group::is_exists(d, g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT), 1, NO_EX)
        && group::is_exists(d, g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), 1, NO_EX)
        && group::is_exists(d, g, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX)
        && group::is_exists(d, g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX);
    (ok, g.len() > 2)
}
/// `s.matfilter2(c,gc,tp,rc)`
fn matfilter2(d: &mut Duel, c: CardId, gc: CardId, tp: u8, rc: CardId) -> bool {
    let _ = tp;
    if (card::get_attribute(d, c) | card::get_attribute(d, gc)) & (ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) == (ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && (card::get_location(d, c) | card::get_location(d, gc)) & (LOCATION_HAND | LOCATION_DECK) == (LOCATION_HAND | LOCATION_DECK)
    {
        // return Group.FromCards(c,gc):CheckWithSumEqual(Card.GetRitualLevel,8,2,2,rc)
        let g = CardSet::from_iter([c, gc]);
        return group::check_with_sum_equal(d, &g, &|d, x| card::get_ritual_level(d, x, rc) as i64, 8, 2, 2).0;
    }
    false
}
/// `s.matfilter1(c)`: `return c:IsAttribute(ATTRIBUTE_LIGHT+ATTRIBUTE_DARK) and c:IsLocation(LOCATION_HAND|LOCATION_DECK) and c:IsAbleToGrave()`
fn matfilter1(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT + ATTRIBUTE_DARK) && card::is_location(d, c, LOCATION_HAND | LOCATION_DECK) && card::is_able_to_grave(d, c, None)
}
