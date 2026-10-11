//! Salamangreat Raccoon
use super::prelude::*;

pub const ID: u32 = 53490455;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain LP equal to opponent's attacking monster's ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.lpcon)
    effect::set_condition(d, e1, Some(condition(lpcon)));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.lptg)
    effect::set_target(d, e1, Some(cost_target(lptg)));
    // e1:SetOperation(s.lpop)
    effect::set_operation(d, e1, Some(operation(lpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add this card from GY to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lpcon(e,tp,eg,ep,ev,re,r,rp)`
fn lpcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    // return d and d:IsControler(tp) and d:IsFaceup() and d:IsSetCard(SET_SALAMANGREAT)
    match duel::get_attack_target(d) {
        Some(t) => card::is_controler(d, t, a.tp) && card::is_faceup(d, t) && card::is_set_card(d, t, &[SET_SALAMANGREAT as u16]),
        None => false,
    }
}
/// `s.lpfilter(c,e)`
fn lpfilter(d: &mut Duel, c: Option<CardId>, e: EffectId) -> bool {
    // return c and c:IsOnField() and c:IsFaceup() and c:IsCanBeEffectTarget(e)
    match c {
        Some(c) => card::is_on_field(d, c) && card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e)),
        None => false,
    }
}
/// `s.lptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local tg=Duel.GetAttackTarget()
    let tg = duel::get_attack_target(d);
    // if chkc then return chkc==at or chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == at || Some(chkc) == tg;
    }
    // if chk==0 then return s.lpfilter(at,e) and s.lpfilter(tg,e) end
    if a.chk == Some(0) {
        return lpfilter(d, at, e) && lpfilter(d, tg, e);
    }
    // local g=Group.FromCards(at,tg)
    let mut g = CardSet::new();
    if let Some(x) = at {
        group::add_card(&mut g, &CardSet::from_card(x));
    }
    if let Some(x) = tg {
        group::add_card(&mut g, &CardSet::from_card(x));
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // local rec=at:GetAttack()
    let rec = card::get_attack(d, at.nil());
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, rec as i32);
    true
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if at and g:IsContains(at) and at:IsFaceup() then
    if let Some(at) = at {
        if group::is_contains(&g, at) && card::is_faceup(d, at) {
            // Duel.Recover(tp,at:GetAttack(),REASON_EFFECT)
            let amount = card::get_attack(d, at);
            duel::recover(d, tp, amount as i64, REASON_EFFECT, false, None);
        }
    }
    // local tg=g:Filter(Card.IsControler,nil,tp):GetFirst()
    let filtered = group::filter(d, &g, &|d, c| card::is_controler(d, c, tp), NO_EX);
    let tg = group::get_first(d, &filtered);
    // if tg and tg:IsFaceup() then
    if let Some(tg) = tg {
        if card::is_faceup(d, tg) {
            // Cannot be destroyed by battle
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3000)
            effect::set_description(d, e1, 3000);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tg:RegisterEffect(e1)
            card::register_effect(d, tg, e1, false);
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc);
    // return #eg==1
    //     and tc:IsLocation(LOCATION_GRAVE) and tc:IsReason(REASON_BATTLE)
    //     and bc:IsRelateToBattle() and bc:IsControler(tp) and bc:IsSetCard(SET_SALAMANGREAT)
    if eg.len() != 1 || !card::is_location(d, tc, LOCATION_GRAVE) || !card::is_reason(d, tc, REASON_BATTLE) {
        return false;
    }
    let bc = bc.nil();
    card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, tp) && card::is_set_card(d, bc, &[SET_SALAMANGREAT as u16])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - a.tp, c);
    }
}
