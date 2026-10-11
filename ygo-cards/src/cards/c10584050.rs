//! Springans Blast!
use super::prelude::*;

pub const ID: u32 = 10584050;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPRINGANS], listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SPRINGANS),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SPRINGANS as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.bonusfilter(c)
fn bonusfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:ListsCodeAsMaterial(CARD_ALBAZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION) && card::lists_code_as_material(d, c, &[CARD_ALBAZ])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local seqs={}
    let mut seqs: Vec<i64> = Vec::new();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local zone1=Duel.SelectFieldZone(tp,1,0,LOCATION_MZONE,ZONES_EMZ<<16)
    let zone1 = duel::select_field_zone(d, a.tp, 1, 0, LOCATION_MZONE, Some(ZONES_EMZ << 16));
    // table.insert(seqs, math.log(zone1,2)-16)
    seqs.push((zone1 as f64).log2() as i64 - 16);
    // Duel.Hint(HINT_ZONE,tp,zone1)
    duel::hint(d, HINT_ZONE as u8, a.tp, zone1 as u64);
    // if Duel.IsExistingMatchingCard(s.bonusfilter,tp,LOCATION_MZONE,0,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::is_existing_matching_card(d, Some(&bonusfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
        // local zone2=Duel.SelectFieldZone(tp,1,0,LOCATION_MZONE,(ZONES_EMZ<<16)|zone1)
        let zone2 = duel::select_field_zone(d, a.tp, 1, 0, LOCATION_MZONE, Some((ZONES_EMZ << 16) | zone1));
        // table.insert(seqs, math.log(zone2,2)-16)
        seqs.push((zone2 as f64).log2() as i64 - 16);
        // Duel.Hint(HINT_ZONE,tp,zone2)
        duel::hint(d, HINT_ZONE as u8, a.tp, zone2 as u64);
    }
    // e:SetLabel(table.unpack(seqs))
    effect::set_label(d, a.e, &seqs);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local s1,s2=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let s1 = labels.first().copied().unwrap_or(0);
    let s2 = labels.get(1).copied();
    // local z1=Duel.GetFieldCard(1-tp,LOCATION_MZONE,s1)
    let z1 = duel::get_field_card(d, 1 - a.tp, LOCATION_MZONE, s1 as u32);
    match z1 {
        // if z1 and z1:IsFaceup() then
        Some(z1) if card::is_faceup(d, z1) => {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // z1:RegisterEffect(e1)
            card::register_effect(d, z1, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // z1:RegisterEffect(e2)
            card::register_effect(d, z1, e2, false);
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e3:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
            effect::set_code(d, e3, EFFECT_CANNOT_DIRECT_ATTACK);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // z1:RegisterEffect(e3)
            card::register_effect(d, z1, e3, false);
        }
        // elseif z1==nil then
        None => {
            // local e4=Effect.CreateEffect(c)
            let e4 = effect::create_effect(d, c);
            // e4:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e4, EFFECT_TYPE_FIELD);
            // e4:SetCode(EFFECT_DISABLE_FIELD)
            effect::set_code(d, e4, EFFECT_DISABLE_FIELD);
            // e4:SetValue((1<<s1)<<((1-tp)*16))
            effect::set_value(d, e4, (1i64 << s1) << ((1 - a.tp as i64) * 16));
            // e4:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
            // Duel.RegisterEffect(e4,tp)
            duel::register_effect(d, e4, a.tp);
        }
        _ => {}
    }
    // if s2 then
    if let Some(s2) = s2 {
        // local z2=Duel.GetFieldCard(1-tp,LOCATION_MZONE,s2)
        let z2 = duel::get_field_card(d, 1 - a.tp, LOCATION_MZONE, s2 as u32);
        match z2 {
            // if z2 and z2:IsFaceup() then
            Some(z2) if card::is_faceup(d, z2) => {
                // local e5=Effect.CreateEffect(c)
                let e5 = effect::create_effect(d, c);
                // e5:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
                // e5:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e5, EFFECT_DISABLE);
                // e5:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e5, RESETS_STANDARD_PHASE_END, 1);
                // z2:RegisterEffect(e5)
                card::register_effect(d, z2, e5, false);
                // local e6=Effect.CreateEffect(c)
                let e6 = effect::create_effect(d, c);
                // e6:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
                // e6:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e6, EFFECT_DISABLE_EFFECT);
                // e6:SetValue(RESET_TURN_SET)
                effect::set_value(d, e6, RESET_TURN_SET as i64);
                // e6:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e6, RESETS_STANDARD_PHASE_END, 1);
                // z2:RegisterEffect(e6)
                card::register_effect(d, z2, e6, false);
                // local e7=Effect.CreateEffect(c)
                let e7 = effect::create_effect(d, c);
                // e7:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
                // e7:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e7, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e7:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
                effect::set_code(d, e7, EFFECT_CANNOT_DIRECT_ATTACK);
                // e7:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e7, RESETS_STANDARD_PHASE_END, 1);
                // z2:RegisterEffect(e7)
                card::register_effect(d, z2, e7, false);
            }
            // elseif z2==nil then
            None => {
                // local e8=Effect.CreateEffect(c)
                let e8 = effect::create_effect(d, c);
                // e8:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e8, EFFECT_TYPE_FIELD);
                // e8:SetCode(EFFECT_DISABLE_FIELD)
                effect::set_code(d, e8, EFFECT_DISABLE_FIELD);
                // e8:SetValue((1<<s2)<<((1-tp)*16))
                effect::set_value(d, e8, (1i64 << s2) << ((1 - a.tp as i64) * 16));
                // e8:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e8, RESETS_STANDARD_PHASE_END, 1);
                // Duel.RegisterEffect(e8,tp)
                duel::register_effect(d, e8, a.tp);
            }
            _ => {}
        }
    }
}
