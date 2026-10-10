//! Branded in White
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 34995106;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon
    // local e1=Fusion.CreateSummonEff({handler=c,extrafil=s.fextra,extraop=s.extraop,extratg=s.extratg})
    let params = fusion::SummonEff {
        extrafil: Some(std::sync::Arc::new(fextra)),
        extraop: Some(fusion::extra_op(extraop)),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    };
    let e1 = fusion::create_summon_eff(d, c, params);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set itself during the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // if not sg:IsExists(Card.IsRace,1,nil,RACE_DRAGON) then return false end
    let sg = a.g(1);
    if !group::is_exists(d, &sg, &|d, c| card::is_race(d, c, RACE_DRAGON as u64), 1, NO_EX) {
        return false;
    }
    // if sg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE) then return sg:IsExists(Card.IsCode,1,nil,CARD_ALBAZ) end
    if group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX) {
        return group::is_exists(d, &sg, &|d, c| card::is_code(d, c, &[CARD_ALBAZ]), 1, NO_EX);
    }
    // return true
    true
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // local eg=Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
    let eg = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #eg>0 and (mg+eg):IsExists(Card.IsCode,1,nil,CARD_ALBAZ) then
    let mut union = mg.clone();
    group::add_card(&mut union, &eg);
    if !eg.is_empty() && group::is_exists(d, &union, &|d, c| card::is_code(d, c, &[CARD_ALBAZ]), 1, NO_EX) {
        // if #eg>0 then return eg,s.fcheck end
        if !eg.is_empty() {
            return (Some(eg), Some(lua_fn(fcheck)));
        }
    }
    // return nil,s.fcheck
    (None, Some(lua_fn(fcheck)))
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, _tp: u8, sg: &mut CardSet) -> bool {
    // local rg=sg:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let rg = group::filter(d, &*sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT|REASON_MATERIAL|REASON_FUSION)
        duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None, None);
        // sg:Sub(rg)
        group::remove_card(sg, &rg);
    }
    true
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsReason(REASON_COST) and re and re:IsActivated() and re:GetHandler():IsCode(CARD_ALBAZ) then
    if card::is_reason(d, c, REASON_COST) {
        if let Some(re) = a.re {
            if effect::is_activated(d, re) && card::is_code(d, effect::get_handler(d, re).nil(), &[CARD_ALBAZ]) {
                // Set itself from the GY
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(aux.Stringid(id,1))
                effect::set_description(d, e1, aux::stringid(ID, 1));
                // e1:SetCategory(CATEGORY_SET)
                effect::set_category(d, e1, CATEGORY_SET);
                // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
                // e1:SetCode(EVENT_PHASE+PHASE_END)
                effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
                // e1:SetRange(LOCATION_GRAVE)
                effect::set_range(d, e1, LOCATION_GRAVE);
                // e1:SetCountLimit(1,{id,1})
                effect::set_count_limit(d, e1, 1, ID, 1, 0);
                // e1:SetTarget(s.settg)
                effect::set_target(d, e1, Some(cost_target(settg)));
                // e1:SetOperation(s.setop)
                effect::set_operation(d, e1, Some(operation(setop)));
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c)
        duel::s_set(d, a.tp, c, None, true);
    }
}
