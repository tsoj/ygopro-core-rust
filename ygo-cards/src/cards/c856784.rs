//! Draw of Fate
use super::prelude::*;

pub const ID: u32 = 856784;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local mg=g and g:GetMaxGroup(Card.GetAttack)
    let mg = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|(mg, _)| mg);
    // return Duel.GetLP(tp)<Duel.GetLP(1-tp) and mg and mg:IsExists(Card.IsControler,1,nil,1-tp)
    if duel::get_lp(d, a.tp) >= duel::get_lp(d, 1 - a.tp) {
        return false;
    }
    match mg {
        Some(mg) => group::is_exists(d, &mg, &|d, c| card::is_controler(d, c, 1 - a.tp), 1, NO_EX),
        None => false,
    }
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(aux.NOT(Card.IsPublic),tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| !card::is_public(d, c)), a.tp, LOCATION_DECK, 0, NO_EX);
        // return Duel.IsPlayerCanDraw(tp,1) and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>2
        //     and g:GetClassCount(Card.GetCode)>=3
        return duel::is_player_can_draw(d, a.tp, 1)
            && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 2
            && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dn = info[1].int() as u32;
    // local g=Duel.GetMatchingGroup(aux.NOT(Card.IsPublic),tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| !card::is_public(d, c)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)>=3 then
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3 {
        // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_CONFIRM)
        let dncheck = aux::dncheck();
        let mut sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&dncheck), 1, a.tp, Some(HINTMSG_CONFIRM as u64), None, None, false)
            .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg.clone());
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // while (#sg>0) do
        while !sg.is_empty() {
            // dg=sg:RandomSelect(tp,1)
            // (the Lua `dg` is a global; nothing else reads it, so it is a local here)
            let dg = group::random_select(d, &sg, a.tp, 1);
            // sg:Sub(dg)
            group::remove_card(&mut sg, &dg);
            // Duel.MoveSequence(dg:GetFirst(),0)
            let first = group::get_first(d, &dg).nil();
            duel::move_sequence(d, first, 0, None);
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, p, dn, REASON_EFFECT);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e1, EFFECT_CANNOT_SSET);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetOperation(s.aclimit1)
    effect::set_operation(d, e2, Some(operation(aclimit1)));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, h);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e3, EVENT_CHAIN_NEGATED);
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // e3:SetOperation(s.aclimit2)
    effect::set_operation(d, e3, Some(operation(aclimit2)));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // local e4=Effect.CreateEffect(e:GetHandler())
    let e4 = effect::create_effect(d, h);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
    // e4:SetCondition(s.econ)
    effect::set_condition(d, e4, Some(condition(econ)));
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, a.tp);
    // local e5=Effect.CreateEffect(e:GetHandler())
    let e5 = effect::create_effect(d, h);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e5, RESET_PHASE | PHASE_END, 1);
    // e5:SetTargetRange(1,0)
    effect::set_target_range(d, e5, 1, 0);
    // Duel.RegisterEffect(e5,tp)
    duel::register_effect(d, e5, a.tp);
}
/// `s.aclimit1(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit1(d: &mut Duel, a: &EffArgs) {
    // if ep~=tp then return end
    if a.ep != a.tp {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `s.aclimit2(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit2(d: &mut Duel, a: &EffArgs) {
    // if ep~=tp then return end
    if a.ep != a.tp {
        return;
    }
    // Duel.ResetFlagEffect(tp,id)
    duel::reset_flag_effect(d, a.tp, ID);
}
/// `s.econ(e)`
fn econ(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)~=0
    let p = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, p, ID) != 0
}
