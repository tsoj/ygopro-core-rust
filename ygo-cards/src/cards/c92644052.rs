//! Performapal Duelist Extraordinaire
//! ＥＭ稀代の決闘者
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 92644052;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[92428405],
    listed_series: &[SET_SUPREME_KING_DRAGON, SET_SUPREME_KING_GATE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Return this card to the hand, and if you do, banish 1 Spell from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function() return Duel.GetAttacker() and Duel.GetAttackTarget() end)
    effect::set_condition(d, e1, Some(condition(battle_cond)));
    // e1:SetTarget(s.selfthtg)
    effect::set_target(d, e1, Some(cost_target(selfthtg)));
    // e1:SetOperation(s.selfthop)
    effect::set_operation(d, e1, Some(operation(selfthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Supreme King Dragon" monster, "Supreme King Gate" monster, or "Soul of the Supreme King" from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end)
    effect::set_condition(d, e2, Some(condition(opp_damage_cond)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Each player can banish 1 Spell from their Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_STANDBY);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // The attacked player can apply the following effect: ● Add 1 of your banished Spells to your hand, then discard it, and if you do, negate that attack.
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e4, aux::stringid(ID, 3));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_HANDES)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_HANDES);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.negtg)
    effect::set_target(d, e4, Some(cost_target(negtg)));
    // e4:SetOperation(s.negop)
    effect::set_operation(d, e4, Some(operation(negop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function() return Duel.GetAttacker() and Duel.GetAttackTarget() end`
fn battle_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_attacker(d).is_some() && duel::get_attack_target(d).is_some()
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end`
fn opp_damage_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsAbleToRemove()
    card::is_spell(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.selfthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None) && duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.selfthop(e,tp,eg,ep,ev,re,r,rp)`
fn selfthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0 and c:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&rmfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
    // local bg=Group.FromCards(Duel.GetAttacker(),Duel.GetAttackTarget()):Match(function(c) return c:IsRelateToBattle() and c:IsControler(tp) end,nil)
    let mut bg = CardSet::new();
    if let Some(att) = duel::get_attacker(d) {
        group::add_card(&mut bg, &CardSet::from_card(att));
    }
    if let Some(at) = duel::get_attack_target(d) {
        group::add_card(&mut bg, &CardSet::from_card(at));
    }
    group::match_(d, &mut bg, &|d, x| card::is_relate_to_battle(d, x) && card::is_controler(d, x, tp), NO_EX);
    // if #bg==0 then return end
    if bg.is_empty() {
        return;
    }
    // for bc in bg:Iter() do
    for bc in group::iter(d, &bg) {
        // For that battle, your monster cannot be destroyed
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
    // For that battle, the battle damage you take is halved
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(HALF_DAMAGE)
    effect::set_value(d, e2, HALF_DAMAGE as i64);
    // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard({SET_SUPREME_KING_DRAGON,SET_SUPREME_KING_GATE}) and c:IsMonster()) or c:IsCode(92428405)) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16, SET_SUPREME_KING_GATE as u16]) && card::is_monster(d, c)) || card::is_code(d, c, &[92428405]))
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
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
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_ALL,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, _a: &EffArgs) {
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // local step=turn_p==0 and 1 or -1
    // for p=turn_p,1-turn_p,step do
    let order: [u8; 2] = if turn_p == 0 { [0, 1] } else { [1, 0] };
    for p in order {
        // local g=Duel.GetMatchingGroup(s.rmfilter,p,LOCATION_DECK,0,1,nil)
        let g = duel::get_matching_group(d, Some(&rmfilter), p, LOCATION_DECK, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(p,aux.Stringid(id,4)) then
        if !g.is_empty() && duel::select_yes_no(d, p, aux::stringid(ID, 4)) {
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_REMOVE as u64);
            // local rg=g:Select(p,1,1,nil)
            let rg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
            // if #rg>0 then
            if !rg.is_empty() {
                // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
                duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `local attacked_player=bc and bc:GetControler() or (1-Duel.GetAttacker():GetControler())`
fn attacked_player(d: &mut Duel) -> u8 {
    match duel::get_attack_target(d) {
        Some(bc) => card::get_controler(d, bc),
        None => 1 - card::get_controler(d, duel::get_attacker(d).nil()),
    }
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=Duel.GetAttackTarget()
    // local attacked_player=...
    let ap = attacked_player(d);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,attacked_player,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, ap, LOCATION_REMOVED as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,attacked_player,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, ap, 1);
    true
}
/// `s.rmthfilter(c)`
fn rmthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsFaceup() and c:IsAbleToHand()
    card::is_spell(d, c) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, _a: &EffArgs) {
    // local attacked_player=...
    let ap = attacked_player(d);
    // local g=Duel.GetMatchingGroup(s.rmthfilter,attacked_player,LOCATION_REMOVED,0,1,nil)
    let g = duel::get_matching_group(d, Some(&rmthfilter), ap, LOCATION_REMOVED, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(attacked_player,aux.Stringid(id,5)) then
    if !g.is_empty() && duel::select_yes_no(d, ap, aux::stringid(ID, 5)) {
        // Duel.Hint(HINT_SELECTMSG,attacked_player,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, ap, HINTMSG_ATOHAND as u64);
        // local sc=g:Select(attacked_player,1,1,nil):GetFirst()
        let sel = group::select(d, &g, ap, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if not sc then return end
        let Some(sc) = sc else {
            return;
        };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // if Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND)
        //     and sc:IsDiscardable(REASON_EFFECT) then
        if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0
            && card::is_location(d, sc, LOCATION_HAND)
            && card::is_discardable(d, sc, Some(REASON_EFFECT), None)
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SendtoGrave(sc,REASON_EFFECT|REASON_DISCARD)>0 then
            if duel::send_to_grave(d, sc, REASON_EFFECT | REASON_DISCARD, None, None) > 0 {
                // Duel.NegateAttack()
                duel::negate_attack(d);
            }
        }
    }
}
