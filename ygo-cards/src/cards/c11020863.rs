//! Royal Straight Slasher
use super::prelude::*;

pub const ID: u32 = 11020863;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[58415502, CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be Special Summoned by "Royal Straight"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy all opponent cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // Special Summon up to 3 LIGHT Warrior monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_JACK_KNIGHT)
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_QUEEN_KNIGHT)
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_KING_KNIGHT)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_JACK_KNIGHT])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_QUEEN_KNIGHT])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_KING_KNIGHT])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsLevelAbove(1) and c:IsLevelBelow(5) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_level_above(d, c, 1) && card::is_level_below(d, c, 5) && card::is_able_to_grave(d, c, None)
}
/// `s.tgrescon(sg)`: `res,not res` where `res` is "all levels differ"
fn tgrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local res=#sg==sg:GetClassCount(Card.GetLevel)
    let res = sg.len() == group::get_class_count(d, sg, &|d, c| card::get_level(d, c) as i64);
    // return res,not res
    (res, !res)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // if #dg<1 then return false end
        if dg.is_empty() {
            return false;
        }
        // local tg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
        let tg = duel::get_matching_group(d, Some(&tgfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // return #tg>4 and aux.SelectUnselectGroup(tg,e,tp,5,5,s.tgrescon,0)
        let dn = &tgrescon;
        return tg.len() > 4 && matches!(aux::select_unselect_group(d, &tg, Some(a.e), a.tp, Some(5), Some(5), Some(&dn), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,5,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 5, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if #g<5 then return end
    if g.len() < 5 {
        return;
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,5,5,s.tgrescon,1,tp,HINTMSG_TOGRAVE)
    let dn = &tgrescon;
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(5), Some(5), Some(&dn), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #tg==5 and Duel.SendtoGrave(tg,REASON_EFFECT)>0 then
    if tg.len() == 5 && duel::send_to_grave(d, tg, REASON_EFFECT, None, None) > 0 {
        // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
        let dg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #dg<1 then return end
        if dg.is_empty() {
            return;
        }
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_WARRIOR)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_WARRIOR) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // local ft=math.min(3,Duel.GetLocationCount(tp,LOCATION_MZONE))
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0.min(3);
    // if chk==0 then return ft>0 and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,ft,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, ft as u16, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,0,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, n, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #g>0 and (#g<2 or not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then
    if !g.is_empty() && (g.len() < 2 || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()) {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
