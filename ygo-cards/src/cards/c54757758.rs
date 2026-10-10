//! Mudragon of the Swamp
use super::prelude::*;

pub const ID: u32 = 54757758;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter(d, c, a))), 2)]);
    // Cannot be targeted
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // Prevent target
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.tglimit)
    effect::set_target(d, e2, Some(effect_card_fn(tglimit)));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change attribute
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.atttg)
    effect::set_target(d, e3, Some(cost_target(atttg)));
    // e3:SetOperation(s.attop)
    effect::set_operation(d, e3, Some(operation(attop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,sp,sub,mg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return not sg or sg:FilterCount(aux.TRUE,c)==0 or (sg:IsExists(Card.IsAttribute,1,c,c:GetAttribute(),fc,sumtype,sp) and not sg:IsExists(Card.IsRace,1,c,c:GetRace(),fc,sumtype,sp))
    let Some(sg) = a.sg else {
        return true;
    };
    if group::filter_count(d, sg, &|_, _| true, c.into()) == 0 {
        return true;
    }
    let (fc, sumtype, sp) = (a.fc, a.sumtype, a.tp);
    let attr = card::get_attribute(d, c);
    let race = card::get_race(d, c);
    group::is_exists(d, sg, &|d, x| card::is_attribute_ctx(d, x, attr, Some(fc), sumtype, Some(sp)), 1, c.into())
        && !group::is_exists(d, sg, &|d, x| card::is_race_ctx(d, x, race, Some(fc), sumtype, Some(sp)), 1, c.into())
}
/// `s.tglimit(e,c)`
fn tglimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(e:GetHandler():GetAttribute())
    let h = effect::get_handler(d, e).nil();
    let attr = card::get_attribute(d, h);
    card::is_attribute(d, c, attr)
}
/// `s.atttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local aat=e:GetHandler():AnnounceAnotherAttribute(tp)
    let h = effect::get_handler(d, a.e).nil();
    let aat = card::announce_another_attribute(d, h, a.tp);
    // e:SetLabel(aat)
    effect::set_label(d, a.e, &[aat as i64]);
    true
}
/// `s.attop(e,tp,eg,ep,ev,re,r,rp)`
fn attop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e1, label);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
