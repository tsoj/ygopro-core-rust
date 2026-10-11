//! Royal Rhino with Deceitful Dice
use super::prelude::*;

pub const ID: u32 = 74289646;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Roll a six-sided die
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_ATKCHANGE+CATEGORY_DAMAGE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_ATKCHANGE + CATEGORY_DAMAGE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e) return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        })),
    );
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 1, tp, 0);
    // local chain_link=ev+1
    let chain_link = a.ev + 1;
    // if chain_link==2 then
    if chain_link == 2 {
        // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,500)
        let h = effect::get_handler(d, a.e).nil();
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, tp, 500);
    } else if chain_link == 3 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,500)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - tp, 500);
    } else if chain_link >= 4 {
        // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
        let g = duel::get_field_group(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,PLAYER_ALL,LOCATION_ONFIELD)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local res=Duel.TossDice(tp,1)
    let res = duel::toss_dice(d, tp, 1, 0)[0] as u32;
    // local chain_link=Duel.GetCurrentChain()
    let chain_link = duel::get_current_chain(d, false);
    // if chain_link==2 then
    if chain_link == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not (c:IsFaceup() and c:IsRelateToEffect(e)) then return end
        if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e)) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(res*500)
        effect::set_value(d, e1, (res * 500) as i64);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else if chain_link == 3 {
        // Duel.Damage(1-tp,res*500,REASON_EFFECT)
        duel::damage(d, 1 - tp, (res * 500) as i64, REASON_EFFECT, false, None);
    } else if chain_link >= 4 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,res,nil)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, res as u16, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
