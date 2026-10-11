//! Baneshade
use super::prelude::*;

pub const ID: u32 = 31969219;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effect to an opponent's monster based on the number of cards with its name in their GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if c:IsFacedown() then return false end
    if card::is_facedown(d, c) {
        return false;
    }
    // local ct=Duel.GetMatchingGroupCount(Card.IsCode,tp,0,LOCATION_GRAVE,nil,c:GetCode())
    let codes = card::get_code(d, c);
    let ct = duel::get_matching_group_count(d, Some(&|d, x| card::is_code(d, x, &codes)), tp, 0, LOCATION_GRAVE, NO_EX);
    // if ct==0 then return false end
    if ct == 0 {
        return false;
    }
    // if ct==1 then return true end
    if ct == 1 {
        return true;
    }
    // if ct==2 then return c:IsAbleToRemove() end
    if ct == 2 {
        return card::is_able_to_remove(d, c, None, None, None);
    }
    // return c:IsAbleToRemove(tp,POS_FACEDOWN) and Duel.IsExistingMatchingCard(s.rmfdfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,c,tp,c:GetCode())
    card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)
        && duel::is_existing_matching_card(d, Some(&|d, x| rmfdfilter(d, x, tp, &codes)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, c.into())
}
/// `s.rmfdfilter(c,tp,...)`: `c:IsFaceup() and c:IsCode(...) and c:IsAbleToRemove(tp,POS_FACEDOWN)`
fn rmfdfilter(d: &mut Duel, c: CardId, tp: u8, codes: &[u32]) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, codes) && card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)
}
/// `s.efftg`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && tgfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_ONFIELD,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_ONFIELD,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local gg=Duel.GetMatchingGroup(Card.IsCode,tp,0,LOCATION_GRAVE,nil,tc:GetCode())
    let codes = card::get_code(d, tc);
    let gg = duel::get_matching_group(d, Some(&|d, x| card::is_code(d, x, &codes)), a.tp, 0, LOCATION_GRAVE, NO_EX);
    // local ct=#gg
    let ct = gg.len();
    if ct == 1 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    } else if ct == 2 {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    } else if ct >= 3 {
        // local rg=Duel.GetMatchingGroup(s.rmfdfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil,tp,tc:GetCode())
        let rg = duel::get_matching_group(d, Some(&|d, x| rmfdfilter(d, x, a.tp, &codes)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rg,#rg,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg.clone()), rg.len() as u8, a.tp, 0);
    }
    true
}
/// `s.effop`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // local gg=Duel.GetMatchingGroup(Card.IsCode,tp,0,LOCATION_GRAVE,nil,tc:GetCode())
    let codes = card::get_code(d, tc);
    let gg = duel::get_matching_group(d, Some(&|d, x| card::is_code(d, x, &codes)), a.tp, 0, LOCATION_GRAVE, NO_EX);
    // local ct=#gg
    let ct = gg.len();
    if ct == 1 {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    } else if ct == 2 {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
    } else if ct >= 3 {
        // local rg=Duel.GetMatchingGroup(s.rmfdfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil,tp,tc:GetCode())+tc
        let mut rg = duel::get_matching_group(d, Some(&|d, x| rmfdfilter(d, x, a.tp, &codes)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
        group::add_card(&mut rg, &CardSet::from_card(tc));
        // if #rg>0 then Duel.Remove(rg,POS_FACEDOWN,REASON_EFFECT) end
        if !rg.is_empty() {
            duel::remove(d, rg, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
        }
    }
}
