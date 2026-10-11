//! Against the Wind
//! アゲインスト・ウィンド
use super::prelude::*;

pub const ID: u32 = 64952266;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_DAMAGE);
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
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BLACKWING) and c:IsMonster() and c:GetAttack()>0 and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::is_monster(d, c) && card::get_attack(d, c) > 0 && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Damage(tp,tc:GetAttack(),REASON_EFFECT)~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let atk = card::get_attack(d, tc);
            if duel::damage(d, a.tp, atk as i64, REASON_EFFECT, false, None) != 0 {
                // Duel.SendtoHand(tc,nil,REASON_EFFECT)
                duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,tc)
                duel::confirm_cards(d, 1 - a.tp, tc);
            }
        }
    }
}
