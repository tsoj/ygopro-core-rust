//! Winged Kuriboh LV9
use super::prelude::*;

pub const ID: u32 = 33776734;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], lv: Some((9, SET_WINGED_KURIBOH as u32)), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e2, LOCATION_SZONE, LOCATION_SZONE);
    // e2:SetTarget(s.rmtarget)
    effect::set_target(d, e2, Some(effect_card_fn(rmtarget)));
    // e2:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk,def
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e3, EFFECT_SET_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.val)
    effect::set_value_fn(d, e3, effect_card_value(val));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_SET_DEFENSE)
    effect::set_code(d, e4, EFFECT_SET_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // ge1:SetCode(EVENT_CHAINING)
        effect::set_code(d, ge1, EVENT_CHAINING);
        // ge1:SetOperation(s.checkop1)
        effect::set_operation(d, ge1, Some(operation(checkop1)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_CHAIN_NEGATED)
        effect::set_code(d, ge2, EVENT_CHAIN_NEGATED);
        // ge2:SetOperation(s.checkop2)
        effect::set_operation(d, ge2, Some(operation(checkop2)));
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
}
/// `s.checkop1(e,tp,eg,ep,ev,re,r,rp)`
fn checkop1(d: &mut Duel, a: &EffArgs) {
    // if re and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() then
    if let Some(re) = a.re {
        if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) {
            // re:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            let h = effect::get_handler(d, re).nil();
            card::register_flag_effect(d, h, ID, RESET_EVENT + RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.checkop2(e,tp,eg,ep,ev,re,r,rp)`
fn checkop2(d: &mut Duel, a: &EffArgs) {
    // if re and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() then
    if let Some(re) = a.re {
        if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) {
            // re:GetHandler():ResetFlagEffect(id)
            let h = effect::get_handler(d, re).nil();
            card::reset_flag_effect(d, h, ID);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)>=2
    duel::get_current_chain(d, true) >= 2
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,0)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.rmtarget(e,c)`
fn rmtarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetFlagEffect(id)>0 and Duel.IsPlayerCanRemove(e:GetHandlerPlayer(),c)
    if !(card::is_faceup(d, c) && card::get_flag_effect(d, c, ID) > 0) {
        return false;
    }
    let p = effect::get_handler_player(d, e);
    duel::is_player_can_remove(d, p, Some((c, None)))
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsSpell,c:GetControler(),0,LOCATION_GRAVE,nil)*500
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_spell(d, x)), p, 0, LOCATION_GRAVE, NO_EX) as i64 * 500
}
