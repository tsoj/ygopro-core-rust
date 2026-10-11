//! Delta Tri
use super::prelude::*;

pub const ID: u32 = 12079734;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(aux.bdocon)
    effect::set_condition(d, e1, Some(aux::bdocon()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,ec)`
fn filter1(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsType(TYPE_UNION) and c:CheckUnionTarget(ec) and aux.CheckUnionEquip(c,ec)
    card::is_type(d, c, TYPE_UNION) && card::check_union_target(d, c, ec) && aux::check_union_equip(d, c, ec)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if e:GetLabel()==0 then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter1(chkc,c)
        if effect::get_label(d, a.e) == 0 {
            return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter1(d, chkc, c);
        }
        // else return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter2(chkc) end
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter2(d, chkc);
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,0,1,nil,c)
    let b1 = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, x| filter1(d, x, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,nil) and Duel.IsPlayerCanDraw(tp,1)
    let b2 = duel::is_existing_target(d, Some(&filter2), a.tp, LOCATION_MZONE, 0, 1, NO_EX) && duel::is_player_can_draw(d, a.tp, 1);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    let sel: i32;
    // if b1 and b2 then
    if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        sel = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    // elseif b1 then
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))
        sel = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]);
    // else op=Duel.SelectOption(tp,aux.Stringid(id,2))+1 end
    } else {
        sel = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) + 1;
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[sel as i64]);
    // if op==0 then
    if sel == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,0,1,1,nil,c)
        let g = duel::select_target(d, a.tp, Some(&|d, x| filter1(d, x, c)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_EQUIP);
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    // else
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&filter2), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and c:IsFaceup() and tc and tc:IsRelateToEffect(e)
        //   and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
        //   and aux.CheckUnionEquip(tc,c) and Duel.Equip(tp,tc,c,false) then
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            if let Some(tc) = tc {
                if card::is_relate_to_effect(d, tc, a.e)
                    && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
                    && aux::check_union_equip(d, tc, c)
                    && duel::equip(d, a.tp, tc, c, false, false)
                {
                    // aux.SetUnionState(tc)
                    aux::set_union_state(d, tc);
                }
            }
        }
    // else
    } else if let Some(tc) = tc {
        // if tc and tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0
            && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
        {
            // if tc:IsLocation(LOCATION_DECK) then Duel.ShuffleDeck(tp) end
            if card::is_location(d, tc, LOCATION_DECK) {
                duel::shuffle_deck(d, a.tp);
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
