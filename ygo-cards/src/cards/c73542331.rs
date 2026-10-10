//! Kashtira Shangri-Ira
use super::prelude::*;

pub const ID: u32 = 73542331;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KASHTIRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 7 monsters
    // Xyz.AddProcedure(c,nil,7,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, Some(xyz::MaxCt::Infinite), None, false, None);
    // Special Summon 1 "Kashtira" monster from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make 1 unused Main Monster Zone or Spell & Trap Zone unusable while this monster is face-up on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.zcon)
    effect::set_condition(d, e2, Some(condition(zcon)));
    // e2:SetTarget(s.ztg)
    effect::set_target(d, e2, Some(cost_target(ztg)));
    // e2:SetOperation(s.zop)
    effect::set_operation(d, e2, Some(operation(zop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card on the field would be destroyed by battle or card effect, you can detach 1 material from this card instead
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_KASHTIRA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_KASHTIRA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.zfilter(c,tp)`
fn zfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFacedown() and c:IsControler(1-tp) and c:IsPreviousControler(1-tp)
    card::is_facedown(d, c) && card::is_controler(d, c, 1 - tp) && card::is_previous_controler(d, c, 1 - tp)
}
/// `s.zcon(e,tp,eg,ep,ev,re,r,rp)`
fn zcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.zfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| zfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.ztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE,PLAYER_NONE,0)
    //     +Duel.GetLocationCount(tp,LOCATION_SZONE,PLAYER_NONE,0)
    //     +Duel.GetLocationCount(1-tp,LOCATION_MZONE,PLAYER_NONE,0)
    //     +Duel.GetLocationCount(1-tp,LOCATION_SZONE,PLAYER_NONE,0)>0 end
    if a.chk == Some(0) {
        let n1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
        let n2 = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
        let n3 = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
        let n4 = duel::get_location_count(d, 1 - a.tp, LOCATION_SZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
        return n1 + n2 + n3 + n4 > 0;
    }
    // local dis=Duel.SelectDisableField(tp,1,LOCATION_ONFIELD,LOCATION_ONFIELD,0)
    let dis = duel::select_disable_field(d, a.tp, 1, LOCATION_ONFIELD, LOCATION_ONFIELD, Some(0), false);
    // Duel.Hint(HINT_ZONE,tp,dis)
    duel::hint(d, HINT_ZONE as u8, a.tp, dis as u64);
    // Duel.SetTargetParam(dis)
    duel::set_target_param(d, dis);
    true
}
/// `s.zop(e,tp,eg,ep,ev,re,r,rp)`
fn zop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local dis=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
        let mut dis = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
        // if not c:IsControler(tp) then
        if !card::is_controler(d, c, a.tp) {
            // if dis>4096 then dis=dis>>16 else dis=dis<<16 end
            if dis > 4096 {
                dis >>= 16;
            } else {
                dis <<= 16;
            }
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_DISABLE_FIELD)
        effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetOperation(function(e) return dis end)
        effect::set_operation(d, e1, Some(lua_fn(move |_d, _a| dis)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    // return Duel.SelectEffectYesNo(tp,c,96) and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0
    duel::select_effect_yes_no(d, a.tp, c, Some(96)) && card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT) > 0
}
