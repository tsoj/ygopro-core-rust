//! Supreme King Dragon Starving Venom
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 43387895;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion materials: 2 DARK Pendulum monsters
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(ffilter)), 2)]);
    // Special Summon this card by tributing the above cards you control
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit,nil,nil,nil,false)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(Arc::new(|d, tp| duel::get_release_group(d, tp, false, false, None))),
            material_operation: Some(Arc::new(|d, g, _tp, _c| {
                // Duel.Release(g,REASON_COST|REASON_MATERIAL)
                duel::release(d, g.clone(), REASON_COST | REASON_MATERIAL, None);
            })),
            sumcon: Some(Some(lua_fn(splimit))),
            cannot_be_lizard: Some(false),
            ..Default::default()
        },
    );
    // Copy the name and effect of another monster
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.copycost)
    effect::set_cost(d, e1, Some(cost_target(copycost)));
    // e1:SetTarget(s.copytg)
    effect::set_target(d, e1, Some(cost_target(copytg)));
    // e1:SetOperation(s.copyop)
    effect::set_operation(d, e1, Some(operation(copyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK,fc,sumtype,tp) and c:IsType(TYPE_PENDULUM,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_PENDULUM, Some(fc), sumtype, Some(tp))
}
/// `s.splimit(e,se,sp,st)`: `(st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    let st = a.int(3) as u64;
    (st & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64
}
/// `s.copycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn copycost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(41209827)==0 end
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, 41209827, None);
    }
    // c:RegisterFlagEffect(41209827,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, 41209827, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
/// `s.copyfilter(c)`
fn copyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and not c:IsType(TYPE_TOKEN) and (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE))
    card::is_monster(d, c) && !card::is_type(d, c, TYPE_TOKEN) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
}
/// `s.copytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn copytg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.copyfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && copyfilter(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.copyfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&copyfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.copyfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,1,c)
    duel::select_target(d, a.tp, Some(&copyfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, c.into());
    true
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local code=tc:GetOriginalCode()
            let code = card::get_original_code(d, tc);
            // This card's name becomes the target's name
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_CODE)
            effect::set_code(d, e1, EFFECT_CHANGE_CODE);
            // e1:SetValue(code)
            effect::set_value(d, e1, code as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // if not tc:IsType(TYPE_TRAPMONSTER) then
            if !card::is_type(d, tc, TYPE_TRAPMONSTER) {
                // c:CopyEffect(code,RESETS_STANDARD_PHASE_END,1)
                card::copy_effect(d, c, code, RESETS_STANDARD_PHASE_END, 1);
            }
        }
    }
    // Inflict piercing battle damage to your opponent for the rest of the turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
