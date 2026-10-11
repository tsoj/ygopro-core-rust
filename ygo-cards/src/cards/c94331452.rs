//! Symphonic Warrior Drumss
use super::prelude::*;

pub const ID: u32 = 94331452;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SYMPHONIC_WARRIOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change the Attribute of 1 "Symphonic Warrior" monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target(LOCATION_MZONE))
    effect::set_target(d, e1, Some(cost_target(target(LOCATION_MZONE))));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change the Attribute of 1 "Symphonic Warrior" monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.target(0))
    effect::set_target(d, e2, Some(cost_target(target(0))));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(oppo)`: the target function for the given `oppo` (the opponent's zone flag)
fn target(oppo: u32) -> impl Fn(&mut Duel, &EffArgs) -> bool + Send + Sync + 'static {
    move |d, a| target_body(d, a, oppo)
}
/// `function (e,tp,eg,ep,ev,re,r,rp,chk,chkc)` of `s.target(oppo)`
fn target_body(d: &mut Duel, a: &EffArgs, oppo: u32) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and (oppo==0 or chkc:IsControler(tp)) and chkc:IsFaceup()
    //     and chkc:IsSetCard(SET_SYMPHONIC_WARRIOR) and chkc:IsAttributeExcept(e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && (oppo == 0 || card::is_controler(d, chkc, a.tp))
            && card::is_faceup(d, chkc)
            && card::is_set_card(d, chkc, &[SET_SYMPHONIC_WARRIOR as u16])
            && card::is_attribute_except(d, chkc, effect::get_label(d, a.e) as u32, None, 0, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_SYMPHONIC_WARRIOR),tp,LOCATION_MZONE,oppo,1,nil) end
    let faceup_sw = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_SYMPHONIC_WARRIOR as u16]));
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_sw), a.tp, LOCATION_MZONE, oppo, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsCanBeEffectTarget,aux.FaceupFilter(Card.IsSetCard,SET_SYMPHONIC_WARRIOR)),tp,LOCATION_MZONE,oppo,nil,e)
    let e = a.e;
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_can_be_effect_target(d, c, Some(e)) && faceup_sw(d, c)),
        a.tp,
        LOCATION_MZONE,
        oppo,
        NO_EX,
    );
    // local att=Duel.AnnounceAnotherAttribute(g,tp)
    let att = duel::announce_another_attribute(d, &g, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local sel=g:FilterSelect(tp,Card.IsAttributeExcept,1,1,nil,att)
    let sel = group::filter_select(d, &g, a.tp, &|d, c| card::is_attribute_except(d, c, att, None, 0, None), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(sel)
    duel::set_target_card(d, sel);
    // e:SetLabel(att)
    effect::set_label(d, e, &[att as i64]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e1, label);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
