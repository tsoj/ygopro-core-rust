//! Bayonet Punisher
use super::prelude::*;

pub const ID: u32 = 36092504;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BORREL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BORREL) and c:IsFaceup() and c:IsMonster() and c:IsType(TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ+TYPE_LINK)
    card::is_set_card(d, c, &[SET_BORREL as u16]) && card::is_faceup(d, c) && card::is_monster(d, c) && card::is_type(d, c, TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ + TYPE_LINK)
}
/// `s.exfilter(c)`
fn exfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA) and c:IsFacedown()
    card::is_location(d, c, LOCATION_EXTRA) && card::is_facedown(d, c)
}
/// `s.stfilter(c)`
fn stfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsOnField()
    card::is_spell_trap(d, c) && card::is_on_field(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_EXTRA,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_EXTRA, NO_EX);
    // local bf=g:IsExists(Card.IsType,1,nil,TYPE_FUSION) and rg:IsExists(Card.IsLocation,1,nil,LOCATION_MZONE)
    let bf = group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_FUSION), 1, NO_EX) && group::is_exists(d, &rg, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, NO_EX);
    // local bs=g:IsExists(Card.IsType,1,nil,TYPE_SYNCHRO) and rg:IsExists(s.exfilter,3,nil)
    let bs = group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_SYNCHRO), 1, NO_EX) && group::is_exists(d, &rg, &exfilter, 3, NO_EX);
    // local bx=g:IsExists(Card.IsType,1,nil,TYPE_XYZ) and rg:IsExists(s.stfilter,1,nil)
    let bx = group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_XYZ), 1, NO_EX) && group::is_exists(d, &rg, &stfilter, 1, NO_EX);
    // local bl=g:IsExists(Card.IsType,1,nil,TYPE_LINK) and rg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE)
    let bl = group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_LINK), 1, NO_EX) && group::is_exists(d, &rg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX);
    // if chk==0 then return bf or bs or bx or bl end
    if a.chk == Some(0) {
        return bf || bs || bx || bl;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackAbove,3000),tp,LOCATION_MZONE,0,1,nil) then
    let faceup_atk = aux::faceup_filter(|d, c| card::is_attack_above(d, c, 3000));
    if duel::is_existing_matching_card(d, Some(&faceup_atk), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // Duel.SetChainLimit(s.chlimit)
        duel::set_chain_limit(d, lua_fn(chlimit));
    }
    true
}
/// `s.chlimit(e,ep,tp)`
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    // return tp==ep
    a.p(2) == a.p(1)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_EXTRA,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_EXTRA, NO_EX);
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // local break_chk=0
    let mut break_chk: i32 = 0;
    // if g:IsExists(Card.IsType,1,nil,TYPE_FUSION) and rg:IsExists(Card.IsLocation,1,nil,LOCATION_MZONE) then
    if group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_FUSION), 1, NO_EX) && group::is_exists(d, &rg, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, NO_EX) {
        // Fusion: Banish 1 monster the opponent controls
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // og=rg:FilterSelect(tp,Card.IsLocation,1,1,nil,LOCATION_MZONE)
        let og = group::filter_select(d, &rg, tp, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(og,true)
        duel::hint_selection(d, og.clone(), true);
        // break_chk=Duel.Remove(og,POS_FACEUP,REASON_EFFECT)
        break_chk = duel::remove(d, og, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // if g:IsExists(Card.IsType,1,nil,TYPE_SYNCHRO) and rg:IsExists(s.exfilter,3,nil) then
    if group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_SYNCHRO), 1, NO_EX) && group::is_exists(d, &rg, &exfilter, 3, NO_EX) {
        // Synchro: Banish 3 random face-down cards from the opponent's Extra Deck
        // if break_chk>0 then Duel.BreakEffect() end
        if break_chk > 0 {
            duel::break_effect(d);
        }
        // og=rg:Filter(s.exfilter,nil):RandomSelect(tp,3)
        let filtered = group::filter(d, &rg, &exfilter, NO_EX);
        let og = group::random_select(d, &filtered, tp, 3);
        // Duel.HintSelection(og,true)
        duel::hint_selection(d, og.clone(), true);
        // break_chk=Duel.Remove(og,POS_FACEUP,REASON_EFFECT)
        break_chk = duel::remove(d, og, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // if g:IsExists(Card.IsType,1,nil,TYPE_XYZ) and rg:IsExists(s.stfilter,1,nil) then
    if group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_XYZ), 1, NO_EX) && group::is_exists(d, &rg, &stfilter, 1, NO_EX) {
        // Xyz: Banish 1 Spell/Trap the opponent controls
        // if break_chk>0 then Duel.BreakEffect() end
        if break_chk > 0 {
            duel::break_effect(d);
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // og=rg:FilterSelect(tp,s.stfilter,1,1,nil)
        let og = group::filter_select(d, &rg, tp, &stfilter, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(og,true)
        duel::hint_selection(d, og.clone(), true);
        // break_chk=Duel.Remove(og,POS_FACEUP,REASON_EFFECT)
        break_chk = duel::remove(d, og, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // if g:IsExists(Card.IsType,1,nil,TYPE_LINK) and rg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE) then
    if group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_LINK), 1, NO_EX) && group::is_exists(d, &rg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX) {
        // Link: Banish up to 3 cards in the opponent's GY
        // if break_chk>0 then Duel.BreakEffect() end
        if break_chk > 0 {
            duel::break_effect(d);
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // og=rg:FilterSelect(tp,Card.IsLocation,1,3,nil,LOCATION_GRAVE)
        let og = group::filter_select(d, &rg, tp, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, 3, false, NO_EX).nil();
        // Duel.HintSelection(og,true)
        duel::hint_selection(d, og.clone(), true);
        // Duel.Remove(og,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, og, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
