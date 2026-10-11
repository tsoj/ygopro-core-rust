//! Plundered Power Patron Plane - Vidolia
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 51669847;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[53589300, 68231287, 5914858],
    listed_series: &[SET_POWER_PATRON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can reveal 1 "Power Patron" monster in your Extra Deck; banish (face-down) 1 card from your hand, then add to your hand, or Special Summon, 1 monster from your Deck that mentions the revealed monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE | CATEGORY_TOHAND | CATEGORY_SEARCH | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_FZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(
        d,
        e1,
        Some(cost::reveal(
            Some(Arc::new(|d, c, e, tp| revealfilter(d, c, e, tp))),
            false,
            Some(cost::Count::N(1)),
            Some(cost::Count::N(1)),
            Some(Arc::new(|d, e, _tp, g| {
                // function(e,tp,g) e:SetLabel(g:GetFirst():GetCode()) end
                let tc = group::get_first(d, g).nil();
                let code = card::get_code(d, tc)[0];
                effect::set_label(d, e, &[code as i64]);
            })),
            Some(LOCATION_EXTRA),
        )),
    );
    effect::set_target(d, e1, Some(cost_target(thsptg)));
    effect::set_operation(d, e1, Some(operation(thspop)));
    card::register_effect(d, c, e1, false);
    // Monsters your opponent controls lose 100 ATK for each banished card
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_value(|d, _e, _c| -100 * duel::get_field_group_count(d, 0, LOCATION_REMOVED, LOCATION_REMOVED) as i64));
    card::register_effect(d, c, e2, false);
    // Your opponent cannot activate card effects in the GY while you control "Nerva ...", "Jupiter ...", and "Junora ..."
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_target_range(d, e3, 0, 1);
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| cannotactcon(d, a.e(0)))));
    effect::set_value_fn(d, e3, lua_fn(|d, a: Args| effect::get_activate_location(d, a.e(1)) == LOCATION_GRAVE as u32));
    card::register_effect(d, c, e3, false);
}
/// `s.cannotactcon(e)`
fn cannotactcon(d: &mut Duel, e: EffectId) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,53589300),tp,LOCATION_ONFIELD,0,1,nil)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,68231287),tp,LOCATION_ONFIELD,0,1,nil)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,5914858),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[53589300])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[68231287])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[5914858])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.revealfilter(c,e,tp)`
fn revealfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_POWER_PATRON) and not c:IsPublic()
    //   and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode(),Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    if !(card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && !card::is_public(d, c)) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| thspfilter(d, x, e, tp, code, mmz_chk)),
        tp,
        LOCATION_DECK,
        0,
        1,
        NO_EX,
    )
}
/// `s.thspfilter(c,e,tp,code,mmz_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32, mmz_chk: bool) -> bool {
    // return c:ListsCode(code) and c:IsMonster() and (c:IsAbleToHand() or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::lists_code(d, c, &[code])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None)
            || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_HAND,0,1,nil,nil,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, None, Some(POS_FACEDOWN as u8), None)),
            a.tp,
            LOCATION_HAND,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_HAND,0,1,1,nil,nil,POS_FACEDOWN)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| card::is_able_to_remove(d, c, None, Some(POS_FACEDOWN as u8), None)),
        a.tp,
        LOCATION_HAND,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g==0 or Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::remove(d, g.clone(), POS_FACEDOWN as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e) as u32;
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,code,mmz_chk):GetFirst()
    let sg = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, x| thspfilter(d, x, a.e, a.tp, code, mmz_chk)),
        a.tp,
        LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if not sc then return end
    let sc = match group::get_first(d, &sg) {
        Some(sc) => sc,
        None => return,
    };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // aux.ToHandOrElse(sc,tp,function() return mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //   function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end, aux.Stringid(id,2))
    let tp = a.tp;
    let e = a.e;
    aux::to_hand_or_else(
        d,
        sc,
        tp,
        Some(&|d, _c| mmz_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
        Some(&|d, _g| {
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            0
        }),
        Some(aux::stringid(ID, 2)),
    );
}
