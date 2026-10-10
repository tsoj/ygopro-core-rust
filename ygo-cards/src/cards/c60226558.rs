//! Nephe Shaddoll Fusion
use super::prelude::*;

pub const ID: u32 = 60226558;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_SHADDOLL}
    listed_series: &[SET_SHADDOLL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_SHADDOLL),nil,nil,s.target,s.operation)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_set_card(d, x, &[SET_SHADDOLL as u16]))),
        None,
        None,
        Some(std::sync::Arc::new(target)),
        Some(operation(op)),
        None,
        None,
    );
    // spsummon
    // local params = {aux.FilterBoolFunction(Card.IsSetCard,SET_SHADDOLL),nil,nil,nil,s.forcedmat}
    let params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, x, _tp| card::is_set_card(d, x, &[SET_SHADDOLL as u16]))),
        gc: Some(std::sync::Arc::new(forcedmat)),
        ..Default::default()
    };
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.fcond)
    effect::set_condition(d, e3, Some(condition(fcond)));
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e3, Some(tg));
    // e3:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let eop = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e3, Some(eop));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn target(d: &mut Duel, a: &EffArgs, tc: Option<CardId>) {
    // local att=tc:AnnounceAnotherAttribute(tp)
    let att = card::announce_another_attribute(d, tc.nil(), a.tp);
    // e:SetLabel(att)
    effect::set_label(d, a.e, &[att as i64]);
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
                // local att=e:GetLabel()
                let att = effect::get_label(d, a.e);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_EQUIP)
                effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
                // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
                effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
                // e1:SetValue(att)
                effect::set_value(d, e1, att);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
                // c:SetHint(CHINT_ATTRIBUTE,att)
                card::set_hint(d, c, CHINT_ATTRIBUTE as u8, att as u64);
            }
        }
    }
}
fn fcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget() and e:GetHandler():GetEquipTarget():IsControler(tp)
    let h = effect::get_handler(d, a.e).nil();
    match card::get_equip_target(d, h) {
        Some(t) => card::is_controler(d, t, a.tp),
        None => false,
    }
}
fn forcedmat(d: &mut Duel, a: &EffArgs) -> Value {
    // return e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    Value::card(card::get_equip_target(d, h))
}
