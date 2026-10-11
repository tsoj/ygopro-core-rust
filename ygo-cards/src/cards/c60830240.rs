//! Void Reignition
use super::prelude::*;

pub const ID: u32 = 60830240;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNOID, SET_VOID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Discard entire hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.dccost)
    effect::set_cost(d, e1, Some(cost_target(dccost)));
    // e1:SetTarget(s.dctg)
    effect::set_target(d, e1, Some(cost_target(dctg)));
    // e1:SetOperation(s.dcop)
    effect::set_operation(d, e1, Some(operation(dcop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return up to 11 banished "Infernoid" monsters to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.dccostfilter(c)`
fn dccostfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsPublic() and ((c:IsSetCard(SET_INFERNOID) and c:IsMonster()) or (c:IsSetCard(SET_VOID) and c:IsSpellTrap()))
    !card::is_public(d, c)
        && ((card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::is_monster(d, c))
            || (card::is_set_card(d, c, &[SET_VOID as u16]) && card::is_spell_trap(d, c)))
}
/// `s.dccost(e,tp,...,chk)`
fn dccost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.dccostfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&dccostfilter), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.dccostfilter,tp,LOCATION_HAND,0,1,1,c)
    let g = duel::select_matching_card(d, tp, Some(&dccostfilter), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
/// `s.dctg(e,tp,...,chk)`
fn dctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)-e:GetHandler()
    let mut g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    let h = effect::get_handler(d, a.e).nil();
    group::remove_card(&mut g, &CardSet::from_card(h));
    // local ct=#g
    let ct = g.len();
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDraw(tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, tp, ct as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, Some(g), ct as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    true
}
/// `s.dcop(e,tp,...)`
fn dcop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
    let ct = duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    // if ct>0 and Duel.IsPlayerCanDraw(tp) then
    if ct > 0 && duel::is_player_can_draw(d, tp, 0) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,ct,REASON_EFFECT)
        duel::draw(d, tp, ct as u32, REASON_EFFECT);
    }
}
/// `s.tgcon(e,tp,...)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.exccon(e,tp,eg,ep,ev,re,r,rp)
    //     and not Duel.IsExistingMatchingCard(aux.NOT(aux.FaceupFilter(Card.IsRace,RACE_FIEND)),tp,LOCATION_MZONE,0,1,nil)
    aux::exccon().call(d, &[Value::Effect(a.e)]).truthy()
        && !duel::is_existing_matching_card(d, Some(&|d, c| !(card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsMonster() and c:IsSetCard(SET_INFERNOID) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_monster(d, c) && card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.tgtg(e,tp,...,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_REMOVED,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e)), tp, LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,11,aux.dncheck,1,tp,aux.Stringid(id,2))
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(11), Some(&aux::dncheck()), 1, tp, Some(aux::stringid(ID, 2)), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,tg,#tg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(tg.clone()), tg.len() as u8, 0, 0);
    true
}
/// `s.tgop(e,tp,...)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT|REASON_RETURN) end
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_RETURN, None, None);
    }
}
