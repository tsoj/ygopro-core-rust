//! Thorn Fangs of Violet Poison
//! 紫毒の茨牙
use super::prelude::*;

pub const ID: u32 = 72374522;
const CARD_STARVING_VENOM_FUSION_DRAGON: u32 = 41209827;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PREDAP], listed_names: &[CARD_STARVING_VENOM_FUSION_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_HANDES+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_HANDES + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_STARVING_VENOM_FUSION_DRAGON) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackBelow,c:GetAttack()-1),tp,0,LOCATION_MZONE,1,nil)
    if !(card::is_code(d, c, &[CARD_STARVING_VENOM_FUSION_DRAGON]) && card::is_faceup(d, c)) {
        return false;
    }
    let atk = card::get_attack(d, c) - 1;
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_attack_below(d, c, atk))), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackBelow,tc:GetAttack()-1),tp,0,LOCATION_MZONE,nil)
    let atk = card::get_attack(d, tc) - 1;
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_attack_below(d, c, atk))), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,g:GetSum(Card.GetBaseAttack))
    let sum = group::get_sum(d, &g, &|d, c| card::get_base_attack(d, c) as i64);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, sum as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, handler);
        // e1:SetDescription(3206)
        effect::set_description(d, e1, 3206);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackBelow,tc:GetAttack()-1),tp,0,LOCATION_MZONE,nil)
        let atk = card::get_attack(d, tc) - 1;
        let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_attack_below(d, c, atk))), tp, 0, LOCATION_MZONE, NO_EX);
        // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
        let hg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
        // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 and #hg>0 and #hg==hg:FilterCount(Card.IsDiscardable,nil,REASON_EFFECT)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g.is_empty()
            && duel::destroy(d, g, REASON_EFFECT, None, None) > 0
            && !hg.is_empty()
            && hg.len() == group::filter_count(d, &hg, &|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None), NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // local dam=Duel.GetOperatedGroup():Match(aux.NOT(Card.IsTextAttack),nil,-2):GetSum(Card.GetTextAttack)
            let mut og = duel::get_operated_group(d);
            group::match_(d, &mut og, &|d, c| !card::is_text_attack(d, c, -2), NO_EX);
            let dam = group::get_sum(d, &og, &|d, c| card::get_text_attack(d, c) as i64);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SendtoGrave(hg,REASON_EFFECT|REASON_DISCARD)==#hg then
            if duel::send_to_grave(d, hg.clone(), REASON_EFFECT | REASON_DISCARD, None, None) == hg.len() as i32 {
                // Duel.Damage(1-tp,dam,REASON_EFFECT)
                duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
            }
        }
    }
}
