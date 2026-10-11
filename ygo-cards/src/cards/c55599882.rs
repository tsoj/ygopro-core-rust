//! Bujintervention
use super::prelude::*;

pub const ID: u32 = 55599882;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BUJIN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BUJIN)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BUJIN as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if chk==0 then return bc and bc:IsLocation(LOCATION_GRAVE) and bc:IsReason(REASON_BATTLE) and bc:IsMonster()
    //     and c:GetBaseAttack()<=bc:GetAttack() and bc:IsAbleToRemove() end
    if a.chk == Some(0) {
        return match bc {
            None => false,
            Some(bc) => {
                card::is_location(d, bc, LOCATION_GRAVE)
                    && card::is_reason(d, bc, REASON_BATTLE)
                    && card::is_monster(d, bc)
                    && card::get_base_attack(d, c) <= card::get_attack(d, bc)
                    && card::is_able_to_remove(d, bc, None, None, None)
            }
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.rmfilter(c,code)`
fn rmfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsMonster() and c:IsAbleToRemove()
    card::is_code(d, c, &[code]) && card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h).nil();
    // if Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)==0 then return end
    if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE,nil,tc:GetCode())
    let code = card::get_code(d, tc)[0];
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| rmfilter(d, c, code)),
        tp,
        0,
        LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE,
        NO_EX,
    );
    // if #g>0 then
    if !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsImmuneToEffect(e) then
    let Some(tc) = tc else {
        // tc:IsRelateToEffect(e) on nil is a Lua error
        crate::lua_error("attempt to index a nil value");
    };
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_REMOVE)
        effect::set_category(d, e1, CATEGORY_REMOVE);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
        // e1:SetTarget(s.rmtg)
        effect::set_target(d, e1, Some(cost_target(rmtg)));
        // e1:SetOperation(s.rmop)
        effect::set_operation(d, e1, Some(operation(rmop)));
        // e1:SetReset(RESET_EVENT|RESET_TURN_SET|RESET_TOHAND|RESET_TODECK|RESET_TOFIELD|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | RESET_TURN_SET | RESET_TOHAND | RESET_TODECK | RESET_TOFIELD | RESET_PHASE | PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
