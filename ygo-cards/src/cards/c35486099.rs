//! Crystal Blessing
use super::prelude::*;

pub const ID: u32 = 35486099;

// s.listed_series={SET_CRYSTAL_BEAST}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRYSTAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && !card::is_forbidden(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if not Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) then return false end
        if !duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX) {
            return false;
        }
        // if e:GetHandler():IsLocation(LOCATION_HAND) then
        let h = effect::get_handler(d, a.e).nil();
        if card::is_location(d, h, LOCATION_HAND) {
            // return Duel.GetLocationCount(tp,LOCATION_SZONE)>1
            return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 1;
        } else {
            // else return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
            return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
        }
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft>2 then ft=2 end
    if ft > 2 {
        ft = 2;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,ft,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, ft as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let e = a.e;
    let mut sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // if #sg>ft then
        if sg.len() as i32 > ft {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
            // local rg=sg:Select(tp,ft,ft,nil)
            let rg = group::select(d, &sg, tp, ft as u16, ft as u16, false, NO_EX).nil();
            // sg=rg
            sg = rg;
        }
        // local tc=sg:GetFirst()
        let _ = group::get_first(d, &sg);
        // for tc in aux.Next(sg) do
        for tc in group::iter(d, &sg) {
            // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
        // Duel.RaiseEvent(sg,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
        duel::raise_event(d, sg, EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(e), 0, tp, 0, 0);
    }
}
