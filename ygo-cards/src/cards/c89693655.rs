//! Subspace Battle
use super::prelude::*;

pub const ID: u32 = 89693655;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply this effect 3 times, in sequence: ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.revealfilter(c)`
fn revealfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttackAbove(0) and not c:IsPublic()
    card::is_attack_above(d, c, 0) && !card::is_public(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.revealfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&revealfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::get_field_group_count(d, tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_ALL,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_ALL,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 500);
    true
}
/// The Lua `e1:SetValue(function(e,re,tp) ... end)` bodies: `re:IsMonsterEffect() and (rc:HasFlagEffect(flag) or rc:IsCode(codes))`,
/// with the codes kept as the effect label (the Lua upvalue `tp_codes` / `opp_codes`).
fn valfn(d: &mut Duel, a: Args, flag: u32) -> bool {
    // function(e,re,tp) local rc=re:GetHandler()
    let e = a.e(0);
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and (rc:HasFlagEffect(flag) or rc:IsCode(codes)) end
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if card::has_flag_effect(d, rc, flag, None) {
        return true;
    }
    let codes: Vec<u32> = effect::get_labels(d, e).into_iter().map(|x| x as u32).collect();
    card::is_code(d, rc, &codes)
}
/// `e1:SetValue(function(e,re,tp) ... id ... tp_codes end)`
fn value_tp(d: &mut Duel, a: Args) -> bool {
    valfn(d, a, ID)
}
/// `e2:SetValue(function(e,re,tp) ... id+1 ... opp_codes end)`
fn value_opp(d: &mut Duel, a: Args) -> bool {
    valfn(d, a, ID + 1)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not (Duel.IsExistingMatchingCard(s.revealfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.revealfilter,tp,0,LOCATION_DECK,1,nil)) then return end
    if !(duel::is_existing_matching_card(d, Some(&revealfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&revealfilter), tp, 0, LOCATION_DECK, 1, NO_EX))
    {
        return;
    }
    // local opp=1-tp
    let opp = 1 - tp;
    // local tp_codes={}
    // local opp_codes={}
    let mut tp_codes: Vec<i64> = Vec::new();
    let mut opp_codes: Vec<i64> = Vec::new();
    // local break_chk=false
    let mut break_chk = false;
    // for i=1,3 do
    for _i in 1..=3 {
        // if break_chk then Duel.BreakEffect() end
        if break_chk {
            duel::break_effect(d);
        }
        // break_chk=true
        break_chk = true;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // sc1=Duel.SelectMatchingCard(tp,s.revealfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g1 = duel::select_matching_card(d, tp, Some(&revealfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc1 = group::get_first(d, &g1).nil();
        // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_CONFIRM as u64);
        // sc2=Duel.SelectMatchingCard(opp,s.revealfilter,opp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g2 = duel::select_matching_card(d, opp, Some(&revealfilter), opp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc2 = group::get_first(d, &g2).nil();
        // Duel.ConfirmCards(opp,sc1)
        duel::confirm_cards(d, opp, CardSet::from_card(sc1));
        // Duel.ConfirmCards(tp,sc2)
        duel::confirm_cards(d, tp, CardSet::from_card(sc2));
        // table.insert(tp_codes,sc1:GetCode())
        let code1 = card::get_code(d, sc1)[0];
        tp_codes.push(code1 as i64);
        // table.insert(opp_codes,sc2:GetCode())
        let code2 = card::get_code(d, sc2)[0];
        opp_codes.push(code2 as i64);
        // sc1:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, sc1, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // sc2:RegisterFlagEffect(id+1,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, sc2, ID + 1, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // atk1=sc1:GetAttack()
        let atk1 = card::get_attack(d, sc1);
        // atk2=sc2:GetAttack()
        let atk2 = card::get_attack(d, sc2);
        let mut search_card: Option<CardId> = None;
        let mut destroy_card: Option<CardId> = None;
        // if atk1>atk2 then
        if atk1 > atk2 {
            // search_card=sc1
            search_card = Some(sc1);
            // destroy_card=sc2
            destroy_card = Some(sc2);
        // elseif atk2>atk1 then
        } else if atk2 > atk1 {
            // search_card=sc2
            search_card = Some(sc2);
            // destroy_card=sc1
            destroy_card = Some(sc1);
        // else
        } else {
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
            // Duel.ShuffleDeck(opp)
            duel::shuffle_deck(d, opp);
        }
        // if search_card and destroy_card then
        if let (Some(sc), Some(dc)) = (search_card, destroy_card) {
            // if Duel.SendtoHand(search_card,nil,REASON_EFFECT)>0 then
            if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0 {
                // local prev_ctrl=search_card:GetPreviousControler()
                let prev_ctrl = card::get_previous_controler(d, sc);
                // Duel.ConfirmCards(1-prev_ctrl,search_card)
                duel::confirm_cards(d, 1 - prev_ctrl, CardSet::from_card(sc));
                // Duel.ShuffleHand(prev_ctrl)
                duel::shuffle_hand(d, prev_ctrl);
            } else {
                // Duel.SendtoGrave(search_card,REASON_RULE)
                duel::send_to_grave(d, CardSet::from_card(sc), REASON_RULE, None, None);
            }
            // if Duel.Destroy(destroy_card,REASON_EFFECT)>0 then
            if duel::destroy(d, CardSet::from_card(dc), REASON_EFFECT, None, None) > 0 {
                // Duel.Damage(destroy_card:GetPreviousControler(),500,REASON_EFFECT)
                let p = card::get_previous_controler(d, dc);
                duel::damage(d, p, 500, REASON_EFFECT, false, None);
            }
        }
        // if not (Duel.IsExistingMatchingCard(s.revealfilter,tp,LOCATION_DECK,0,1,nil)
        //     and Duel.IsExistingMatchingCard(s.revealfilter,tp,0,LOCATION_DECK,1,nil)) then break end
        if !(duel::is_existing_matching_card(d, Some(&revealfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&revealfilter), tp, 0, LOCATION_DECK, 1, NO_EX))
        {
            break;
        }
    }
    // Each player cannot activate the effects of the monster they revealed and monsters with the same name as it for the rest of this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(e,re,tp) ... tp_codes ... end)
    effect::set_value_fn(d, e1, lua_fn(value_tp));
    effect::set_label(d, e1, &tp_codes);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetValue(function(e,re,tp) ... opp_codes ... end)
    effect::set_value_fn(d, e2, lua_fn(value_opp));
    effect::set_label(d, e2, &opp_codes);
    // Duel.RegisterEffect(e2,opp)
    duel::register_effect(d, e2, opp);
}
