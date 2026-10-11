//! Clown Crew Dristy
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 30109445;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 Ritual, Fusion, Synchro, Xyz, and/or Pendulum Monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_RITUAL|TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_PENDULUM),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| {
            card::is_type_ctx(d, c, TYPE_RITUAL | TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_PENDULUM, sc, st, Some(tp))
        })),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // Your Tribute Summoned monsters can make a second attack during each Battle Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsTributeSummoned))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_tribute_summoned(d, c))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Tributed: You can activate 1 of these effects (but you can only use each of these effects of "Clown Crew Drish" once per turn);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsLinkMonster,Card.IsFaceup,Card.IsAbleToExtra),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_link_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    // local b2=not Duel.HasFlagEffect(tp,id+1) and ((Duel.IsPlayerCanDraw(tp)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,nil))
    //     or (Duel.IsPlayerCanDraw(1-tp) and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_HAND,1,nil)))
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && ((duel::is_player_can_draw(d, a.tp, 0)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX))
            || (duel::is_player_can_draw(d, 1 - a.tp, 0)
                && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_HAND, 1, NO_EX)));
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TOEXTRA)
        effect::set_category(d, a.e, CATEGORY_TOEXTRA);
        // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,PLAYER_ALL,LOCATION_MZONE|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, PLAYER_ALL as u8, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TODECK|CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_TODECK | CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_ALL,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_ALL as u8, LOCATION_HAND as i32);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --● Return all Link Monsters on the field and in the GYs to the Extra Deck
        // local g=Duel.GetMatchingGroup(aux.AND(Card.IsLinkMonster,Card.IsFaceup,Card.IsAbleToExtra),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_link_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            NO_EX,
        );
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // --● Each player shuffles their entire hand into the Deck, then they draw the same number of cards they shuffled
        // local turn_player=Duel.GetTurnPlayer()
        let turn_player = duel::get_turn_player(d);
        // local step=turn_player==0 and 1 or -1
        // for p=turn_player,1-turn_player,step do
        // (the loop runs over the two players: turn player first)
        let players: [u8; 2] = if turn_player == 0 { [0, 1] } else { [1, 0] };
        for p in players {
            // local g=Duel.GetFieldGroup(p,LOCATION_HAND,0)
            let g = duel::get_field_group(d, p, LOCATION_HAND, 0);
            // if #g>0 and Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and Duel.IsPlayerCanDraw(p) then
            if !g.is_empty() && duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 && duel::is_player_can_draw(d, p, 0) {
                // local draw_count=Duel.GetOperatedGroup():FilterCount(Card.IsPreviousControler,nil,p)
                let operated = duel::get_operated_group(d);
                let draw_count = group::filter_count(d, &operated, &|d, c| card::is_previous_controler(d, c, p), NO_EX);
                // if draw_count>0 then
                if draw_count > 0 {
                    // Duel.ShuffleDeck(p)
                    duel::shuffle_deck(d, p);
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.Draw(p,draw_count,REASON_EFFECT)
                    duel::draw(d, p, draw_count as u32, REASON_EFFECT);
                }
            }
        }
    }
}
