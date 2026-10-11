//! Cupid Serve
use super::prelude::*;

pub const ID: u32 = 1566817;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Unaffected by the activated effects of monsters with a higher level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| efilter(d, a.e(0), a.e(1))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase its level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.lvcost)
    effect::set_cost(d, e2, Some(cost_target(lvcost)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    // local c,tc=e:GetHandler(),te:GetHandler()
    let c = effect::get_handler(d, e).nil();
    let tc = effect::get_handler(d, te).nil();
    // return te:IsMonsterEffect() and (not tc:HasLevel() or tc:GetLevel()>c:GetLevel())
    effect::is_monster_effect(d, te) && (!card::has_level(d, tc) || card::get_level(d, tc) > card::get_level(d, c))
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c)
    card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.lvcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,3,e:GetHandler())
    let cg = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 3, false, h.into()).nil();
    // Duel.Remove(cg,POS_FACEUP,REASON_COST)
    let n = cg.len();
    duel::remove(d, cg, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(#cg)
    effect::set_label(d, a.e, &[n as i64]);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(ct)
        effect::set_value(d, e1, ct);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
