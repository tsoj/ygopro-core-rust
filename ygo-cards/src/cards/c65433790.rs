//! Battle Royal Mode - Joining
use super::prelude::*;

pub const ID: u32 = 65433790;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate by targeting 1 monster
    // aux.AddPersistentProcedure(c,PLAYER_ALL,aux.FaceupFilter(Card.IsType,TYPE_EFFECT))
    aux::add_persistent_procedure(
        d,
        c,
        Some(PLAYER_ALL),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT))),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    );
    // The first two times that monster would be destroyed by battle each turn, it is not destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetCountLimit(2)
    effect::set_count_limit(d, e1, 2, 0, 0, 0);
    // e1:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    // e1:SetValue(function(_,_,r) return r&REASON_BATTLE==REASON_BATTLE end)
    effect::set_value_fn(d, e1, lua_fn(|_d, a: Args| (a.int(2) & REASON_BATTLE as i64) == REASON_BATTLE as i64));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When it is destroyed by battle, the player that destroyed it gains 2000 LP
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetOperation(s.lpop)
    effect::set_operation(d, e2, Some(operation(lpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Level 4 or lower monster from your hand or Deck, but lose 2000 LP
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e3, EVENT_DAMAGE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(function(_,_,_,ep) return Duel.IsTurnPlayer(1-ep) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.ep))));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h);
    // if tc and eg:IsContains(tc) and tc:IsReason(REASON_BATTLE) then
    if let Some(tc) = tc {
        if group::is_contains(&a.eg(), tc) && card::is_reason(d, tc, REASON_BATTLE) {
            // Duel.Recover(tc:GetReasonPlayer(),2000,REASON_EFFECT)
            let p = card::get_reason_player(d, tc);
            duel::recover(d, p, 2000, REASON_EFFECT, false, None);
        }
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let ep = a.ep;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e)
    //   and Duel.GetLocationCount(ep,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,ep,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,ep)
    //   and Duel.SelectYesNo(ep,aux.Stringid(id,1))) then return end
    if !(card::is_relate_to_effect(d, c, a.e)
        && duel::get_location_count(d, ep, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, a.e, ep)), ep, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, ep, aux::stringid(ID, 1)))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,ep,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, ep, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(ep,s.spfilter,ep,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,ep)
    let sg = duel::select_matching_card(d, ep, Some(&|d, cc| spfilter(d, cc, a.e, ep)), ep, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #sg>0 and Duel.SpecialSummon(sg,0,ep,ep,false,false,POS_FACEUP)>0 then
    if !sg.is_empty() && duel::special_summon(d, sg, 0, ep, ep, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.SetLP(ep,Duel.GetLP(ep)-2000)
        let lp = duel::get_lp(d, ep);
        duel::set_lp(d, ep, lp - 2000);
    }
}
