//! Head Judging
use super::prelude::*;

pub const ID: u32 = 38143903;
pub const INFO: ScriptInfo = ScriptInfo {
    toss_coin: true,
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
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Toss a coin and apply the appropriate effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COIN+CATEGORY_TOGRAVE+CATEGORY_NEGATE+CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_COIN + CATEGORY_TOGRAVE + CATEGORY_NEGATE + CATEGORY_CONTROL);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(cost_target(tg)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and Duel.IsChainNegatable(ev) and re:GetActivateLocation()&LOCATION_ONFIELD>0
    let re = a.re.nil();
    effect::is_monster_effect(d, re)
        && duel::is_chain_negatable(d, a.ev as u8)
        && (effect::get_activate_location(d, re) & LOCATION_ONFIELD) > 0
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    // if chk==0 then return c:IsAbleToGrave()
    //   and (not re:GetHandler():IsRelateToEffect(re) or re:GetHandler():IsAbleToChangeControler()) end
    if a.chk == Some(0) {
        return card::is_able_to_grave(d, c, None)
            && (!card::is_relate_to_effect(d, rh, re) || card::is_able_to_change_controler(d, rh));
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,rp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.rp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,re:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, Some(CardSet::from_card(rh)), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) then
    if card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,re:GetHandler(),1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(rh)), 1, 0, 0);
    }
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local p=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CONTROLER)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CONTROLER]);
    let p = info[0].int() as u8;
    // if Duel.CallCoin(p) then
    if duel::call_coin(d, p) {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    } else {
        // elseif Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rh, re) {
            // Duel.GetControl(re:GetHandler(),1-p)
            duel::get_control(d, rh, 1 - p, 0, 0, None, None);
        }
    }
}
