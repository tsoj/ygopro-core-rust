//! D.D. Trap Hole
use super::prelude::*;

pub const ID: u32 = 5606466;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY | CATEGORY_REMOVE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_MSET);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e2, EVENT_CHANGE_POS);
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetTarget(s.target2)
    effect::set_target(d, e3, Some(cost_target(target2)));
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE) and c:GetReasonPlayer()==tp and c:IsCanBeEffectTarget(e) and c:IsAbleToRemove()
    card::is_position(d, c, POS_FACEDOWN_DEFENSE)
        && card::get_reason_player(d, c) == tp
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.filter2(c,e)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsAbleToRemove() and c:IsCanBeEffectTarget(e)
    card::is_able_to_remove(d, c, None, None, None) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.filter3(c,e,tp)`
fn filter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE) and c:IsSummonPlayer(tp) and c:IsCanBeEffectTarget(e) and c:IsAbleToRemove()
    card::is_position(d, c, POS_FACEDOWN_DEFENSE)
        && card::is_summon_player(d, c, tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local sg=eg:Filter(s.filter,nil,e,1-tp)
        let sg = group::filter(d, &a.eg.clone().nil(), &|d, c| filter(d, c, e, 1 - tp), NO_EX);
        // return #sg==1 and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE,0,1,sg:GetFirst(),e)
        let first = group::get_first(d, &sg);
        return sg.len() == 1
            && duel::is_existing_matching_card(d, Some(&|d, c| filter2(d, c, e)), tp, LOCATION_MZONE, 0, 1, first.into());
    }
    // local sg1=eg:Filter(s.filter,nil,e,1-tp)
    let mut sg1 = group::filter(d, &a.eg.clone().nil(), &|d, c| filter(d, c, e, 1 - tp), NO_EX);
    // e:SetLabelObject(sg1:GetFirst())
    let first1 = group::get_first(d, &sg1);
    effect::set_label_object(d, e, Value::card(first1));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local sg2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,sg1:GetFirst(),e)
    let sg2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, e)), tp, LOCATION_MZONE, 0, 1, 1, false, first1.into()).nil();
    // sg1:Merge(sg2)
    group::add_card(&mut sg1, &sg2);
    // Duel.SetTargetCard(sg1)
    duel::set_target_card(d, &sg1);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg1,#sg1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg1.clone()), sg1.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,sg1,#sg1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(sg1.clone()), sg1.len() as u8, 0, 0);
    true
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local sg=eg:Filter(s.filter3,nil,e,1-tp)
        let sg = group::filter(d, &a.eg.clone().nil(), &|d, c| filter3(d, c, e, 1 - tp), NO_EX);
        // return #sg==1 and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE,0,1,sg:GetFirst(),e)
        let first = group::get_first(d, &sg);
        return sg.len() == 1
            && duel::is_existing_matching_card(d, Some(&|d, c| filter2(d, c, e)), tp, LOCATION_MZONE, 0, 1, first.into());
    }
    // local sg1=eg:Filter(s.filter3,nil,e,1-tp)
    let mut sg1 = group::filter(d, &a.eg.clone().nil(), &|d, c| filter3(d, c, e, 1 - tp), NO_EX);
    // e:SetLabelObject(sg1:GetFirst())
    let first1 = group::get_first(d, &sg1);
    effect::set_label_object(d, e, Value::card(first1));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local sg2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,sg1:GetFirst(),e)
    let sg2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, e)), tp, LOCATION_MZONE, 0, 1, 1, false, first1.into()).nil();
    // sg1:Merge(sg2)
    group::add_card(&mut sg1, &sg2);
    // Duel.SetTargetCard(sg1)
    duel::set_target_card(d, &sg1);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg1,#sg1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg1.clone()), sg1.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,sg1,#sg1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(sg1.clone()), sg1.len() as u8, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local tc1=g:GetFirst()
    // local tc2=g:GetNext()
    let order = group::iter(d, &g);
    let tc1 = order.first().copied();
    let tc2 = order.get(1).copied();
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, e);
    // if tc1 and tc1:IsRelateToEffect(e) and tc2 and tc2:IsRelateToEffect(e) and sc:IsFacedown() then
    if let (Some(tc1), Some(tc2)) = (tc1, tc2) {
        if card::is_relate_to_effect(d, tc1, e) && card::is_relate_to_effect(d, tc2, e) {
            let sc = sc.as_card().nil();
            if card::is_facedown(d, sc) {
                // Duel.Destroy(g,REASON_EFFECT,LOCATION_REMOVED)
                duel::destroy(d, g, REASON_EFFECT, Some(LOCATION_REMOVED), None);
            }
        }
    }
}
