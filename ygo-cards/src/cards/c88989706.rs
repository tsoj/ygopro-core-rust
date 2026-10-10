//! Great Dezard
use super::prelude::*;

pub const ID: u32 = 88989706;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[39711336], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // check
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
    // negate
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_NEGATE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_F);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // special summon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `function s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToBattle() then c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD,0,0) end
    if card::is_relate_to_battle(d, c) {
        card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    }
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():GetFlagEffect(id+1)==0 or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if card::get_flag_effect(d, h, ID + 1) == 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    // if not tg or not tg:IsContains(e:GetHandler()) then return false end
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    let Some(tg) = tg else { return false };
    if !tg.contains(h) {
        return false;
    }
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    // if re:GetHandler():IsRelateToEffect(re) then
    let rc = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rc, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetCurrentChain()~=ev+1 then return end
    if duel::get_current_chain(d, false) != a.ev as i32 + 1 {
        return;
    }
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id+1)>=2
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID + 1) >= 2
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(39711336) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[39711336]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), 0, tp, tp, true, false, POS_FACEUP as u8, None) > 0 {
        // g:GetFirst():CompleteProcedure()
        let tc = group::get_first(d, &g).nil();
        card::complete_procedure(d, tc);
    }
}
