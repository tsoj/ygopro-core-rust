//! A・O・J ディサイシブ・アームズ
//! Ally of Justice Decisive Armor
use super::prelude::*;

pub const ID: u32 = 9888196;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),2,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 2, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // destroy1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetTarget(s.destg1)
    effect::set_target(d, e1, Some(cost_target(destg1)));
    // e1:SetOperation(s.desop1)
    effect::set_operation(d, e1, Some(operation(desop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy2
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetCost(s.descost2)
    effect::set_cost(d, e2, Some(cost_target(descost2)));
    // e2:SetTarget(s.destg2)
    effect::set_target(d, e2, Some(cost_target(destg2)));
    // e2:SetOperation(s.desop2)
    effect::set_operation(d, e2, Some(operation(desop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // handes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_HANDES+CATEGORY_TOGRAVE+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_HANDES + CATEGORY_TOGRAVE + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.con)
    effect::set_condition(d, e3, Some(condition(con)));
    // e3:SetCost(s.hdcost)
    effect::set_cost(d, e3, Some(cost_target(hdcost)));
    // e3:SetTarget(s.hdtg)
    effect::set_target(d, e3, Some(cost_target(hdtg)));
    // e3:SetOperation(s.hdop)
    effect::set_operation(d, e3, Some(operation(hdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.confilter(c)`
fn confilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_LIGHT)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.confilter,tp,0,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&confilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
/// `s.destg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and s.filter1(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && filter1(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter1), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop1(e,tp,eg,ep,ev,re,r,rp)`
fn desop1(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFacedown() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_facedown(d, tc) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
/// `s.descost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.destg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter2,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter2), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&filter2), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&filter2), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
/// `s.hdcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,0,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, 1 - tp, LOCATION_HAND as i32);
    true
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, g.clone());
    // local sg=g:Filter(Card.IsAttribute,nil,ATTRIBUTE_LIGHT)
    let sg = group::filter(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // local atk=0
        let mut atk: i64 = 0;
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg.clone(), REASON_EFFECT, None, None);
        // local tc=sg:GetFirst()
        let _ = group::get_first(d, &sg);
        // for tc in aux.Next(sg) do
        for tc in group::iter(d, &sg) {
            // local tatk=tc:GetAttack()
            let mut tatk = card::get_attack(d, tc) as i64;
            // if tatk<0 then tatk=0 end
            if tatk < 0 {
                tatk = 0;
            }
            // atk=atk+tatk
            atk += tatk;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,atk,REASON_EFFECT)
        duel::damage(d, 1 - tp, atk, REASON_EFFECT, false, None);
    }
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
}
