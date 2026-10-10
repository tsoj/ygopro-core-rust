//! Succumbing-Song Morganite
use super::prelude::*;

pub const ID: u32 = 81756619;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MORGANITE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects for the rest of the Duel
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
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
    // Add 1 "Morganite" card from your Deck to your hand, then place 1 card from your hand on the bottom of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.chk == Some(0) {
        return !duel::has_flag_effect(d, a.tp, ID, None);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,0,0,1)
    duel::register_flag_effect(d, tp, ID, 0, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot activate monster effects in the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Your monsters can make up to 2 attacks on monsters during each Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK_MONSTER);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // If your monster battles an opponent's monster, any battle damage it inflicts to your opponent is doubled
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.doubledmgtg)
    effect::set_target(d, e3, Some(effect_card_fn(doubledmgtg)));
    // e3:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e3, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let Some(rc) = effect::get_handler(d, re) else {
        return false;
    };
    // return rc and rc:IsLocation(LOCATION_HAND) and re:IsMonsterEffect()
    card::is_location(d, rc, LOCATION_HAND) && effect::is_monster_effect(d, re)
}
/// `s.doubledmgtg(e,c)`
fn doubledmgtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsControler(1-e:GetHandlerPlayer())
    let Some(bc) = card::get_battle_target(d, c) else {
        return false;
    };
    let p = 1 - effect::get_handler_player(d, e);
    card::is_controler(d, bc, p)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MORGANITE) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MORGANITE as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local hg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let hg = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #hg==0 or Duel.SendtoHand(hg,nil,REASON_EFFECT)==0 then return end
    if hg.is_empty() || duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.ConfirmCards(1-tp,hg)
    duel::confirm_cards(d, 1 - tp, hg);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local dg=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil)
    let dg = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoDeck(dg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, dg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
