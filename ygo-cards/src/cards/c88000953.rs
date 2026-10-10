//! Rasterliger
use super::prelude::*;

pub const ID: u32 = 88000953;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, except Tokens
    // Link.AddProcedure(c,aux.NOT(aux.FilterBoolFunctionEx(Card.IsType,TYPE_TOKEN)),2)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| !card::is_type_ctx(d, x, TYPE_TOKEN, sc, st, Some(tp)))), 2, None, None, None, None);
    // This card gains ATK equal to the ATK of 1 Link Monster in either GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Destroy a number of cards on the field equal to the number of cards Tributed
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(descost)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsLinkMonster() and chkc:HasNonZeroAttack() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_link_monster(d, chkc) && card::has_non_zero_attack(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsLinkMonster,Card.HasNonZeroAttack),tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&linkatk), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,aux.AND(Card.IsLinkMonster,Card.HasNonZeroAttack),tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
    duel::select_target(d, a.tp, Some(&linkatk), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    true
}
/// `aux.AND(Card.IsLinkMonster,Card.HasNonZeroAttack)`
fn linkatk(d: &mut Duel, c: CardId) -> bool {
    card::is_link_monster(d, c) && card::has_non_zero_attack(d, c)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        let tc = tc.nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(tc:GetAttack())
        let atk = card::get_attack(d, tc) as i64;
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.descostfilter(c,lg)`
fn descostfilter(_d: &mut Duel, c: CardId, lg: &CardSet) -> bool {
    // return lg:IsContains(c)
    group::is_contains(lg, c)
}
/// `s.rescon(sg,tp,exg)`
fn rescon(d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet) -> (bool, bool) {
    // return Duel.IsExistingMatchingCard(nil,0,LOCATION_ONFIELD,LOCATION_ONFIELD,#sg,sg)
    (duel::is_existing_matching_card(d, None, 0, LOCATION_ONFIELD, LOCATION_ONFIELD, sg.len() as u32, sg.into()), false)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // if chk==0 then return #lg>0 and Duel.CheckReleaseGroupCost(tp,s.descostfilter,1,false,s.rescon,nil,lg) end
    if a.chk == Some(0) {
        return !lg.is_empty() && duel::check_release_group_cost(d, a.tp, Some(&|d, c| descostfilter(d, c, &lg)), 1, 1, false, Some(&rescon), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.descostfilter,1,#lg,false,s.rescon,nil,lg)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| descostfilter(d, c, &lg)), 1, lg.len(), false, Some(&rescon), NO_EX);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[g.len() as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=label==-100
        let res = label == -100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return res;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,label,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), label as u8, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)<ct then return end
    if (duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD) as i64) < ct {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,ct,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u16, ct as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
