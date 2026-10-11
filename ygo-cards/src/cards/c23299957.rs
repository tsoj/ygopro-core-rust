//! Vehicroid Connection Zone
use super::prelude::*;

pub const ID: u32 = 23299957;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VEHICROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon 1 "Vehicroid" fusion monster
    // Using monsters from hand or field as fusion material
    // c:RegisterEffect(Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_VEHICROID),nil,nil,nil,nil,s.stage2))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_VEHICROID as u16]))),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    card::register_effect(d, c, e1, false);
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    if chk == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // Cannot be destroyed by card effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3001)
        effect::set_description(d, e1, 3001);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc.nil(), e1, true);
        // Cannot be negated
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3308)
        effect::set_description(d, e2, 3308);
        // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CANNOT_DISABLE)
        effect::set_code(d, e2, EFFECT_CANNOT_DISABLE);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc.nil(), e2, true);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetCode(EFFECT_CANNOT_DISEFFECT)
        effect::set_code(d, e3, EFFECT_CANNOT_DISEFFECT);
        // e3:SetRange(LOCATION_MZONE)
        effect::set_range(d, e3, LOCATION_MZONE);
        // e3:SetValue(s.efilter)
        effect::set_value_fn(d, e3, lua_fn(efilter));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e3,true)
        card::register_effect(d, tc.nil(), e3, true);
    }
}
/// `s.efilter(e,ct)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // local te=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT)
    let ct = a.int(1) as u8;
    let te = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // return te:GetHandler()==e:GetHandler()
    effect::get_handler(d, te) == effect::get_handler(d, a.e(0))
}
