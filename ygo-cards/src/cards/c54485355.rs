//! 昇華騎士－エクスパラディン (Sublimation Knight)
use super::prelude::*;

pub const ID: u32 = 54485355;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip 1 FIRE warrior or gemini monster from hand or deck to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval_lua), lua_fn(equip_lua), e1, None, None, None);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e2)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval_lua), lua_fn(equip_lua), e2, None, None, None);
    // Register before leaving
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    // e3:SetOperation(s.eqcheck)
    effect::set_operation(d, e3, Some(operation(eqcheck)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special summon all gemini monsters that were equipped to this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    // return ec:IsControler(tp) and ((ec:IsRace(RACE_WARRIOR) and ec:IsAttribute(ATTRIBUTE_FIRE)) or ec:IsType(TYPE_GEMINI))
    card::is_controler(d, ec, tp)
        && ((card::is_race(d, ec, RACE_WARRIOR) && card::is_attribute(d, ec, ATTRIBUTE_FIRE)) || card::is_type(d, ec, TYPE_GEMINI))
}
/// `eqval` as the Lua value function: `(ec, c, tp)`
fn eqval_lua(d: &mut Duel, a: Args) -> bool {
    eqval(d, a.c(0), a.p(2))
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsRace(RACE_WARRIOR) and c:IsAttribute(ATTRIBUTE_FIRE)) or c:IsType(TYPE_GEMINI)) and not c:IsForbidden()
    ((card::is_race(d, c, RACE_WARRIOR) && card::is_attribute(d, c, ATTRIBUTE_FIRE)) || card::is_type(d, c, TYPE_GEMINI)) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,...,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true)) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_OWNER_RELATE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
/// `s.equipop` as the Lua callback of `aux.AddEREquipLimit`: `(c,e,tp,tc)`
fn equip_lua(d: &mut Duel, a: Args) {
    equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
}
/// `s.eqop(e,tp,...)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || card::is_facedown(d, c)
        || !card::is_relate_to_effect(d, c, a.e)
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // s.equipop(c,e,tp,tc)
        equipop(d, c, a.e, a.tp, tc.nil());
    } else {
        // Duel.SendtoGrave(tc,REASON_EFFECT)
        duel::send_to_grave(d, tc.nil(), REASON_EFFECT, None, None);
    }
}
/// `s.spfilter(c)`: `c:GetOriginalType()&TYPE_GEMINI==TYPE_GEMINI`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_original_type(d, c) & TYPE_GEMINI == TYPE_GEMINI
}
/// `s.eqcheck(e,tp,...)`
fn eqcheck(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject() then e:GetLabelObject():DeleteGroup() end
    // (DeleteGroup does nothing in Rust: the old group is simply replaced below)
    // local g=e:GetHandler():GetEquipGroup():Match(s.spfilter,nil)
    let h = effect::get_handler(d, a.e).nil();
    let mut g = card::get_equip_group(d, h);
    group::match_(d, &mut g, &spfilter, NO_EX);
    // g:KeepAlive()  (nothing to do: groups are values)
    // e:SetLabelObject(g)
    effect::set_label_object(d, a.e, Value::Group(g));
}
/// The equipped Gemini group stored in `e3` (`e:GetLabelObject():GetLabelObject()`)
fn equipped_group(d: &mut Duel, e: EffectId) -> Option<CardSet> {
    let e3 = effect::get_label_object(d, e).as_effect()?;
    effect::get_label_object(d, e3).as_group().cloned()
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)` of e4
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=e:GetLabelObject():GetLabelObject()  (not used by the condition)
    let _g = equipped_group(d, a.e);
    // return (c:IsReason(REASON_BATTLE) or rp==1-tp) and c:GetPreviousControler()==tp and c:IsPreviousLocation(LOCATION_MZONE)
    (card::is_reason(d, c, REASON_BATTLE) || a.rp == 1 - a.tp)
        && card::get_previous_controler(d, c) == a.tp
        && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsType(TYPE_GEMINI)
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_type(d, c, TYPE_GEMINI)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():GetLabelObject()
    let g = equipped_group(d, a.e);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>0 and g:IsExists(s.spfilter2,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0 && group::is_exists(d, &g.clone().nil(), &|d, c| spfilter2(d, c, a.e, a.tp), 1, NO_EX);
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,ft,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, ft as u8, 0, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=e:GetLabelObject():GetLabelObject()
    let g = equipped_group(d, a.e);
    // if ft<=0 or #g==0 then return end
    if ft <= 0 {
        return;
    }
    let mut g = g.nil();
    if g.is_empty() {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if #g>ft then
    if g.len() as i64 > ft as i64 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // g=g:Select(tp,ft,ft,nil)
        g = group::select(d, &g, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // tc:EnableGeminiStatus()
            card::enable_gemini_status(d, tc);
            // tc:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,64)
            card::register_flag_effect(d, tc, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, 64);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
