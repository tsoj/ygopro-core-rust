//! Ice Mirror
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 69492187;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
//Check for a level 3 or lower WATER monster
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(3) and c:IsAttribute(ATTRIBUTE_WATER)
    //   and Duel.GetFirstMatchingCard(s.filter2,tp,LOCATION_DECK,0,nil,c:GetCode(),e,tp)
    card::is_faceup(d, c)
        && card::is_level_below(d, c, 3)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && {
            let code = card::get_code(d, c)[0];
            duel::get_first_matching_card(d, Some(&|d, x| filter2(d, x, code, e, tp)), tp, LOCATION_DECK, 0, NO_EX).is_some()
        }
}
/// `s.filter2(c,code,e,tp)`
fn filter2(d: &mut Duel, c: CardId, code: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    // (ygoai override lua_fixes/c69492187.lua: the original calls s.filter(chkc) without e and tp, so
    // Duel.GetFirstMatchingCard gets a nil player and the core raises; the fix passes e,tp)
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp)end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,tc:GetCode(),e,tp)
        let code = card::get_code(d, tc)[0];
        let sg = duel::select_matching_card(d, tp, Some(&|d, x| filter2(d, x, code, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sg and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetAbsoluteRange(tp,1,0)
            effect::set_absolute_range(d, e1, tp, 1, 0);
            // e1:SetCondition(s.splimcon)
            effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| splimcon(d, a.e(0)))));
            // e1:SetTarget(s.splimit)
            effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| splimit(d, c))));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sg:GetFirst():RegisterEffect(e1,true)
            let first = group::get_first(d, &sg).nil();
            card::register_effect(d, first, e1, true);
        }
    }
}
/// `s.splimcon(e)`
fn splimcon(d: &mut Duel, e: EffectId) -> bool {
    // return e:GetHandler():IsControler(e:GetOwnerPlayer())
    let c = effect::get_handler(d, e).nil();
    let p = effect::get_owner_player(d, e);
    card::is_controler(d, c, p)
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA)
    card::is_location(d, c, LOCATION_EXTRA)
}
