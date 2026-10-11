//! S-Force Origin
//! Ｓ－Ｆｏｒｃｅ オリジン
use super::prelude::*;

pub const ID: u32 = 99199231;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "S-Force" monster from your hand, GY, or banishment, but banish it when it leaves the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If another card(s) is banished while this card is in your GY: You can banish it; take 2 "S-Force" cards from your Deck, except "S-Force Origin", banish 1 of them, and send the other to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsLocation,1,nil,LOCATION_REMOVED) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let eg = a.eg.clone().nil();
        group::is_exists(d, &eg, &|d, x| card::is_location(d, x, LOCATION_REMOVED), 1, NO_EX)
    })));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.bantgtg)
    effect::set_target(d, e2, Some(cost_target(bantgtg)));
    // e2:SetOperation(s.bantgop)
    effect::set_operation(d, e2, Some(operation(bantgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_S_FORCE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_S_FORCE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_MZONE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp):GetFirst()
    let nv = aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp));
    let sel = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // Banish it when it leaves the field
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3300)
            effect::set_description(d, e1, 3300);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
        }
    }
}
/// `s.bantgfilter(c)`
fn bantgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_S_FORCE) and (c:IsAbleToRemove() or c:IsAbleToGrave()) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_S_FORCE as u16])
        && (card::is_able_to_remove(d, c, None, None, None) || card::is_able_to_grave(d, c, None))
        && !card::is_code(d, c, &[ID])
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsAbleToRemove,1,nil) and sg:IsExists(Card.IsAbleToGrave,1,nil)
    let r = group::is_exists(d, sg, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_able_to_grave(d, c, None), 1, NO_EX);
    (r, false)
}
/// `s.bantgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bantgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.bantgfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&bantgfilter), tp, LOCATION_DECK, 0, NO_EX);
        // return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.banfilter(c,sg)`
fn banfilter(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsAbleToRemove() and sg:IsExists(Card.IsAbleToGrave,1,c)
    card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, sg, &|d, x| card::is_able_to_grave(d, x, None), 1, c.into())
}
/// `s.bantgop(e,tp,eg,ep,ev,re,r,rp)`
fn bantgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.bantgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&bantgfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,2))
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 2)), None, None, false)
        .unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local bg=sg:FilterSelect(tp,s.banfilter,1,1,nil,sg)
    let bg = group::filter_select(d, &sg, tp, &|d, c| banfilter(d, c, &sg), 1, 1, false, NO_EX).nil();
    // if #bg>0 then
    if !bg.is_empty() {
        // Duel.Remove(bg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, bg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None);
        // Duel.SendtoGrave(sg-bg,REASON_EFFECT)
        let mut rest = sg.clone();
        group::remove_card(&mut rest, &bg);
        duel::send_to_grave(d, rest, REASON_EFFECT, None, None);
    }
}
