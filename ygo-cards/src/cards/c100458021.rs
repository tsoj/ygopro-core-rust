//! Raise Moon Starter Squeeze
use super::prelude::*;

pub const ID: u32 = 100458021;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // (Quick Effect): You can discard this card, then activate 1 of these effects (but you can only use each of these effects of "Raise Moon Starter Squeeze" once per turn);
    // ● You cannot Special Summon from the Extra Deck until the end of the next turn, except Rank 7 Xyz Monsters, also this turn, each time a card(s) is added to your opponent's hand, except by drawing it, immediately draw 1 card
    // ● Draw 1 card
    // You cannot add cards from the Deck to the hand during the Duel you activate this effect, except by drawing them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(Cost.AND(Cost.SelfDiscard,s.effcost))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::self_discard(), cost_target(effcost)])));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of cards being added to a player's hand, except by drawing them
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_HAND)
        effect::set_code(d, ge1, EVENT_TO_HAND);
        // ge1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
        effect::set_operation(d, ge1, Some(operation(track_to_hand)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.chk == Some(0) {
        return !duel::has_flag_effect(d, a.tp, ID, None);
    }
    // You cannot add cards from the Deck to the hand during the Duel you activate this effect, except by drawing them
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_TO_HAND)
    effect::set_code(d, e1, EFFECT_CANNOT_TO_HAND);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,tp,re) return c:IsLocation(LOCATION_DECK) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_DECK))));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// Stores the chosen option in the effect's chain data (`e:GetChainData().choice`)
fn choice_value(choice: Option<usize>) -> Value {
    choice.map_or(Value::Nil, |x| Value::Int(x as i64))
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local option_1=not Duel.HasFlagEffect(tp,id+100)
    let option_1 = !duel::has_flag_effect(d, a.tp, ID + 100, None);
    // local option_2=not Duel.HasFlagEffect(tp,id+200) and Duel.IsPlayerCanDraw(tp,1)
    let option_2 = !duel::has_flag_effect(d, a.tp, ID + 200, None) && duel::is_player_can_draw(d, a.tp, 1);
    // if chk==0 then return option_1 or option_2 end
    if a.chk == Some(0) {
        return option_1 || option_2;
    }
    // local choice=Duel.SelectEffect(tp, {option_1,aux.Stringid(id,2)}, {option_2,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, a.tp, &[(option_1, aux::stringid(ID, 2)), (option_2, aux::stringid(ID, 3))]);
    // e:GetChainData().choice=choice
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice_value(choice));
    // if choice==1 then
    if choice == Some(1) {
        // Duel.RegisterFlagEffect(tp,id+100,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 100, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // elseif choice==2 then
    } else if choice == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+200,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 200, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local choice=e:GetChainData().choice
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // if choice==1 then
    if choice == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,4))
        effect::set_description(d, e1, aux::stringid(ID, 4));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not (c:IsRank(7) and c:IsXyzMonster()) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| extra_not_rank7_xyz(d, c))));
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,5))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 5)), 0, None);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_TO_HAND)
        effect::set_code(d, e2, EVENT_TO_HAND);
        // e2:SetCondition(s.drcon)
        effect::set_condition(d, e2, Some(condition(drcon)));
        // e2:SetOperation(s.drop)
        effect::set_operation(d, e2, Some(operation(drop_op)));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    // elseif choice==2 then
    } else if choice == 2 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
/// `c:IsLocation(LOCATION_EXTRA) and not (c:IsRank(7) and c:IsXyzMonster())`
fn extra_not_rank7_xyz(d: &mut Duel, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_EXTRA) && !(card::is_rank(d, c, &[7]) && card::is_xyz_monster(d, c))
}
/// `s.drconfilter(c,opp)`
fn drconfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return c:IsControler(opp) and not c:IsReason(REASON_DRAW)
    card::is_controler(d, c, opp) && !card::is_reason(d, c, REASON_DRAW)
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.drconfilter,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    let opp = 1 - a.tp;
    group::is_exists(d, &eg, &|d, c| drconfilter(d, c, opp), 1, NO_EX)
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
        return;
    }
    // local eff=e:GetLabelObject()
    let eff = effect::get_label_object(d, a.e).as_effect();
    match eff {
        // if eff and not eff:IsDeleted() then eff:SetLabel(eff:GetLabel()+1)
        Some(eff) if !effect::is_deleted(d, eff) => {
            let label = effect::get_label(d, eff);
            effect::set_label(d, eff, &[label + 1]);
        }
        // else
        _ => {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_CHAIN_SOLVED)
            effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
            // e1:SetOperation(s.chainsolvedop)
            effect::set_operation(d, e1, Some(operation(chainsolvedop)));
            // e1:SetLabel(1)
            effect::set_label(d, e1, &[1]);
            // e1:SetLabelObject(e)
            effect::set_label_object(d, e1, Value::Effect(a.e));
            // e1:SetReset(RESET_CHAIN)
            effect::set_reset(d, e1, RESET_CHAIN, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
            // e:SetLabelObject(e1)
            effect::set_label_object(d, a.e, Value::Effect(e1));
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(EVENT_CHAIN_SOLVED)
            effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
            // e2:SetOperation(function() e:SetLabelObject(nil) e1:Reset() end)
            let ea = a.e;
            effect::set_operation(d, e2, Some(operation(move |d, _a| {
                effect::set_label_object(d, ea, Value::Nil);
                effect::reset(d, e1);
            })));
            // e2:SetReset(RESET_CHAIN)
            effect::set_reset(d, e2, RESET_CHAIN, 1);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, a.tp);
        }
    }
}
/// `s.chainsolvedop(e,tp,eg,ep,ev,re,r,rp)`
fn chainsolvedop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    // Duel.Draw(tp,e:GetLabel(),REASON_EFFECT)
    let count = effect::get_label(d, a.e) as u32;
    duel::draw(d, a.tp, count, REASON_EFFECT);
    // e:Reset()
    effect::reset(d, a.e);
    // e:GetLabelObject():SetLabelObject(nil)
    let obj = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label_object(d, obj, Value::Nil);
}
/// The EVENT_TO_HAND tracker of `aux.GlobalCheck` (its flag has no reset)
fn track_to_hand(d: &mut Duel, a: &EffArgs) {
    // for ec in eg:Iter() do
    let eg = a.eg.clone().nil();
    for ec in group::iter(d, &eg) {
        // if ec:IsPreviousLocation(LOCATION_DECK) and not ec:IsReason(REASON_DRAW) then
        if card::is_previous_location(d, ec, LOCATION_DECK) && !card::is_reason(d, ec, REASON_DRAW) {
            // Duel.RegisterFlagEffect(ec:GetReasonPlayer(),id,0,0,1)
            let rp = card::get_reason_player(d, ec);
            duel::register_flag_effect(d, rp, ID, 0, 0, 1, 0);
        }
    }
}
