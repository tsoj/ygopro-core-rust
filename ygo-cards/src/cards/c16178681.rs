//! Odd-Eyes Pendulum Dragon
use super::prelude::*;

pub const ID: u32 = 16178681;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // You can reduce the battle damage you take from an attack involving a Pendulum Monster you control to 0
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PRE_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_PRE_BATTLE_DAMAGE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.nodamcon)
    effect::set_condition(d, e1, Some(condition(nodamcon)));
    // e1:SetOperation(s.nodamop)
    effect::set_operation(d, e1, Some(operation(nodamop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy this card, and if you do, add 1 Pendulum Monster with 1500 or less ATK from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(
        d,
        e2,
        Some(lua_fn(|d: &mut Duel, a: Args| duel::is_turn_player(d, a.p(1)))),
    );
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card battles an opponent's monster, any battle damage this card inflicts to your opponent is doubled
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e3:SetCondition(function(e) local bc=e:GetHandler():GetBattleTarget() return bc and bc:IsControler(1-e:GetHandlerPlayer()) end)
    effect::set_condition(
        d,
        e3,
        Some(lua_fn(|d: &mut Duel, a: Args| {
            let h = effect::get_handler(d, a.e(0)).nil();
            let bc = card::get_battle_target(d, h);
            let hp = effect::get_handler_player(d, a.e(0));
            match bc {
                Some(bc) => card::is_controler(d, bc, 1 - hp),
                None => false,
            }
        })),
    );
    // e3:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e3, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.nodamcon(e,tp,eg,ep,ev,re,r,rp)`
fn nodamcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (ep==tp and not Duel.HasFlagEffect(tp,id)) then return false end
    if !(a.ep == a.tp && !duel::has_flag_effect(d, a.tp, ID, None)) {
        return false;
    }
    // local bc1=Duel.GetAttacker()
    let bc1 = duel::get_attacker(d);
    // local bc2=Duel.GetAttackTarget()
    let bc2 = duel::get_attack_target(d);
    let tp = a.tp;
    // return (bc1 and bc1:IsPendulumMonster() and bc1:IsControler(tp)) or (bc2 and bc2:IsPendulumMonster() and bc2:IsControler(tp))
    let b1 = match bc1 {
        Some(x) => card::is_pendulum_monster(d, x) && card::is_controler(d, x, tp),
        None => false,
    };
    if b1 {
        return true;
    }
    match bc2 {
        Some(x) => card::is_pendulum_monster(d, x) && card::is_controler(d, x, tp),
        None => false,
    }
}
/// `s.nodamop(e,tp,eg,ep,ev,re,r,rp)`
fn nodamop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.HasFlagEffect(tp,id) and Duel.SelectEffectYesNo(tp,e:GetHandler(),aux.Stringid(id,1)) then
    if !duel::has_flag_effect(d, tp, ID, None) {
        let h = effect::get_handler(d, a.e).nil();
        if duel::select_effect_yes_no(d, tp, h, Some(aux::stringid(ID, 1))) {
            // Duel.Hint(HINT_CARD,0,id)
            duel::hint(d, HINT_CARD as u8, 0, ID as u64);
            // Duel.ChangeBattleDamage(tp,0)
            duel::change_battle_damage(d, tp, 0, true);
            // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPendulumMonster() and c:IsAttackBelow(1500) and c:IsAbleToHand()
    card::is_pendulum_monster(d, c) && card::is_attack_below(d, c, 1500) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
